extern crate rand;

use rand::prelude::*;
use bytemuck::{Zeroable, Pod};
use libm;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Different types of waveforms that can be generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum WaveformType {
    Sine = 0, 
    Square = 1,
    Triangle = 2,
    Sawtooth = 3,
    Noise = 4,
    Bounce = 5,
}

impl Default for WaveformType {
    fn default() -> Self {
        WaveformType::Sine
    }
}

impl From<f32> for WaveformType {
    fn from(value: f32) -> Self {
        match value as u8 {
            0 => WaveformType::Sine,
            1 => WaveformType::Square,
            2 => WaveformType::Triangle,
            3 => WaveformType::Sawtooth,
            4 => WaveformType::Noise,
            5 => WaveformType::Bounce,
            _ => WaveformType::Sine,
        }
    }
}

impl From<WaveformType> for f32 {
    fn from(value: WaveformType) -> Self {
        match value {
            WaveformType::Sine => 0.0,
            WaveformType::Square => 1.0,
            WaveformType::Triangle => 2.0,
            WaveformType::Sawtooth => 3.0,
            WaveformType::Noise => 4.0,
            WaveformType::Bounce => 5.0,
        }
    }
}

/// Parameters for calculating a waveform.
#[derive(Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct WaveformParams {
    pub amplitude: f32,
    pub freq: f32,
    pub phase: f32,
    pub offset: f32,
    pub dt: f32,
    pub waveform: WaveformType,
}

// Flash compatible struct. 
#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Pod, Zeroable)]
pub struct WaveformParamsC {
    pub amplitude: f32,
    pub freq: f32,
    pub phase: f32,
    pub offset: f32,
    pub dt: f32,
    pub waveform: f32,
}

impl Into<WaveformParamsC> for WaveformParams {
    fn into(self) -> WaveformParamsC {
        WaveformParamsC {
            amplitude: self.amplitude,
            freq: self.freq,
            phase: self.phase,
            offset: self.offset,
            dt: self.dt,
            waveform: f32::from(self.waveform),
        }
    }
}

impl From<WaveformParamsC> for WaveformParams {
    fn from(value: WaveformParamsC) -> Self {
        WaveformParams {
            amplitude: value.amplitude,
            freq: value.freq,
            phase: value.phase,
            offset: value.offset,
            dt: value.dt,
            waveform: WaveformType::from(value.waveform),
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl WaveformParams {
    pub fn new(dt:f32, amplitude: f32, freq: f32, phase: f32, offset: f32) -> Self {
        WaveformParams {
            dt,
            amplitude,
            freq,
            phase,
            offset,
            waveform: WaveformType::Sine,
        }
    }
}

//// A waveform is a collection of points that represent the dotstar LED values.
#[derive(Clone, Debug)]
pub struct Waveform<const POINTS: usize> {
    params: WaveformParams,
    data: [f32; POINTS],
    mask: [bool; POINTS],
    points_fetched: usize,
    waveform_type: WaveformType,
    rng: rand::rngs::SmallRng,
}

const PI: f32 = core::f32::consts::PI;

impl<const POINTS: usize> Default for Waveform<POINTS> {
    fn default() -> Self {
        Waveform {
            params: WaveformParams::default(),
            data: [0.0; POINTS],
            mask: [false; POINTS],
            points_fetched: 0,
            waveform_type: WaveformType::Sine,
            rng: rand::rngs::SmallRng::seed_from_u64(42),
        }
    }
}

impl<const POINTS: usize> Waveform<POINTS> {
	pub fn new(dt:f32, amplitude: f32, freq: f32, phase: f32, offset: f32, waveform_type: WaveformType) -> Self {
        Waveform {
            params: WaveformParams {
                dt,
                amplitude,
                freq,
                phase,
                offset,
                waveform: waveform_type,
            },
            data: [0.0; POINTS],
            mask: [false; POINTS],
            points_fetched: 0,
            waveform_type,
            rng: rand::rngs::SmallRng::seed_from_u64(42),
        }
    }

    /// Returns the waveform parameters.
    pub fn params(&self) -> WaveformParams {
        self.params
    }

    /// Sets the waveform parameters.
    pub fn set_params(&mut self, p: WaveformParams) {
        self.params = p;
    }

    /// Gets the waveform type.
    pub fn waveform_type(&self) -> WaveformType {
        self.waveform_type
    }

    /// Sets the waveform type.s
    pub fn set_waveform_type(&mut self, waveform_type: WaveformType) {
        self.waveform_type = waveform_type;
    }

    /// Updates the waveform point for a given time, with a dt spacing between points. 
    /// For example, a waveform my have a t0 of 0.0 and consist of 10 points, separated by 0.1 seconds.
    /// This updates a single point in the waveform and returns the value.
    pub fn update_point(&mut self, t: f32, dt: f32, i: usize) -> f32 {
        let p = self.params;
        if i < POINTS || self.mask[i] == false {
            self.data[i] = match self.waveform_type{
                WaveformType::Sine => p.offset + p.amplitude*libm::sinf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase),
                WaveformType::Square => p.offset + if p.amplitude*libm::sinf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase) >= 0.0 { 1.0 } else { -1.0 },
                WaveformType::Triangle => p.offset + (2.0*p.amplitude/PI)*libm::asinf(libm::sinf(2.0*PI*p.phase * (t + (i as f32)*dt) + p.phase)),
                WaveformType::Sawtooth => p.offset + p.amplitude*libm::fmodf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase, 2.0*PI)/PI - 1.0,
                WaveformType::Noise => rand::rngs::SmallRng::random_range(&mut self.rng, p.offset..p.amplitude),
                WaveformType::Bounce => {
                    // This function linearly bounces between the offset and the amplitude at the rate of the frequency.
                    let t = (t + (i as f32)*dt) * p.freq;
                    let t = t - libm::floorf(t);
                    let t = t * 2.0;
                    let t = if t > 1.0 { 2.0 - t } else { t };
                    p.offset + t * (p.amplitude - p.offset)
                }
            };
            self.data[i]
        }else{
            0.0
        }
	}

