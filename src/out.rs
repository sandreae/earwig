use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::unbounded;
use earwig::sample_loop;

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

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // Channel for sending samples to the output stream.
    let (buffer_tx, buffer_rx) = unbounded();

    // Closure for retreiving a single sample from the buffer
    let mut next_value = move || -> (f64, f64) {
        // We only deal with mono modules for now so here we split the signal
        // for stereo output.
        let sample = buffer_rx
            .recv()
            .expect("Receive sample from buffer channel");
        (sample, sample)
    };

    // Build the stream, passing in our buffer getter
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        |err| eprintln!("An error occurred on stream: {}", err),
    )?;

    // Start the stream!!!
    stream.play()?;

    // Iterate over samples recieved in stdin and send them to the output stream.
    //
    // Terminates when stdin closes.
    sample_loop![sample in {
        buffer_tx.send(sample).expect("Could not send on buffer channel");
    }];

    Ok(())
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
