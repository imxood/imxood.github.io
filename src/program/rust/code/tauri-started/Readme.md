
cargo install tauri-cli --version "^1.0.0"

cargo tauri --help

mkdir tauri-started && cd tauri-started

cargo tauri init

cargo tauri dev

根据提示:

✔ What is your app name? · tauri-started
✔ What should the window title be? · hello world
✔ Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src-tauri/tauri.conf.json" file that will be created? · ../web
✔ What is the url of your dev server? · ../web
