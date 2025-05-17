extern crate rand;

use rand::prelude::*;
use bytemuck::{Zeroable, Pod};
use libm;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::waveform_type::WaveformType;
use crate::waveform_mode::WaveformMode;
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
    pub mode: WaveformMode,
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
    pub waveform: u16,
    pub mode: u16,
}

impl Into<WaveformParamsC> for WaveformParams {
    fn into(self) -> WaveformParamsC {
        WaveformParamsC {
            amplitude: self.amplitude,
            freq: self.freq,
            phase: self.phase,
            offset: self.offset,
            dt: self.dt,
            waveform: u16::from(self.waveform),
            mode: u16::from(self.mode),
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
            mode: WaveformMode::from(value.mode),
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl WaveformParams {
    pub fn new(dt:f32, amplitude: f32, freq: f32, phase: f32, offset: f32, waveform: WaveformType, mode: WaveformMode) -> Self {
        WaveformParams {
            dt,
            amplitude,
            freq,
            phase,
            offset,
            waveform,
            mode,
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
    rng: rand::rngs::SmallRng,
    all_leds: f32,
}

const PI: f32 = core::f32::consts::PI;

impl<const POINTS: usize> Default for Waveform<POINTS> {
    fn default() -> Self {
        Waveform {
            params: WaveformParams::default(),
            data: [0.0; POINTS],
            mask: [false; POINTS],
            points_fetched: 0,
            rng: rand::rngs::SmallRng::seed_from_u64(42),
            all_leds: 0.0,
        }
    }
}

impl<const POINTS: usize> Waveform<POINTS> {
	pub fn new(params: WaveformParams) -> Self {
        Waveform {
            params,
            data: [0.0; POINTS],
            mask: [false; POINTS],
            points_fetched: 0,
            rng: rand::rngs::SmallRng::seed_from_u64(42),
            all_leds: 0.0,
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

    /// Updates the waveform point for a given time, with a dt spacing between points. 
    /// For example, a waveform my have a t0 of 0.0 and consist of 10 points, separated by 0.1 seconds.
    /// This updates a single point in the waveform and returns the value.
    pub fn update_point(&mut self, t: f32, dt: f32, i: usize) -> f32 {
        let p = self.params;

        // Compute bounce value if needed
        self.all_leds = if self.params.waveform == WaveformType::Bounce && i == 0 {
            // This function linearly bounces between the offset and the amplitude at the rate of the frequency.
            let t = (t + (i as f32)*dt) * p.freq;
            let t = t - libm::floorf(t);
            let t = t * 2.0;
            let t = if t > 1.0 { 2.0 - t } else { t };
            p.offset + t * (p.amplitude - p.offset)
        }else{
            self.all_leds
        };

        if self.params.mode == WaveformMode::InPlace && i == 0{
            self.all_leds = match self.params.waveform {
                WaveformType::Sine => p.offset + p.amplitude*libm::sinf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase),
                WaveformType::Square => p.offset + if p.amplitude*libm::sinf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase) >= 0.0 { 1.0 } else { -1.0 },
                WaveformType::Triangle => p.offset + (2.0*p.amplitude/PI)*libm::asinf(libm::sinf(2.0*PI*p.phase * (t + (i as f32)*dt) + p.phase)),
                WaveformType::Sawtooth => p.offset + p.amplitude*libm::fmodf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase, 2.0*PI)/PI - 1.0,
                WaveformType::Noise => rand::rngs::SmallRng::random_range(&mut self.rng, p.offset..(p.offset + p.amplitude)),
                WaveformType::Bounce => self.all_leds,
            };
        }

        if i < POINTS || self.mask[i] == false {
            if self.params.mode == WaveformMode::Dynamic {
                self.data[i] = match self.params.waveform {
                    WaveformType::Sine => p.offset + p.amplitude*libm::sinf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase),
                    WaveformType::Square => p.offset + if p.amplitude*libm::sinf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase) >= 0.0 { 1.0 } else { -1.0 },
                    WaveformType::Triangle => p.offset + (2.0*p.amplitude/PI)*libm::asinf(libm::sinf(2.0*PI*p.phase * (t + (i as f32)*dt) + p.phase)),
                    WaveformType::Sawtooth => p.offset + p.amplitude*libm::fmodf(2.0*PI*p.freq * (t + (i as f32)*dt) + p.phase, 2.0*PI)/PI - 1.0,
                    WaveformType::Noise => rand::rngs::SmallRng::random_range(&mut self.rng, p.offset..(p.offset + p.amplitude)),
                    WaveformType::Bounce => self.all_leds,
                };
            }else{
                self.data[i] = self.all_leds;
            }
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
        let params = WaveformParams {
            amplitude: 1.0,
            freq: 1.0,
            phase: 0.0,
            offset: 0.0,
            dt: 0.1,
            waveform: WaveformType::Sine,
        };
        let mut waveform = Waveform::<10>::new(params);
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
        let params = WaveformParams {
            amplitude: 1.0,
            freq: 1.0,
            phase: 0.0,
            offset: 0.0,
            dt: 0.1,
            waveform: WaveformType::Square,
        };
        let mut waveform = Waveform::<10>::new(params);
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