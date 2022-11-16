use std::{env, io};

use earwig::utils::{next_sample, lerp};

// https://en.wikipedia.org/wiki/Xorshift
fn xorshift(mut state: u32) -> u32 {
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
    return state;
}

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // How strongly to interpolate samples toward chaos.
    let t: f64 = match args.get(1) {
        Some(ms) => ms.parse().unwrap_or(0.2),
        None => 0.5,
    };

    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let mut random_state = 1234567;

    // Infinite loop over samples passed via stdin
    loop {
        // Get the next sample
        let mut sample = match next_sample(&mut lines) {
            Some(sample) => sample,
            None => continue,
        };

        // Make things more chaotic.
        random_state = xorshift(random_state);
        // Tame the chaos into a random amplitude.
        let mut x = (random_state as f64) / (u32::MAX as f64);
        if random_state % 2 == 0 {
            x = -x;
        }

        // Turn that boring sample into something more chaotic!
        sample = lerp(sample, x, t);

        // print the sample to stdout
        println!("{sample}")
    }
}
