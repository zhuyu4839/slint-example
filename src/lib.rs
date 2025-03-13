use std::rc::Rc;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

pub(crate) mod ui;

pub fn main() -> anyhow::Result<()> {
    let app = ui::AppWindow::new()?;

    let context = app.global::<ui::Context>();
    let mut username: String = context.get_username().into();
    let lastname = username.pop().unwrap_or_default();
    context.set_lastname(lastname.into());
    app.global::<ui::Context>().on_button_clicked(|func_idx, sub_idx| {
        println!("function index: {func_idx}, sub-function index: {sub_idx}");
    });

    let functions = context.get_functions();
    let length = functions.iter().len();
    context.set_goupbox_heights(ModelRc::from(Rc::new(VecModel::from(vec![0.; length]))));

    app.run()?;

    Ok(())
}


