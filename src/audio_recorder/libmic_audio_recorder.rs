use libmic_rs::mic::Recorder;
use crate::audio_recorder::audio_recorder::{AudioRecorder, AudioRecorderState};

pub struct LibmicAudioRecorder {
    state: AudioRecorderState,
}

impl Default for LibmicAudioRecorder {
    fn default() -> Self {
        Self {
            state: AudioRecorderState::Stopped,
        }
    }
}

impl AudioRecorder for LibmicAudioRecorder {
    fn get_state(&self) -> &AudioRecorderState {
        &self.state
    }

    fn init(&mut self) {}

    fn record(&mut self) {
        let _ = Recorder::record_to_file("recordings/test.wav", 5);
    }

    fn stop(&mut self) {}
}