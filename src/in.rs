// use std::env;
//
// use web_audio_api::context::{AudioContext, BaseAudioContext};
//
// fn main() {
//     // Parse args or set defaults
//     let args: Vec<String> = env::args().collect();
//     let path = args.get(1).cloned().unwrap_or_else(|| "sample_01.wav".into());
//
//     // Get audio context
//     let context = AudioContext::default();
//
//     // Load and decode audio
//     let audio_file = std::fs::File::open(path).unwrap();
//
//     // TODO: there must be a better way to do this without making `web-audio-api` a dependency.
//     let buffer = context
//         .decode_audio_data_sync(audio_file)
//         .unwrap()
//         .get_channel_data(0)
//         .to_owned();
//
//     // Close the audio context as it is no longer needed.
//     context.close_sync();
//
//     // Pipe the samples into stdout
//     for sample in buffer {
//         println!("{sample}")
//     }
// }
