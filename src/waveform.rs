use libm;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum WaveformType {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

#[derive(Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct WaveformParams {
    pub amplitude: f32,
    pub freq: f32,
    pub phase: f32,
    pub offset: f32,
    pub dt: f32,
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
        }
    }

    pub fn get(&self) -> WaveformParams {
        *self
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = dt;
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;

    }

    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq;
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase;
    }

    pub fn set_offset(&mut self, offset: f32) {
        self.offset = offset;
    }

    pub fn set(&mut self, w: WaveformParams) {
        *self = w;
    }

    pub fn get_amplitude(&self) -> f32 {
        self.amplitude
    }

    pub fn get_freq(&self) -> f32 {
        self.freq
    }

    pub fn get_phase(&self) -> f32 {
        self.phase
    }

    pub fn get_offset(&self) -> f32 {
        self.offset
    }

    pub fn get_dt(&self) -> f32 {
        self.dt
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Waveform<const POINTS: usize> {
    params: WaveformParams,
    data: [f32; POINTS],
    mask: [bool; POINTS],
    points_fetched: usize,
    waveform_type: WaveformType,
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
            },
            data: [0.0; POINTS],
            mask: [false; POINTS],
            points_fetched: 0,
            waveform_type,
        }
    }

    pub fn params(&self) -> WaveformParams {
        self.params.get()
    }

    pub fn set_params(&mut self, p: WaveformParams) {
        self.params.set(p);
    }

    pub fn waveform_type(&self) -> WaveformType {
        self.waveform_type
    }

    pub fn set_waveform_type(&mut self, waveform_type: WaveformType) {
        self.waveform_type = waveform_type;
    }

    pub fn update_point(&mut self, t: f32, dt: f32, i: usize) -> f32 {
        let p = self.params.get();
        if i < POINTS || self.mask[i] == false {
            match self.waveform_type{
                WaveformType::Sine => self.data[i] = p.get_offset() + p.get_amplitude()*libm::sinf(2.0*PI*p.get_freq() * (t + (i as f32)*dt) + p.get_phase()),
                //WaveformType::Square => self.data[i] = p.get_offset() + p.get_amplitude()*libm::sinf(2.0*PI*p.get_freq() * (t + (i as f32)*dt) + p.get_phase()).signum(),
                WaveformType::Triangle => self.data[i] = p.get_offset() + (2.0*p.get_amplitude()/PI)*libm::asinf(libm::sinf(2.0*PI*p.get_freq() * (t + (i as f32)*dt) + p.get_phase())),
                WaveformType::Sawtooth => self.data[i] = p.get_offset() + p.get_amplitude()*libm::fmodf(2.0*PI*p.get_freq() * (t + (i as f32)*dt) + p.get_phase(), 2.0*PI)/PI - 1.0,
                WaveformType::Noise => self.data[i] = p.get_offset() + p.get_amplitude()*libm::sinf(2.0*PI*p.get_freq() * (t + (i as f32)*dt) + p.get_phase()),
                _ => self.data[i] = 0.0,
            }
            self.data[i]
        }else{
            0.0
        }
	}


	pub fn reset(&mut self) {
		for i in 0..POINTS {
            self.mask[i] = false;
        }
        self.points_fetched = 0;
	}  	

    pub fn mask(&mut self, i: usize) {
        if i < POINTS {
            self.mask[i] = !self.mask[i];
        }
    }	
}

mod tests {
    use crate::waveform::{Waveform, WaveformType};

    #[test]
    fn new_waveform() {
        let waveform = Waveform::<16>::new(1.0, 1.0, 1.0, 0.0, 0.0, WaveformType::Sine);
        assert_eq!(waveform.params().get_amplitude(), 1.0);
        assert_eq!(waveform.params().get_freq(), 1.0);
        assert_eq!(waveform.params().get_phase(), 0.0);
        assert_eq!(waveform.params().get_offset(), 0.0);
    }

    #[test]
    fn new_default_waveform() {
        let waveform = Waveform::<16>::default();
        assert_eq!(waveform.params().get_amplitude(), 0.0);
        assert_eq!(waveform.params().get_freq(), 0.0);
        assert_eq!(waveform.params().get_phase(), 0.0);
        assert_eq!(waveform.params().get_offset(), 0.0);
        assert_eq!(waveform.waveform_type, WaveformType::Sine);
    }   

    // Test sinusoidal waveform
    #[test]
    fn update_sine() {
        let mut waveform = Waveform::<10>::new(0.1, 1.0, 1.0, 0.0, 0.0, WaveformType::Sine);
        // waveform
        // assert_eq!(waveform.data[0], 0.0);
        // assert_eq!(waveform.data[1], 0.58778524);
        // assert_eq!(waveform.data[2], 0.95105654);
        // assert_eq!(waveform.data[3], 0.9510565);
        // assert_eq!(waveform.data[4], 0.5877852);
        // assert_eq!(waveform.data[5], -8.742278e-8);
        // assert_eq!(waveform.data[6], -0.58778554);
        // assert_eq!(waveform.data[7], -0.9510565);
        // assert_eq!(waveform.data[8], -0.9510565);
        // assert_eq!(waveform.data[9], -0.58778495);
    }

    // Test square waveform
    #[test]
    fn update_square() {
        let mut waveform = Waveform::<10>::new(0.1, 1.0, 1.0, 0.0, 0.0, WaveformType::Square);
        // waveform.update(0.0, 0.1);
        // assert_eq!(waveform.data[0], 1.0);
        // assert_eq!(waveform.data[1], 1.0);
        // assert_eq!(waveform.data[2], 1.0);
        // assert_eq!(waveform.data[3], 1.0);
        // assert_eq!(waveform.data[4], 1.0);
        // assert_eq!(waveform.data[5], -1.0);
        // assert_eq!(waveform.data[6], -1.0);
        // assert_eq!(waveform.data[7], -1.0);
        // assert_eq!(waveform.data[8], -1.0);
        // assert_eq!(waveform.data[9], -1.0);
    }

    // Test triangle waveform
    #[test]
    fn update_triangle() {
        let mut waveform = Waveform::<20>::new(0.1, 1.0, 1.0, 0.0, 0.0, WaveformType::Triangle);
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