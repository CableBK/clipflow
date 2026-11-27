slint::include_modules!();

use slint::{ModelRc, SharedString, VecModel};

fn main() -> Result<(), slint::PlatformError> {

    let app = MainWindow::new()?;

    app.set_clipboard_items(
        ModelRc::new(VecModel::from(items))
    );

    app.run()
}