    /// Updates the waveform for a given time, with a dt spacing between points.
    pub fn update(&mut self, t: f32, dt: f32) -> &[f32; POINTS] {
        for i in 0..POINTS {
            self.update_point(t, dt, i);
        }

        &self.data
    }

    /// Resets the mask and data arrays.
	pub fn reset(&mut self) {
		for i in 0..POINTS {
            self.mask[i] = false;
            self.data[i] = 0.0;
        }
        self.points_fetched = 0;
	}  	

    /// Mask a point in the waveform. Currently not used.
    pub fn mask(&mut self, i: usize) {
        if i < POINTS {
            self.mask[i] = !self.mask[i];
        }
    }	
}

#[cfg(test)]
mod tests {
    use crate::waveform::{Waveform, WaveformType};

    const T0: f32 = 0.0;
    const DT: f32 = 0.1;

    // Test sinusoidal waveform
    #[test]
    fn update_sine() {
        let mut waveform = Waveform::<10>::new(0.1, 1.0, 1.0, 0.0, 0.0, WaveformType::Sine);
        let data = waveform.update(T0, DT);
        assert_eq!(data[0], 0.0);
        assert_eq!(data[1], 0.58778524);
        assert_eq!(data[2], 0.95105654);
        assert_eq!(data[3], 0.9510565);
        assert_eq!(data[4], 0.5877852);
        assert_eq!(data[5], -8.742278e-8);
        assert_eq!(data[6], -0.58778554);
        assert_eq!(data[7], -0.9510565);
        assert_eq!(data[8], -0.9510565);
        assert_eq!(data[9], -0.58778495);
    }

    // Test square waveform
    #[test]
    fn update_square() {
        let mut waveform = Waveform::<10>::new(0.1, 1.0, 1.0, 0.0, 0.0, WaveformType::Square);
        let data = waveform.update(T0, DT);
        assert_eq!(data[0], 1.0);
        assert_eq!(data[1], 1.0);
        assert_eq!(data[2], 1.0);
        assert_eq!(data[3], 1.0);
        assert_eq!(data[4], 1.0);
        assert_eq!(data[5], -1.0);
        assert_eq!(data[6], -1.0);
        assert_eq!(data[7], -1.0);
        assert_eq!(data[8], -1.0);
        assert_eq!(data[9], -1.0);
    }

    // Test triangle waveform
    #[test]
    fn update_triangle() {
        //let mut waveform = Waveform::<20>::new(0.1, 1.0, 1.0, 0.0, 0.0, WaveformType::Triangle);
        // waveform.update(0.0, 1.0/20.0);
        // assert_eq!(waveform.data[0], 0.0);
        // assert!((0.19..0.21).contains(&waveform.data[1]));
        // assert!((0.39..0.41).contains(&waveform.data[2]));
        // assert!((0.59..0.61).contains(&waveform.data[3]));
        // assert!((0.79..0.81).contains(&waveform.data[4]));
        // assert!((0.99..1.01).contains(&waveform.data[5]));
        // assert!((0.79..0.82).contains(&waveform.data[6]));
        // assert!((0.59..0.61).contains(&waveform.data[7]));
        // assert!((0.39..0.41).contains(&waveform.data[8]));
        // assert!((0.19..0.21).contains(&waveform.data[9]));
        // assert!((-0.01..0.01).contains(&waveform.data[10]));
        // assert!((-0.21..-0.19).contains(&waveform.data[11]));
        // assert!((-0.41..-0.39).contains(&waveform.data[12]));
        // assert!((-0.61..-0.59).contains(&waveform.data[13]));
        // assert!((-0.81..-0.79).contains(&waveform.data[14]));
        // assert!((-1.01..-0.99).contains(&waveform.data[15]));
        // assert!((-0.81..-0.79).contains(&waveform.data[16]));
        // assert!((-0.61..-0.59).contains(&waveform.data[17]));
        // assert!((-0.41..-0.39).contains(&waveform.data[18]));
        // assert!((-0.21..-0.19).contains(&waveform.data[19]));
    }

}