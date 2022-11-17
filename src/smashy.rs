use earwig::utils::sample_loop;

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
    let mut max_sample = 0.8;

    sample_loop![mut sample in {
        // Update max_sample
        if sample > max_sample {
            max_sample = sample
        };
    
        // smash the sample
        sample = smash(sample, max_sample);
    
        // print the smashed sample to stdout
        println!("{sample}")
    }];
}
