//! Utility methods used across multiple modules

use std::io::{Lines, StdinLock};

pub fn next_sample(lines: &mut Lines<StdinLock>) -> Option<f64> {
    let line = lines.next();

    match line {
        Some(line) => {
            match line.expect("Can read line").parse() {
                Ok(sample) => Some(sample),
                Err(_) => None,
            }
        },
        None => None,
    }
}

// https://en.wikipedia.org/wiki/Linear_interpolation#Programming_language_support
pub fn lerp(v0: f64, v1: f64, t: f64) -> f64 {
    return (1.0 - t) * v0 + t * v1;
}