use std::thread;
use std::thread::sleep;
use std::time::Duration;
use audio_recorder_rs::{Recorder};
use hound::{WavSpec, WavWriter};
use crate::audio_recorder::audio_recorder::{AudioRecorder, AudioRecorderState};

pub struct AudioRecorderAudioRecorder {
    state: AudioRecorderState,
    recorder: Option<Recorder>,
}

impl Default for AudioRecorderAudioRecorder {
    fn default() -> Self {
        Self {
            state: AudioRecorderState::Stopped,
            recorder: None,
        }
    }
}

impl AudioRecorder for AudioRecorderAudioRecorder {
    fn get_state(&self) -> &AudioRecorderState {
        &self.state
    }

    fn init(&mut self) {
        self.recorder = Some(Recorder::new());
    }

    fn record(&mut self) {
        let recorder = self.recorder.as_mut().unwrap();
        let receiver = recorder.start(
            true,
        ).expect("Failed to start audio receiver");

        // Configure the WAV file header
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = WavWriter::create(
            "recordings/test-2.wav",
            spec,
        ).unwrap();

        thread::spawn(move || {
            while let Ok(d) = receiver.recv() {
                for sample in d {
                    writer.write_sample(sample).ok();
                }
            }
        });

        sleep(Duration::from_secs(10));
        recorder.stop();
    }

    fn stop(&mut self) {
        self.recorder.as_mut().unwrap().stop();
    }
}