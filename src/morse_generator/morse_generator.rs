use hound::WavWriter;
use morsify::{MorseCharacterSet, MorseCode, Options};

pub fn generate_morse(input: &str, file_path: &str) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(file_path, spec)
        .map_err(|e| e.to_string())?;

    let sample_rate = 44100.0;
    let dot_duration = 0.1; // Seconds
    let frequency = 700.0;  // Hz

    let morse_code = to_morse(input);
    for char in morse_code.chars() {
        match char {
            '.' => {
                write_sample(&mut writer, dot_duration, sample_rate, frequency);
                // "intra-character space"
                write_silence(&mut writer, dot_duration, sample_rate);
            }
            '-' => {
                write_sample(&mut writer, dot_duration * 3.0, sample_rate, frequency);
                // "intra-character space"
                write_silence(&mut writer, dot_duration, sample_rate);
            }
            // Between words ("inter-word space")
            '/' => {
                // 6 because it can occur only after '.' or '-'
                // which already add one duration of silence (total 7)
                write_silence(&mut writer, dot_duration * 6.0, sample_rate);
            }
            // Between chars ("inter-character space")
            ' ' => {
                // 2 because it can occur only after '.' or '-'
                // which already add one duration of silence (total 3)
                write_silence(&mut writer, dot_duration * 2.0, sample_rate);
            }
            _ => {
                panic!("Unrecognized morse character: {}", char);
            }
        }
    }


    Ok(())
}

fn to_morse(input: &str) -> String {
    let options = Options {
        dash: '-',
        dot: '.',
        space: '/',
        separator: ' ',
        character_set_order: vec![
            MorseCharacterSet::Latin,
            MorseCharacterSet::Numbers,
            MorseCharacterSet::Punctuation,
            MorseCharacterSet::Greek,
        ],
    };

    let morse_code = MorseCode::new(options);

    // Encode a text message to Morse code
    morse_code.encode(input)
}

fn write_sample(
    writer: &mut WavWriter<std::io::BufWriter<std::fs::File>>,
    duration: f32,
    rate: f32,
    frequency: f32,
) {
    // Generate Sine Wave for the tone
    for t in 0..(duration * rate) as u32 {
        let sample = (t as f32 * frequency * 2.0 * std::f32::consts::PI / rate).sin();
        let amplitude = i16::MAX as f32 * 0.5;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

fn write_silence(
    writer: &mut WavWriter<std::io::BufWriter<std::fs::File>>,
    duration: f32,
    rate: f32,
) {
    for _ in 0..(duration * rate) as u32 {
        writer.write_sample(0i16).unwrap();
    }
}