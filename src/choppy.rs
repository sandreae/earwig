use std::env;

use web_audio_api::context::{AudioContext, BaseAudioContext};

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "sample.wav".into());
    let sample_size: usize = match args.get(2) {
        Some(ms) => ms.parse().unwrap_or(1000),
        None => 1000,
    };

    // Get audio context
    let context = AudioContext::default();
    let sample_rate = context.sample_rate();

    // Load and decode audio
    let audio_file = std::fs::File::open(path).unwrap();

    // TODO: there must be a better way to do this without making `web-audio-api` a dependency. 
    let buffer = context
        .decode_audio_data_sync(audio_file)
        .unwrap()
        .get_channel_data(0)
        .to_owned();

    // Close the audio context as it is no longer needed.
    context.close_sync();
    
    loop {
        // Now we chop out semi randomised chunks from the audio buffer and pipe
        // the raw samples straight to stdout.

        // 1. Get a randomised offset within the bounds of the buffer length
        let offset = (buffer.len() as f32 * rand::random::<f32>()) as usize;

        // 2. From the offset position onwards, chop off samples one by one
        for i in 0..(sample_rate as usize * sample_size / 1000) {
            match buffer.get(i + offset) {
                // 3. And print them to stdout, one line per sample
                Some(sample) => println!("{sample}"),
                None => break,
            }
        }
        }
}
