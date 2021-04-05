## 安装软件包管理工具choco

	administrator权限打开powershell, 执行:
		Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

	使用choco安装(admin权限):
		make - 构建工具
		openocd - debug server
		gcc-arm-embedded - 编译工具链

	choco install make openocd gcc-arm-embedded


## 安装非choco管理的软件

	这下面的在choco中没有找到, 需要手动安装:

		stm32cubemx:
			https://www.st.com/zh/development-tools/stm32cubemx.html
			
		stlink 驱动, 调试使用
			https://my.st.com/content/my_st_com/zh/products/development-tools/software-development-tools/stm32-software-development-tools/stm32-utilities/stsw-link009.html



## 使用 cmsisdap 调试 stm32

    创建文件 dap-stm32.cfg, 添加以下内容:

		interface cmsis-dap
		transport select swd
		source [find target/stm32f1x.cfg]

    烧写程序:
        openocd -f dap-stm32.cfg -c "program build/stm32f103rct6.bin 0x8000000 reset exit"


	用vscode调试, 安装插件: 1.c/c++ 2.cortex-debug

	打开 launch.json, 新添加一项调试配置:

		{
			"name": "Cortex Debug",
			"cwd": "${workspaceRoot}",
			"executable": "build/stm32f103rct6.elf",
			"request": "launch",
			"type": "cortex-debug",
			"servertype": "openocd",
			"configFiles": [
				"dap-stm32.cfg"
			]
		}
