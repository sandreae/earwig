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