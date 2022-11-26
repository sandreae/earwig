use std::fmt::Debug;
use std::io::Write;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, StreamConfig};

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("failed to find input device");

    let mut config: StreamConfig = device
        .default_input_config()
        .expect("Failed to get default input config")
        .into();

    // We just want one channel
    config.channels = 1;

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

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
