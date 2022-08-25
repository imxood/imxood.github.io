use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{bounded, Receiver, RecvTimeoutError};
use serialport::SerialPort;

use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct Step {
    /// step 的名称
    name: String,
    cmd: String,
    /// 超时
    timeout: Duration,
    /// 成功条件
    ok: String,
    /// 失败条件
    fail: String,
    /// 只读, 不会调用 step.write 函数
    /// 如果是 at device, 则 不会发送写请求
    /// 如果是 exe device, 则 不会创建进程, 直接结束
    readonly: bool,
    /// 结束标志, 根据 step 输出, 匹配到 结束标志 就会退出
    has_endflag: bool,
    /// 捕获输入 到 step
    /// 对于 at device, 将类似 shell.
    has_capture: bool,
}

impl Step {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            cmd: String::new(),
            timeout: Duration::MAX,
            ok: "OK".into(),
            fail: "FAIL".into(),
            readonly: false,
            has_endflag: true,
            has_capture: false,
        }
    }

    pub fn run(&mut self, dev: &mut impl RunDevice) -> Result<()> {
        dev.open()?;
        dev.set_timeout(self.timeout)?;

        if !self.readonly {
            println!("-- run cmd: {}", dev.cmd());
            dev.write(self.cmd.as_bytes())?;
        }
        let mut success = false;
        let mut fail = false;

        loop {
            if dev.read()? == 0 {
                break;
            }

            if self.has_endflag {
                // 执行失败
                if dev.check_end_flag(&self.fail) {
                    fail = true;
                    break;
                }

                // 执行成功
                if dev.check_end_flag(&self.ok) {
                    success = true;
                    break;
                }
            }
        }

        println!("\n-- raw_str: {:?}", dev.out_buf());

        if success {
            println!("-- {} run success", &self.name);
            return Ok(());
        }

        if fail {
            return Err(Error::Error("异常条件已满足, 退出执行".into()));
        }

        Err(Error::Error(
            "结束运行, 成功或失败 条件不满足, 异常!".into(),
        ))
    }

    pub fn with_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.cmd = cmd.into();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_ok(mut self, ok: impl Into<String>) -> Self {
        self.ok = ok.into();
        self
    }

    pub fn with_fail(mut self, fail: impl Into<String>) -> Self {
        self.fail = fail.into();
        self
    }

    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn with_endflag(mut self, endflag: bool) -> Self {
        self.has_endflag = endflag;
        self
    }
}

pub trait RunDevice {
    fn set_timeout(&mut self, _timeout: Duration) -> Result<()> {
        Ok(())
    }

    fn open(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(&mut self) {}

    fn dev_name(&self) -> &str;

    fn cmd(&self) -> String;

    /// 每次执行前需要 清除 out_buf
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        Ok(0)
    }

    fn read(&mut self) -> Result<usize> {
        Ok(0)
    }

    fn out_buf(&self) -> &str;

    fn check_end_flag(&self, end_flag: impl AsRef<str>) -> bool {
        let end_flag = end_flag.as_ref();
        let out_buf = self.out_buf();
        if !end_flag.is_empty() && out_buf.find(end_flag).is_some() {
            true
        } else {
            false
        }
    }
}

pub struct ExeDevice {
    command: Command,
    child: Option<ChildProcess>,
    dev_name: String,
}

pub struct ChildProcess {
    process: Child,
    pub reader: Receiver<String>,
}

impl Drop for ChildProcess {
    fn drop(&mut self) {
        let id = self.process.id();
        println!("wait process {} exit ...", id);
        self.process.kill().ok();
        self.process.wait().ok();
        println!("process {} exited", id);
    }
}

impl ChildProcess {
    pub fn new(cmd: &str) -> Result<Self> {
        let mut arr = "a  b c".split_whitespace();
        let cmd = arr.next().unwrap();

        let mut command = Command::new(cmd);

        while let Some(arg) = arr.next() {
            command.arg(arg);
        }

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let process = command.spawn()?;

        let child_stdout = process.stdout.take().expect("child.stdout is None 异常");
        let child_stderr = process.stderr.take().expect("child.stderr is None 异常");

        let (tx0, rx0) = bounded::<String>(32);
        let tx1 = tx0.clone();

        println!("run process {}", process.id());

        thread::spawn(move || {
            let mut stdout = BufReader::new(child_stdout);
            let mut buf = vec![0u8; 512];
            while let Ok(len) = stdout.read(&mut buf) {
                let data = &buf[..len];
                tx0.send(
                    std::str::from_utf8(data)
                        .expect("从 child_stdout 中获取的数据无法转换到str")
                        .to_string(),
                )
                .ok();
            }
        });

        thread::spawn(move || {
            let mut stderr = BufReader::new(child_stderr);
            let mut buf = vec![0u8; 512];
            while let Ok(len) = stderr.read(&mut buf) {
                let data = &buf[..len];
                tx1.send(
                    std::str::from_utf8(data)
                        .expect("从 child_stderr 中获取的数据无法转换到str")
                        .to_string(),
                )
                .ok();
            }
        });

        Ok(Self {
            process,
            reader: rx0,
        })
    }
}

