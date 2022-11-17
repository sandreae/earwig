use std::{env, io};

use earwig::utils::next_sample;

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // Set the factor by which to change speed.
    let x: f64 = match args.get(1) {
        Some(ms) => ms.parse().unwrap_or(0.5),
        None => 0.5,
    };

    let stdin = io::stdin();
    let mut lines = stdin.lines();

    // why should the comments always explain what is happening, what about the joy of working out why things work on your own?
    let mut p = 0.0;

    // Infinite loop over samples passed via stdin
    loop {
        // Get the next sample
        let sample = match next_sample(&mut lines) {
            Some(sample) => sample,
            None => continue,
        };

        // Isn't this neat?
        p += 1.0;
        while 0.0 < p && p <= 1.0 {
            p -= x;
            println!("{sample}");
        }
    }
}
