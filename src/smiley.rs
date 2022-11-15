// Always stays positive :-)
// (except for the cynic)


use std::env;
use std::io;

fn main() {
    // Parse args or set defaults
    let args: Vec<String> = env::args().collect();

    let is_manic = args.get(1).map(|arg| arg == "manic").unwrap_or(false);
    let is_cynic = args.get(1).map(|arg| arg == "cynic").unwrap_or(false);

    let stdin = io::stdin();

    let mut lines = stdin.lines();
    let mut previous_sample: f64 = 1.0;

    // Infinite loop over samples passed via stdin
    loop {
        let line = lines.next();
        match line {
            Some(line) => match line.expect("can read line").parse::<f64>() {
                Ok(sample) => {
                    if is_manic {
                        println!("{}", sample.abs());
                    } else if is_cynic {
                        println!("{}", sample * previous_sample);
                        previous_sample = sample;
                    } else {
                        println!("{}", sample.max(0.0));
                    }
                }
                Err(_) => {},
            },
            None => {},
        };
    }
}
