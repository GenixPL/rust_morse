use std::fs::File;
use rodio::{Decoder, MixerDeviceSink, Player};
use crate::audio_handler::audio_handler::AudioHandler;

#[derive(Default)]
pub struct RodioAudioHandler {
    handle: Option<MixerDeviceSink>,
}

impl AudioHandler for RodioAudioHandler {
    fn init(&mut self) {
        // Get an OS-Sink handle to the default physical sound device.
        // Note that the playback stops when the handle is dropped.//!
        self.handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream").into();

        let player = Player::connect_new(self.handle.as_ref().unwrap().mixer());
    }

    fn play(&self, file_path: &str) {
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = File::open(
            file_path /* "examples/crystal_bloom.mp3" */
        ).unwrap();

        // Decode that sound file into a source
        let source = Decoder::try_from(file).unwrap();

        // Play the sound directly on the device
        self.handle.as_ref().unwrap().mixer().add(source);
    }
}