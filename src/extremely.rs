use clap::Parser;

use earwig::utils::{sample_loop, lerp};

/// Linearly interpolate each sample toward +/-1
#[derive(Parser, Debug)]
struct Args {
   /// How strongly to interpolate samples toward +/- 1.
   #[arg(default_value_t = 1.0)]
   t: f64,
}

fn main() {
    let args = Args::parse();

    sample_loop![mut sample in {
        // Turn that boring sample into something more extreme!
        sample = lerp(sample, sample.signum(), args.t);

        // print the sample to stdout
        println!("{sample}")
    }];
}
