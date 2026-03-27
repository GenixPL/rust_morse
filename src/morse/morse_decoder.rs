use std::collections::HashSet;
use morsify::{MorseCharacterSet, MorseCode, Options};
use statrs::statistics::Statistics;

pub fn morse_decode(file_path: &str) -> String {
    let mut reader_res = hound::WavReader::open(file_path);
    let mut reader = match reader_res {
        Ok(reader) => reader,
        Err(_) => {
            panic!("Error opening file: {}", file_path);
        }
    };
    let spec = reader.spec();

    let samples: Vec<f32> = reader.samples::<i16>().
        map(|e| {
            // raw data
            let value = e.unwrap();
            // normalized to <-1, 1>
            let normalized = value as f32 / i16::MAX as f32;
            // abs to <0, 1>
            normalized.abs()
        })
        .collect();

    // sample below this value is interpreted as silence
    let silence_threshold = 0.0;

    let mut decoded: String = "".to_string();

    let mut chunk_lengths: Vec<u32> = vec![];
    let mut chunk_length: u32 = 1;
    let mut chunk_silence: bool = samples[0] <= silence_threshold;
    for i in 1..(samples.len() - 1) {
        let sample = samples[i];

        let mut sample_silence = sample <= silence_threshold;
        if sample_silence != chunk_silence {
            // If the new one is different from the previous one,
            // check the next one, and ignore the change if it's
            // one - off (the samples might have singular values
            // that belong to the other category).
            if sample_silence != (samples[i + 1] <= silence_threshold) {
                // println!("off at {}", i);
                // If it's different - one-off, then mark it the same as chunk's.
                sample_silence = chunk_silence
            }
        }

        if chunk_silence == sample_silence {
            chunk_length += 1;
        } else {
            // println!("len: {}, silence: {}", chunk_length, chunk_silence);

            if chunk_length < 4000 {
                // Nothing
            } else if chunk_length < 13_000 {
                if chunk_silence {
                    //
                } else {
                    decoded.push('.');
                }
            } else if chunk_length < 30000 {
                if chunk_silence {
                    decoded.push(' ');
                } else {
                    decoded.push('-');
                }
            } else {
                decoded.push('/');
            }

            // println!("len: {}", chunk_length);
            // println!("sil: {}", chunk_silence);

            chunk_lengths.push(chunk_length);
            chunk_silence = sample_silence;
            chunk_length = 1;
        }
    }

    // println!("{:?}", chunk_lengths);
    println!("decoded: {}", decoded);
    // println!("{:?}", &samples[60..=65]);

    // "dupa".to_string()
    from_morse(decoded.as_str())
}

fn from_morse(input: &str) -> String {
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
    morse_code.decode(input)
}