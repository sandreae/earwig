use std::io::{self, Stdin};
use std::thread;
use std::time;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::unbounded;
use earwig::utils::next_sample;

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()).unwrap(),
    }
}

/// Launch a buffer which captures samples piped via stdin then sends them to the audio stream
/// which should output to the default audio device.
fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // Some setup
    let stdin = io::stdin();
    let channels = config.channels as usize;

    // Channel for sending samples from the buffer to the output stream.
    let (buffer_tx, buffer_rx) = unbounded();

    // Closure for retreiving a single sample from the buffer
    let mut next_value = move || -> (f64, f64) {
        let sample = buffer_rx
            .recv()
            .expect("receive sample from buffer channel");
        (sample, sample)
    };

    // Launch the buffer on it's own thread, passind in the channel sender
    let _ = thread::Builder::new()
        .spawn(move || buffer(buffer_tx, stdin))
        .expect("spawns buffer thread");

    // Method which handles errors occuring on the stream
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    // Build the stream, passing in our buffer getter
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;

    // Start the stream!!!
    stream.play()?;

    // Loop forever
    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
}

/// Buffer which pulls in samples from stdin, parses them, and then sends them
/// on to the output stream via a channel.
///
/// This method will be running in a seperate thread.
fn buffer(tx: crossbeam::channel::Sender<f64>, stdin: Stdin) {
    let mut lines = stdin.lines();

    loop {
        let sample = next_sample(&mut lines).unwrap_or_default();
        tx.send(sample).expect("Could not send on channel");
    }
}

// Helper for writing samples to the stream
fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f64, f64))
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = cpal::Sample::from::<f32>(&(sample.0 as f32));
        let right: T = cpal::Sample::from::<f32>(&(sample.1 as f32));

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}
