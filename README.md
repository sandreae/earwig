# earwig

A modular command line digital signal processor built in rust. Ever wanted to pipe raw audio samples around in the terminal to impress your friends at a party? Well then, you've come to the right place. `earwig` is a collection of CLI audio processors which harness the power of `stdin` and `stdout` to send samples down the signal chain. 

For example: `./in sample.wav | ./choppy | ./out`

## Why

Mainly because once I had the idea, I couldn't unthink it... Also I wanted to become more familiar with digital signal processing, especially in `rust`. And actually I think it might be a really fun music making tool.

## Try it

1) `cargo build --release`

2) `./target/release/in | ./target/release/choppy | ./target/release/out`

## Modules

### Input

* `ear` Listens on a default input device and pipes every sample represented as an amplitude between -1 and 1 to `stdout`.
* `in [path/to/file.wav]` Reads an audio file from disk and pipes every sample represented as an amplitude between -1 and 1 to `stdout`.
* `silly [OPTIONS] [F] [WAVE]` Oscillator... Silly... Get it?

### Output

* `out` Audio output module. Slurps up samples from `stdin` and streams them as audible audio to the nearest output device.
* `imaginary [--width] [--height] [path]` PNG output module. Slurps up samples from `stdin` and creates a png in which several amplitudes are represented by their maximum to fit the given width in pixel.

### Transformation

* `choppy [chunk_size_ms] [max_buffer_len_secs]` Chop up some audio semi-randomly. Fills a fixed length buffer with samples recieved from `stdin` then continually chops it up in a semi-random way, pipes chopped audio samples to `stdout`. All args optional.
* `clippy [threshold] [alignment] [alignment]` It looks like you are trying to clip samples to an absolute value below the threshold amplitude. Unless clippy is chaotic, in which case it does the opposite. Oh, and if clippy is evil, it eats samples rather than fixing them.
* `extremely [t]` Linearly interpolate each sample toward +/-1, `t` between zero and one determines how much to prioritize the extreme value.
* `hungry [threshold]` Eats up any sample below the threshold amplitude.
* `randomly [t]` Linearly interpolate each sample toward random values, `t` between zero and one determines how much to prioritize the random value.
* `reversi [ms]` Reverses the order of samples within the passed time period.
* `smashy` Can't quite explain what this does, it sounds like extreme compression being applied in an unpredictable manner. Takes `stdin` sends via `sdout`.
* `smoothy [n]` Average each sample with the last `n` samples.
* `smiley [manic|cynic]` Constrains samples to stay positive. Results in very mild distortion.
* `speedy [x]` Changes the speed by factor `x`. For example, `speedy 0.5` halves the speed.
* `wobbly [x]` Modulates speed against a control signal, which makes everything sound.... wobbly.... `wobbly 0.5` generates an LFO at frequency 0.5 and modulates the passed samples accordingly.

<!-- ---

#### `previous module`

bla

#### `extremely`

```
Linearly interpolate each sample toward +/-1

Usage: extremely [T]

Arguments:
[T]  How strongly to interpolate samples toward +/- 1 [default: 1]
```

#### `next module`

blubb -->


## Contribute

Yes please!! When doing so, bare in mind:

* one module per file in `src/`
* add a `[[bin]]` entry to `Cargo.toml` for your new module
* add a line to https://github.com/sandreae/earwig#modules describing the module a little
* modules should be fun.. This includes: pointless, "really??", plain noisy, "is it working?" and funky
* lots of code comments please
* keep on the simple side of the tracks where possible, fewer dependencies and transparent logic is prefered
