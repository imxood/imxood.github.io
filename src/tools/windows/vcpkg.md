git clone https://github.com/microsoft/vcpkg

.\vcpkg\bootstrap-vcpkg.bat

添加 .\vcpkg 到环境变量中.

.\vcpkg\vcpkg integrate install

.\vcpkg\vcpkg search [search term]

vcpkg help triplet

.\vcpkg\vcpkg install libusb --triplet=x64-windows-static-md
