#![allow(warnings)]

mod app_state;
mod app_runner;
mod audio_recorder;

mod audio_handler {
    pub mod audio_handler;
    pub mod cpal_audio_handler;
    pub mod rodio_audio_handler;
}

use ratatui::*;
use crate::app_runner::AppRunner;
use crate::audio_handler::rodio_audio_handler::RodioAudioHandler;
use crate::audio_recorder::audio_recorder_audio_recorder::AudioRecorderAudioRecorder;

fn main() -> color_eyre::Result<()> {
    let mut app_runner: AppRunner = AppRunner::new(
        Box::new(RodioAudioHandler::default()),
        Box::new(AudioRecorderAudioRecorder::default()),
    );

    color_eyre::install()?;
    run(|terminal| app_runner.run(terminal))?;
    Ok(())
}








