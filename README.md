# earwig

A modular command line digital signal processor built in rust. Ever wanted to pipe raw audio samples around in the terminal to impress your friends at a party? Well then, you've come to the right place. `earwig` is a collection of CLI audio processors which harness the power of `stdin` and `stdout` to send samples down the signal chain. 

For example: `./choppy sample.wav | ./out`

This would cut random chunks from `sample.wav` and pipe the samples to `stdout` for `./out` to scoop up and output on the default audio device.

## Why

Mainly because once I had the idea, I couldn't unthink it... Also I wanted to become more familiar with digital signal processing, especially in `rust`. And actually I think it might be a really fun music making tool.

## Try it

1) `cargo build`

2) `./target/debug/choppy | ./target/debug/out`

## Modules

* `choppy [sample_path] [sample_size_ms]` Chop up some audio semi-randomly. Takes an audio file as input and continually chops it up in a semi-random way, pipes samples to `stdout`. Optionally takes a second arg setting the starting sample size.
* `smashy` Can't quite explain what this does, it sounds like extreme compression being applied in an unpredictable manner. Takes `stdin` sends via `sdout`.
* `out` Audio output module. Slurps up samples from `stdin` and streams them as audible audio to the nearest output device.

## Contribute

Yes please!! I don't know if two modules cam be considered a chain, so please contribute more. When doing so, bare in mind:

* one module per file in `src/`
* add a `[[bin]]` entry to `Cargo.toml` for your new module
* modules should be fun.. This includes: pointless, "really??", plain noisy, "is it working?" and funky
* lots of code comments please
* keep on the simple side of the tracks where possible, fewer dependencies and transparent logic is prefered