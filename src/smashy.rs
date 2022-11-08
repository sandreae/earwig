use std::io;

/// Takes a sample, compares it with the passed max_samlple (accounting for minus values)
/// then adds half the difference and returns.
fn smash(sample: f64, max_sample: f64) -> f64 {
    if sample >= 0.0 {
        // for positive values add the difference
        sample + ((max_sample - sample) / 2.0)
    } else {
        // for minus values deduct the difference
        0.0 - (sample - ((max_sample + sample) / 2.0))
    }
}

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lines();
    let mut sample: f64 = 0.0;
    let mut max_sample = 0.8;

    // Infinite loop over samples passed via stdin
    loop {
        let line = lines.next();
        sample = match line {
            Some(line) => match line.expect("can read line").parse::<f64>() {
                Ok(sample) => {
                    // keep track of the max sample
                    if sample > max_sample {
                        max_sample = sample
                    };
                    // smash every sample
                    smash(sample, max_sample)
                }
                Err(_) => sample,
            },
            None => sample,
        };

        // print the smashed sample to stdout
        println!("{sample}")
    }
}
