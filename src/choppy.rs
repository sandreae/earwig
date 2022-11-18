use std::env;

use earwig::utils::next_sample;

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();
    // The number of samples in each chopped chunk of audio.
    let chunk_size: usize = match args.get(1) {
        Some(ms) => ms.parse().unwrap_or(1000),
        None => 1000,
    };
    // The max length of the audio buffer.
    let buffer_length_secs: usize = match args.get(2) {
        Some(ms) => ms.parse().unwrap_or(300),
        None => 300,
    };
    let sample_rate = 44100;
    let mut buffer = Vec::new();

    // Capture samples piped in via stdin and fill the buffer
    //
    // The buffer is capped to 3 minutes of audio at a sample rate of 44100.
    while buffer.len() < sample_rate * buffer_length_secs {
        let sample = match next_sample() {
            Some(sample) => sample,
            None => break
        };
        buffer.push(sample);
    }

    loop {
        // Now we chop out semi randomised chunks from the audio buffer and pipe
        // the raw samples straight to stdout.

        // 1. Get a randomised offset within the bounds of the buffer length
        let offset = (buffer.len() as f32 * rand::random::<f32>()) as usize;

        // 2. From the offset position onwards, chop off samples one by one
        for i in 0..(sample_rate * chunk_size / 1000) {
            match buffer.get(i + offset) {
                // 3. And print them to stdout, one line per sample
                Some(sample) => println!("{sample}"),
                None => break,
            }
        }
    }
}
