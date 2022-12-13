//! Utility methods used across multiple modules.

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, SupportedStreamConfig};

/// Generates a loop that executes once for each sample arriving on stdin.
///
/// Call as `sample_loop![sample in {this_code_can_use(sample);}]`
/// or `sample_loop![mut sample in {sample = this_code_can_mutably_use(sample);}]`.
#[macro_export]
macro_rules! sample_loop {
    ($sample:pat in $body:block) => {
        for raw in std::io::stdin().lines() {
            match raw.expect("Can read line").parse::<f64>() {
                Ok($sample) => $body,
                Err(_) => continue, // We forgivingly ignore input that does not parse as a float.
                                    // Neither do we enforce that float to lie between -1 and 1 btw.
            }
        }
    };
}

pub use sample_loop;

/// Retreive the next single sample which arrived via stdin.
///
/// Returns an option which will be None when there is no sample available or if what is
/// received from stdin could not be parsed into an f64.
pub fn next_sample() -> Option<f64> {
    let mut lines = std::io::stdin().lines();

    match lines.next() {
        Some(line) => match line.expect("Can read line").parse() {
            Ok(sample) => Some(sample),
            Err(_) => None,
        },
        None => None,
    }
}

// https://en.wikipedia.org/wiki/Linear_interpolation#Programming_language_support
pub fn lerp(v0: f64, v1: f64, t: f64) -> f64 {
    (1.0 - t) * v0 + t * v1
}

// Get default audio device and config
pub fn audio_device() -> (Device, SupportedStreamConfig) {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    (device, config)
}
