// Disclaimer:
// - Beyond an educated guess that samples might denote an amplitude between -1 to 1, I have no idea how any of this works.
// - I have not listened to what this produces. The initial idea was to make things sluggish by limiting how much samples would differ from the previous ones. But perhaps all this does is to reduce volume in a slightly random way? Only one way to find out: submitting a PR and having you describe the effect to me.

use std::env;
use std::collections::VecDeque;

use earwig::utils::sample_loop;

// Smooth things out by averaging every sample over past samples.

// Maintain a buffer of the last `n` samples, adding `sample` as the newest member at index 0.
fn feed_blender(buf: &mut VecDeque<f64>, sample: f64, n: usize) {
    if buf.len() == n {
        buf.pop_back();
    }
    buf.push_front(sample);
}

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // How long to leave the smoothie in the blender (how many past samples to average over).
    let blending_time: usize = match args.get(1) {
        Some(ms) => ms.parse().map(|n: usize| n + 1).unwrap_or(1),
        None => 1,
    };

    let mut buf = VecDeque::new();

    sample_loop![mut sample in {
        feed_blender(&mut buf, sample, blending_time);
    
        let mut smoothed = 0.0;
        for i in 0..blending_time {
            smoothed += buf.get(i).map(Clone::clone).unwrap_or(sample);
        }
    
        sample = smoothed / (blending_time as f64);
    
        // print the smoothed sample to stdout
        println!("{sample}")
    }];
}
