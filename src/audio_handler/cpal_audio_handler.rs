use std::time::Duration;
use cpal::{Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::audio_handler::audio_handler::AudioHandler;

#[derive(Default)]
pub struct CpalAudioHandler {
    pub stream: Option<cpal::Stream>,
}

impl AudioHandler for CpalAudioHandler {
    fn init(&mut self) {
        println!("Initializing cpal audio handler...");

        let host = cpal::default_host();

        let device = host.default_output_device().expect("no output device available");
        println!("Using audio device: {}", device.name().expect("no device available"));

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");

        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let sample_format = supported_config.sample_format();

        let config = supported_config.into();
        println!("config: {:?}", config);

        self.stream = match sample_format {
            SampleFormat::F32 => device.build_output_stream(
                &config,
                move |data, info| {
                    Self::write_silence::<f32>(data, info)
                },
                err_fn,
                None,
            ).ok(),
            SampleFormat::I16 => device.build_output_stream(
                &config,
                move |data, info| {
                    Self::write_silence::<i16>(data, info)
                },
                err_fn,
                None,
            ).ok(),
            SampleFormat::U16 => device.build_output_stream(
                &config,
                move |data, info| {
                    Self::write_silence::<u16>(data, info)
                },
                err_fn,
                None,
            ).ok(),
            sample_format => panic!("Unsupported sample format '{sample_format}'")
        };
    }

    fn play(&self, file_path: &str) {
        println!("Audio handler playing");
        println!("{:?}", self.stream.is_some());
        self.stream.as_ref().unwrap().play().expect("TODO: panic message");
        std::thread::sleep(Duration::from_secs(5));
    }
}

impl CpalAudioHandler {
    fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
        for sample in data.iter_mut() {
            *sample = Sample::EQUILIBRIUM;
            //     match data. {
            //     SampleFormat::F32 => sound.get_f32(),
            // };
        }
    }
}

