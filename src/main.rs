mod app_state;
mod app_runner;
mod audio_handler {
    pub mod audio_handler;
    pub mod cpal_audio_handler;
}

use ratatui::*;
use crate::app_runner::AppRunner;
use crate::audio_handler::cpal_audio_handler::CpalAudioHandler;

fn main() -> color_eyre::Result<()> {
    let mut app_runner: AppRunner = AppRunner::new(
        Box::new(CpalAudioHandler::default())
    );

    color_eyre::install()?;
    run(|terminal| app_runner.run(terminal))?;
    Ok(())
}








