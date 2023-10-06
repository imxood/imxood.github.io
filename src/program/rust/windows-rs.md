## String -> HSTRING -> PCWSTR

```rs
pub fn SetCurrentProcessExplicitAppUserModelID() {
    unsafe {
        match Shell::SetCurrentProcessExplicitAppUserModelID(PCWSTR::from_raw(
            HSTRING::from("EhExt").as_ptr(),
        )) {
            Ok(s) => {
                log::info!("SetCurrentProcessExplicitAppUserModelID ok");
            }
            Err(e) => {
                log::info!("SetCurrentProcessExplicitAppUserModelID Error: {e}");
            }
        }
    }
}
pub fn GetCurrentProcessExplicitAppUserModelID() {
    unsafe {
        match Shell::GetCurrentProcessExplicitAppUserModelID() {
            Ok(s) => {
                let s = OsString::from_wide(s.as_wide())
                    .to_string_lossy()
                    .into_owned();
                log::info!("GetCurrentProcessExplicitAppUserModelID: {}", s);
            }
            Err(e) => {
                log::info!("GetCurrentProcessExplicitAppUserModelID Error: {e}");
            }
        }
    }
}
```