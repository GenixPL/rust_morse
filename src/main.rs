#![allow(warnings)]

mod app_runner;
mod audio_recorder;
mod text_handler;
mod features;
mod timer;
mod morse;

mod audio_handler {
    pub mod audio_handler;
    pub mod cpal_audio_handler;
    pub mod rodio_audio_handler;
}

use std::cell::RefCell;
use std::rc::Rc;
use ratatui::*;
use crate::app_runner::AppRunner;
use crate::audio_handler::rodio_audio_handler::RodioAudioHandler;
use crate::audio_recorder::audio_recorder_audio_recorder::AudioRecorderAudioRecorder;
use crate::morse::morse_decoder::morse_decode;

fn main() -> color_eyre::Result<()> {
    // morse_decode("recordings/working_morse_encode.wav");
    
    let mut app_runner: AppRunner = AppRunner::new(
        Rc::new(RefCell::new(RodioAudioHandler::default())),
        Box::new(AudioRecorderAudioRecorder::default()),
    );

    color_eyre::install()?;
    run(|terminal| app_runner.run(terminal))?;
    Ok(())
}










