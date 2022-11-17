use std::env;

use earwig::utils::{sample_loop, lerp};

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // How strongly to interpolate samples toward +/- 1.
    let t: f64 = match args.get(1) {
        Some(t) => t.parse().unwrap_or(1.0),
        None => 1.0,
    };

    sample_loop![mut sample in {
        // Turn that boring sample into something more extreme!
        sample = lerp(sample, sample.signum(), t);

        // print the sample to stdout
        println!("{sample}")
    }];
}
