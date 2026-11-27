
slint::include_modules!();

use arboard::Clipboard;
use slint::{ModelRc, SharedString, VecModel};

fn main() -> Result<(), slint::PlatformError> {
    // สร้างหน้าต่าง
    let app = MainWindow::new()?;

    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(error) => {
            eprintln!("Clipboard initialization failed: {error}");
            return app.run();
        }
    };

    let items = match clipboard.get_text() {
        Ok(text) if !text.trim().is_empty() => {
            vec![SharedString::from(text)]
        }
        Ok(_) => {
            vec![]
        }
        Err(error) => {
            eprintln!("Clipboard read failed: {error}");
            vec![]
        }
    };

    app.set_clipboard_items(
        ModelRc::new(VecModel::from(items))
    );

    app.run()
}
