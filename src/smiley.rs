// Always stays positive :-)
// (except for the cynic)

use std::env;

use earwig::utils::sample_loop;

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    let is_manic = args.get(1).map(|arg| arg == "manic").unwrap_or(false);
    let is_cynic = args.get(1).map(|arg| arg == "cynic").unwrap_or(false);

    let mut previous_sample: f64 = 1.0;

    sample_loop![sample in {
        if is_manic {
            println!("{}", sample.abs());
        } else if is_cynic {
            println!("{}", sample * previous_sample);
            previous_sample = sample;
        } else {
            println!("{}", sample.max(0.0));
        }
    }];
}
