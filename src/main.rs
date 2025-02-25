// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = AppWindow::new()?;

    let context = app.global::<ComponentContext>();
    let mut username: String = context.get_username().into();
    let lastname = username.pop().unwrap_or_default();
    context.set_lastname(lastname.into());
    context.on_execute(|func_idx, sub_idx| {
        println!("function index: {func_idx}, sub-function index: {sub_idx}");
    });

    // app.on_request_increase_value({
    //     let ui_handle = app.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    app.run()?;

    Ok(())
}
