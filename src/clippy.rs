use std::env;

use earwig::utils::sample_loop;

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // Set the amplitude threshold for clippy to interact with samples.
    let threshold: f64 = match args.get(1) {
        Some(threshold) => threshold.parse().unwrap_or(0.3),
        None => 0.3,
    };

    let mut lawful = true;
    let mut good = true;

    // Including an argument `"chaotic"` and/or `"evil"` amongst the second two arguments moves clippy's alignment away from lawful good.
    for i in 2..4 {
        if let Some(arg) = args.get(i) {
            if arg == "chaotic" {
                lawful = false;
            }
            if arg == "evil" {
                good = false;
            }
        }
    }

    sample_loop![mut sample in {
        // We need to clip the sample.
        if lawful && sample.abs() > threshold {
            if good {
                // We can fix this sample!
                sample = threshold * sample.signum();
            } else {
                // This sample has no right to pass through *evil laugh*!
                continue
            }
        }

        // Oops, we forgot how clipping is supposed to work.
        if !lawful && sample.abs() < threshold {
            if good {
                // We can "fix" this sample!
                sample = threshold * sample.signum();
            } else {
                // This sample has no right to pass through *evil laugh*!
                continue
            }
        }

        // print the sample to stdout (if it wasn't evilly dispatched of)
        println!("{sample}");
    }];
}
