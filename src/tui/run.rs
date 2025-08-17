use std::path::Path;

use crate::core::task::Tasks;

use super::app::App;

pub fn run_tui() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let path = Path::new("tasks.json");
    let manager = Tasks::from_path(path);
    let app_result = App::new(manager).run(terminal);
    ratatui::restore();
    app_result
}
