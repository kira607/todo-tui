mod app;
mod core;
mod utils;
mod widgets;

use std::path::Path;

use core::Tasks;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let path = Path::new("tasks.json");
    let manager = Tasks::from_path(path);
    let app_result = App::new(manager).run(terminal);
    ratatui::restore();
    app_result
}
