use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Waveform {
    /// A sine wave
    Sine,
    /// A square wave
    Square,
    /// A sawtooth wave
    Saw,
}
use Waveform::*;

/// Oscillator... Silly... Get it?
#[derive(Parser)]
struct Args {
    /// Frequency
    #[arg(default_value_t = 440.0)]
    f: f64,
    /// Max amplitude
    #[arg(short, default_value_t = 1.0)]
    a: f64,
    /// The waveform to generate
    #[arg(value_enum, default_value_t = Sine)]
    wave: Waveform,
}

// Generate a sin wave sample
fn sin(time: f64, amplitude: f64, frequency: f64) -> f64 {
    amplitude * f64::sin(frequency * 2.0 * std::f64::consts::PI * time)
}

// Generate a square wave sample
fn square_wave(time: f64, amplitude: f64, frequency: f64) -> f64 {
    if (time * frequency).fract() >= 0.5 {
        amplitude
    } else {
        -amplitude
    }
}

// Generate a sawtooth wave sample
fn sawtooth_wave(time: f64, amplitude: f64, frequency: f64) -> f64 {
    amplitude * ((time * frequency).fract() * 2.0 - 1.0)
}

fn main() {
    let args = Args::parse();

    // tiiiiiime
    let mut t = 0.0;

    loop {
        // Increment time
        t += 1.0 / 44100.0;

        
        let sample = match args.wave {
            Sine => sin(t, args.a, args.f),
            Square => square_wave(t, args.a, args.f),
            Saw => sawtooth_wave(t, args.a, args.f),
        };

        println!("{sample}");
    }
}
