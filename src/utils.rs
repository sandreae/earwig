//! Utility methods used across multiple modules

use std::io::{Lines, StdinLock};

pub fn next_sample(lines: &mut Lines<StdinLock>) -> f64 {
    let line = lines.next();

    match line {
        Some(line) => line.expect("can read line").parse().unwrap_or(0_f64),
        None => 0_f64,
    }
}