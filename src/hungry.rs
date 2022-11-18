use std::env;

use earwig::utils::sample_loop;

fn eat(sample: f64, threshold: f64) -> Option<f64> {
    if sample > threshold || sample < (0.0 - threshold) {
        Some(sample)
    } else {
        None
    }
}

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // Set the amplitude threshold bellow which any samples will be eaten
    let threshold: f64 = match args.get(1) {
        Some(ms) => ms.parse().unwrap_or(0.2),
        None => 0.2,
    };

    sample_loop![mut sample in {
        // Feeed the sample to hungry
        sample = match eat(sample, threshold) {
            Some(sample) => sample,
            None => continue,
        };
    
        // print the sample to stdout (if it wasn't eaten)
        println!("{sample}")
    }];
}
