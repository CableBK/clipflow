slint::include_modules!();

use arboard::Clipboard;
use slint::{ComponentHandle, ModelRc, SharedString, Timer, TimerMode, VecModel};
use std::time::Duration;

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;

    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(error) => {
            eprintln!("Clipboard initialization failed: {error}");
            return app.run();
        }
    };

    let history = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));

    let last_text = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let app_weak = app.as_weak();

    let timer = Timer::default();

    let history_clone = history.clone();
    let last_text_clone = last_text.clone();

    timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
        let text = match clipboard.get_text() {
            Ok(text) if !text.trim().is_empty() => text,
            _ => return,
        };

        if *last_text_clone.borrow() == text {
            return;
        }

        *last_text_clone.borrow_mut() = text.clone();

        let mut history = history_clone.borrow_mut();

        history.retain(|item| item != &text);

        history.insert(0, text);

        history.truncate(50);

        let items = history
            .iter()
            .cloned()
            .map(SharedString::from)
            .collect::<Vec<_>>();

        drop(history);

        if let Some(app) = app_weak.upgrade() {
            app.set_clipboard_items(ModelRc::new(VecModel::from(items)));
        }
    });

    app.run()
}
