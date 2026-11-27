slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = slint::load_module!("main.slint")?;
    app.run()?;
    Ok(())
}