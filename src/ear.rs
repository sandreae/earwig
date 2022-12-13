use std::fmt::Debug;
use std::io::Write;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::Sample;

use earwig::utils::audio_device;

fn main() -> Result<(), anyhow::Error> {
    // Get the audio device and config
    let (device, config) = audio_device();
    let mut config: cpal::StreamConfig = config.into();

    // We just want one channel
    config.channels = 1;

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    // Create an input stream which listens on the default input device and writes incomming samples
    // to stdout.
    let stream = device.build_input_stream(
        &config,
        move |data, _: &_| write_input_data::<f32>(data),
        err_fn,
    )?;

    stream.play()?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn write_input_data<T>(input: &[T])
where
    T: Sample + Debug,
{
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for &sample in input.iter() {
        writeln!(lock, "{:?}", sample).expect("Can write to stdout");
    }
}
