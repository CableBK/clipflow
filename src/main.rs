slint::include_modules!();

use slint::{ModelRc, SharedString, VecModel};

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;

    let items = vec![
        SharedString::from("Hello, welcome to ClipFlow!"),
        SharedString::from("Cable"),
        SharedString::from("https://github.com/"),
        SharedString::from("Hello Rust Programming"),
    ];
    app.set_clipboard_items(ModelRc::new(VecModel::from(items)));

    app.run()
}
