use std::collections::HashSet;
use statrs::statistics::Statistics;

pub fn morse_decode(file_path: &str) -> String {
    let mut reader = hound::WavReader::open(file_path).unwrap();
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
                // If it's different - one-off, then mark it the same as chunk's.
                sample_silence = chunk_silence
            }
        }

        if chunk_silence == sample_silence {
            chunk_length += 1;
        } else {
            // println!("len: {}, silence: {}", chunk_length, chunk_silence);

            chunk_lengths.push(chunk_length);
            chunk_silence = sample_silence;
            chunk_length = 1;
        }
    }

    let chunk_sizes = chunk_lengths.into_iter().collect::<HashSet<_>>();

    println!("{:?}", chunk_sizes);
    // println!("{:?}", &samples[60..=65]);

    "dupa".to_string()
}

fn median(mut list: Vec<u32>) -> u32 {
    // 1. Arrays must be sorted to find the median
    list.sort();

    let mid = list.len() / 2;
    let median = if list.len() % 2 == 0 {
        (list[mid - 1] + list[mid]) / 2
    } else {
        list[mid]
    };

    median
}