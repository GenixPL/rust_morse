use ratatui::*;
use crate::app_runner::AppRunner;

mod app_state;
mod app_runner;
mod audio_handler;

fn main() -> color_eyre::Result<()> {
    let mut app_runner: AppRunner = AppRunner::default();

    color_eyre::install()?;
    run(|terminal| app_runner.run(terminal))?;
    Ok(())
}








