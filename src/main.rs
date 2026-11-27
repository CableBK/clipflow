
slint::include_modules!();

use arboard::Clipboard;
use slint::{ModelRc, SharedString, VecModel};

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;

    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(error) => {
            eprintln!("Clipboard initialization failed: {error}");
            return Ok(());
        }
    };

    let items = match clipboard.get_text() {
        Ok(text) => vec![SharedString::from(text)],
        Err(error) => {
            eprintln!("Clipboard read failed: {error}");
            vec![SharedString::from(
                "No text available in clipboard"
            )]
        }
    };

    app.set_clipboard_items(
        ModelRc::new(VecModel::from(items))
    );

    app.run()
}