impl ExeDevice {
    pub fn new(cmd: &str, params: &[&str]) -> Result<Self> {
        let mut command = Command::new(cmd);

        for param in params {
            command.arg(param);
        }

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        Ok(Self {
            command,
            child: None,
            dev_name: String::new(),
        })
    }
}

impl RunDevice for ExeDevice {
    fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
        self.timeout = timeout;
        Ok(())
    }

    /// 创建 并执行 新进程
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        let mut child = self.command.spawn()?;

        let child_stdout = child.stdout.take().expect("child.stdout is None 异常");
        let child_stderr = child.stderr.take().expect("child.stderr is None 异常");

        let (tx0, rx0) = bounded::<String>(32);
        let tx1 = tx0.clone();

        println!("run process {}", child.id());

        self.reader = Some(rx0);
        self.child = Some(child);

        thread::spawn(move || {
            let mut stdout = BufReader::new(child_stdout);
            let mut buf = vec![0u8; 512];
            while let Ok(len) = stdout.read(&mut buf) {
                let data = &buf[..len];
                tx0.send(
                    std::str::from_utf8(data)
                        .expect("从 child_stdout 中获取的数据无法转换到str")
                        .to_string(),
                )
                .ok();
            }
        });

        thread::spawn(move || {
            let mut stderr = BufReader::new(child_stderr);
            let mut buf = vec![0u8; 512];
            while let Ok(len) = stderr.read(&mut buf) {
                let data = &buf[..len];
                tx1.send(
                    std::str::from_utf8(data)
                        .expect("从 child_stderr 中获取的数据无法转换到str")
                        .to_string(),
                )
                .ok();
            }
        });

        Ok(0)
    }

    fn read(&mut self) -> Result<usize> {
        let mut len = 0;
        if let Some(render) = &self.reader {
            match render.recv_timeout(self.timeout) {
                Ok(data) => {
                    len = data.len();
                    println!("{}", &data);
                    self.out_buf.push_str(&data);
                }
                Err(RecvTimeoutError::Timeout) => {
                    return Err(Error::Timeout);
                }
                Err(RecvTimeoutError::Disconnected) => {}
            }
        }
        Ok(len)
    }

    fn dev_name(&self) -> &str {
        &self.dev_name
    }

    fn cmd(&self) -> String {
        format!("{:?}", &self.command)
    }

    fn out_buf(&self) -> &str {
        &self.out_buf
    }

    fn check_end_flag(&self, end_flag: impl AsRef<str>) -> bool {
        let end_flag = end_flag.as_ref();
        let out_buf = self.out_buf();
        if !end_flag.is_empty() && out_buf.find(end_flag).is_some() {
            true
        } else {
            false
        }
    }
}

impl Drop for ExeDevice {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            println!("wait process {} exit ...", child.id());
            child.kill().ok();
            child.wait().ok();
            println!("process {} exited", child.id());
        }
    }
}

pub struct AtDevice {
    serial: Option<Box<dyn SerialPort>>,
    out_buf: String,
    port_name: String,
    baudrate: u32,
    timeout: Duration,
    dev_name: String,
}

impl AtDevice {
    pub fn new(
        port_name: impl Into<String>,
        baudrate: u32,
        dev_name: impl Into<String>,
    ) -> Result<Self> {
        let port_name: String = port_name.into();
        Ok(Self {
            port_name,
            baudrate,
            serial: None,
            timeout: Duration::from_secs(5),
            out_buf: String::new(),
            dev_name: dev_name.into(),
        })
    }
}

impl RunDevice for AtDevice {
    fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
        self.timeout = timeout;
        if let Some(serial) = &mut self.serial {
            serial
                .set_timeout(timeout)
                .map_err(|e| Error::Error(e.to_string()))?;
        }
        Ok(())
    }

    fn open(&mut self) -> Result<()> {
        if self.serial.is_none() {
            let serial = match serialport::new(self.port_name.clone(), self.baudrate).open() {
                Ok(s) => s,
                Err(e) => return Err(Error::Error(e.to_string())),
            };
            self.serial = Some(serial);
        }
        Ok(())
    }

    fn close(&mut self) {
        self.serial = None;
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.out_buf.clear();

        if let Some(serial) = &mut self.serial {
            if let Err(e) = serial.set_timeout(self.timeout) {
                return Err(Error::Error(e.to_string()));
            }
            return Ok(serial.write(buf)?);
        }

        Err(Error::Error(format!("串口<{}>未打开", self.port_name)))
    }

    fn read(&mut self) -> Result<usize> {
        if let Some(serial) = &mut self.serial {
            let mut tmp = vec![0u8; 1];
            let len = serial.read(&mut tmp)?;

            if len != 0 {
                let ch = tmp[0] as char;
                print!("{}", ch);
                std::io::stdout().flush()?;
                self.out_buf.push(ch);
            }

            return Ok(len);
        }
        Err(Error::Error(format!("串口<{}>未打开", self.port_name)))
    }

    fn dev_name(&self) -> &str {
        &self.dev_name
    }

    fn cmd(&self) -> String {
        format!("{:?}", &self.)
    }

    fn out_buf(&self) -> &str {
        &self.out_buf
    }
}
