// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;
use slint::{Model, ModelRc, VecModel};

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = AppWindow::new()?;

    let context = app.global::<Context>();
    let mut username: String = context.get_username().into();
    let lastname = username.pop().unwrap_or_default();
    context.set_lastname(lastname.into());
    app.global::<Context>().on_button_clicked(|func_idx, sub_idx| {
        println!("function index: {func_idx}, sub-function index: {sub_idx}");
    });

    let functions = context.get_functions();
    let length = functions.iter().len();
    context.set_goupbox_heights(ModelRc::from(Rc::new(VecModel::from(vec![0.; length]))));

    app.run()?;

    Ok(())
}
