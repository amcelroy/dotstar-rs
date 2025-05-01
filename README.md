# Dotstar for Rust

This library is a Rust implementation for the Dotstar LED strip from Adafruit. 

TODO: Add LED type

## Overview
The Dotstar LED strips are a series of individually addressable RGB LEDs that can be controlled using a data and clock line. This library provides an easy-to-use interface for controlling these LEDs from Rust using the concept of waveforms. A waveform is a sequence of points (# of LEDs) that are updated for a given time period. 

### Time and dt
Since there are multiple LEDs, time represents the 0th LED, and subsequent LEDs are separated by a fixed time, `dt`. 

For example, if there are 30 LEDs and you want a 1Hz sinusoid to span the length of the strip `dt` would be `1/30`.
```rust
let dt = 1 / 30.;
let mut waveform = Waveform::<30>::new(dt, _, _, _, _, _);
```

Changing the frequency of the waveform and keeping `dt` the same would show 2 peaks and 2 troughs in the strip.

### Waveform Amplitude to Optical Brightness
The library uses floating point may to calculate and scale the intensity of the light. -1.0 is off and 1.0 is full brightness. A combination of amplitude and offset can be used to adjust the brightness of the LEDs. Values outside of the range of -1.0 to 1.0 will be clamped to that range.

### Charts
Multiple waveforms can be combined into a chart. For example, a waveform for R, G, and B, can be combined into a chart which keeps track of the state for each LED.

```rust
const LEDS: usize = 30;
const WAVEFORMS: usize = 3;
let dt = 1 / 30.;
let mut chart = Chart::<WAVEFORMS, LEDS>::new(
    Waveform::<LEDS>::new(dt, _, _, _, _, _),
    Waveform::<LEDS>::new(dt, _, _, _, _, _),
    Waveform::<LEDS>::new(dt, _, _, _, _, _),
);
```

### WASM
This library is designed to work with WebAssembly (WASM) for web simulation purposes. This can be enabled using `make wasm` which will build the library in the `./pkg` directory, thanks to `wasm-pack`. 

