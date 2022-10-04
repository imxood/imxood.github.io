use core::iter::once;
use lazy_static::lazy_static;
use std::{ffi::c_void, mem::size_of, os::windows::prelude::OsStrExt, time::Duration};
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{LibraryLoader::*, Power::*, SystemServices::*},
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

lazy_static! {
    static ref THREAD_EVENT_TARGET_WINDOW_CLASS: Vec<u16> = unsafe {
        let class_name= encode_wide("windows api demo");

        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: Default::default(),
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(PCWSTR::null()).unwrap_or_default(),
            hIcon: HICON::default(),
            hCursor: HCURSOR::default(), // must be null in order for cursor state to work properly
            hbrBackground: HBRUSH::default(),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
            hIconSm: HICON::default(),
        };

        // 这个 Class 必须注册
        RegisterClassExW(&class);
        class_name
    };
}

fn main() {
    std::thread::spawn(move || {
        // 创建创建窗口 需要和事件处理 放在同一个线程中.

        // 创建 窗口句柄
        let window = unsafe {
            CreateWindowExW(
                WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_TOOLWINDOW,
                PCWSTR::from_raw(THREAD_EVENT_TARGET_WINDOW_CLASS.clone().as_ptr()),
                PCWSTR::null(),
                WS_OVERLAPPED,
                0,
                0,
                0,
                0,
                HWND::default(),
                HMENU::default(),
                GetModuleHandleW(PCWSTR::null()).unwrap_or_default(),
                core::ptr::null_mut(),
            )
        };

        // // 设置子窗口处理程序
        // if unsafe { SetWindowSubclass(window, Some(window_proc), 0, 0).as_bool() } {
        //     println!("设置窗口子类成功");
        // } else {
        //     println!("设置窗口子类失败");
        // }

        unsafe {
            SetWindowLongPtrW(window, GWL_STYLE, (WS_VISIBLE | WS_POPUP).0 as isize);
        }

        // 注册特定事件
        let mut notify_filter = DEV_BROADCAST_DEVICEINTERFACE_W::default();
        notify_filter.dbcc_size = size_of::<DEV_BROADCAST_DEVICEINTERFACE_W>() as u32;
        notify_filter.dbcc_devicetype = DBT_DEVTYP_DEVICEINTERFACE.0;
        notify_filter.dbcc_classguid = windows::core::GUID::from_values(
            0x25dbce51,
            0x6c8f,
            0x4a72,
            [0x8a, 0x6d, 0xb5, 0x4c, 0x2b, 0x4f, 0xc8, 0x35],
        );
        unsafe {
            RegisterDeviceNotificationW(
                HANDLE(window.0),
                &notify_filter as *const DEV_BROADCAST_DEVICEINTERFACE_W as *const c_void,
                POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(
                    DEVICE_NOTIFY_WINDOW_HANDLE.0 | DEVICE_NOTIFY_ALL_INTERFACE_CLASSES,
                ),
            );
        }
        println!("Register Device Notification");

        // 消息处理
        unsafe {
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

pub fn encode_wide(string: impl AsRef<std::ffi::OsStr>) -> Vec<u16> {
    string.as_ref().encode_wide().chain(once(0)).collect()
}

pub unsafe extern "system" fn call_default_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_DEVICECHANGE {
        match wparam.0 as u32 {
            DBT_DEVICEARRIVAL => {
                println!("device arrival");
            }
            DBT_DEVICEREMOVECOMPLETE => {
                println!("device remove complete");
            }
            DBT_DEVNODES_CHANGED => {
                println!("dev nodes changed");
            }
            _ => {
                println!(
                    "WM_DEVICECHANGE message received, value {} unhandled",
                    wparam.0
                );
            }
        }
        return LRESULT(0);
    }

    DefSubclassProc(hwnd, msg, wparam, lparam)
}
