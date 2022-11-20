use std::env;

use earwig::utils::{sample_loop, lerp};

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
        Some(t) => t.parse().unwrap_or(0.5),
        None => 0.5,
    };

    let mut random_state = 1234567;

    sample_loop![mut sample in {
        // Make things more chaotic.
        random_state = xorshift(random_state);
        // Forge the chaos into a random amplitude of the same sign as the current sample.
        let mut x = (random_state as f64) / (u32::MAX as f64);
        x *= sample.signum();

        // Turn that boring sample into something more chaotic!
        sample = lerp(sample, x, t);

        // print the sample to stdout
        println!("{sample}");
    }];
}
