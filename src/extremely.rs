use std::{env, io};

use earwig::utils::next_sample;

// https://en.wikipedia.org/wiki/Linear_interpolation#Programming_language_support
fn lerp(v0: f64, v1: f64, t: f64) -> f64 {
    return (1.0 - t) * v0 + t * v1;
}

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // How strongly to interpolate samples toward +/- 1.
    let t: f64 = match args.get(1) {
        Some(ms) => ms.parse().unwrap_or(0.2),
        None => 1.0,
    };

    let stdin = io::stdin();
    let mut lines = stdin.lines();

    // Infinite loop over samples passed via stdin
    loop {
        // Get the next sample
        let mut sample = match next_sample(&mut lines) {
            Some(sample) => sample,
            None => continue,
        };

        // Turn that boring sample into something more extreme!
        sample = lerp(sample, sample.signum(), t);

        // print the sample to stdout
        println!("{sample}")
    }
}
