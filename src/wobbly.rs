use std::env;

use earwig::sample_loop;

// Generate a sin wave sample
fn sin(time: f64, amplitude: f64, frequency: f64) -> f64 {
    amplitude * f64::sin(frequency * 2.0 * std::f64::consts::PI * time)
}

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    // The frequency of the modulation signal
    let f: f64 = match args.get(1) {
        Some(ms) => ms.parse().unwrap_or(0.5),
        None => 0.5,
    };

    // tiiiiiime
    let mut t = 0.0;

    // p.....??
    let mut p = 0.0;

    sample_loop![sample in {
        // Increment time
        t += 1.0 / 44100.0;

        // Generate single sample of a sin tone based on current time and
        // provided frequency and amplitude of 0.5. 
        //
        // Bring all values into a range between 0.5 and 1.5.
        let x = sin(t, 0.5, f) + 1.0;

        // It's like `speedy` but all wobbly now!!
        p += 1.0;
        while 0.0 < p && p <= 1.0 {
            p -= x;
            println!("{sample}");
        }
    }];
}
