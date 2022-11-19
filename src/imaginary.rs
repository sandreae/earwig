#![feature(int_roundings)]

use clap::Parser;
use image::{GenericImageView, GrayImage, Luma};

use earwig::utils::{sample_loop, lerp};

/// Visualise amplitudes as a png
#[derive(Parser, Debug)]
struct Args {
   /// The width of the generated image in pixel
   #[arg(short, long, default_value_t = 2500)]
   width: u32,
   /// The height of the generated image in pixel
   #[arg(long, default_value_t = 1250)]
   height: u32,
   /// The output filename
   #[arg()]
   path: String,
}

fn amplitude_to_pixel(amplitude: f64, args: &Args) -> i32 {
    (amplitude * (args.height as f64)) as i32
}

fn main() {
    let args = Args::parse();

    let mut imgbuf = GrayImage::new(args.width, args.height);

    let horizon = args.height / 2;

    for (_x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let l = if y < horizon {0} else {255};
        *pixel = Luma([l]);
    }

    let mut buffer = Vec::new();

    sample_loop![mut sample in {
        // push the sample to buffer
        buffer.push(sample)
    }];

    // compute the number of samples that are summerised in a pixel of width
    let granularity = buffer.len().div_ceil(args.width as usize);

    for (i, samples) in buffer.chunks(granularity).enumerate() {
        let mut accumulated_max = 0.0;
        let mut accumulated_min = 0.0;

        for sample in samples {
            accumulated_max = sample.max(accumulated_max);
            accumulated_min = sample.min(accumulated_min);
        }

        for j in horizon..((horizon as i32) + amplitude_to_pixel(accumulated_max, &args)) as u32 {
            imgbuf.put_pixel(i as u32, j, Luma([0]));
        }
        for j in horizon + ((horizon as i32) + amplitude_to_pixel(accumulated_min, &args)) as u32..horizon {
            imgbuf.put_pixel(i as u32, j, Luma([255]));
        }
    }
    imgbuf.save(&args.path).unwrap();
}
