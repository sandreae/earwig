use std::collections::VecDeque;
use std::env;

use earwig::utils::sample_loop;

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // The length (ms) of each reversed period
    let reverse_duration_ms: usize = match args.get(1) {
        Some(ms) => ms.parse().unwrap_or(1000),
        None => 1000,
    };

    let sample_rate = 44100;
    let mut buffer = VecDeque::new();

    sample_loop![sample in {
        buffer.push_front(sample);
    
        // Once we have filled the buffer with the required number of samples
        // we pipe them out adain in reverse order.
        if buffer.len() > sample_rate * reverse_duration_ms as usize / 1000 {
            for sample in &buffer {
                println!("{sample}")
            }
            buffer.clear()
        }
    }];
}
