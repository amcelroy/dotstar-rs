use libm;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct WaveformParams {
    pub amplitude: f32,
    pub freq: f32,
    pub phase: f32,
    pub offset: f32,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl WaveformParams {
    pub fn new(amplitude: f32, freq: f32, phase: f32, offset: f32) -> Self {
        WaveformParams {
            amplitude,
            freq,
            phase,
            offset,
        }
    }

    pub fn get(&self) -> WaveformParams {
        *self
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
}

#[derive(Copy, Clone, Debug)]
pub struct Waveform<const POINTS: usize> {
    params: WaveformParams,
    data: [f32; POINTS],
    mask: [bool; POINTS],
    mask_i: usize,
    points_fetched: usize,
}

const PI: f32 = core::f32::consts::PI;

impl<const POINTS: usize> Default for Waveform<POINTS> {
    fn default() -> Self {
        Waveform {
            params: WaveformParams::default(),
            data: [0.0; POINTS],
            mask: [true; POINTS],
            mask_i: 0,
            points_fetched: 0,
        }
    }
}

impl<const POINTS: usize> Waveform<POINTS> {
	pub fn new(amplitude: f32, freq: f32, phase: f32, offset: f32) -> Self {
        Waveform {
            params: WaveformParams {
                amplitude,
                freq,
                phase,
                offset,
            },
            data: [0.0; POINTS],
            mask: [true; POINTS],
            mask_i: 0,
            points_fetched: 0,
        }
    }

    pub fn get_params(&self) -> WaveformParams {
        self.params.get()
    }

    pub fn set_params(&mut self, p: WaveformParams) {
        self.params.set(p);
    }

    pub fn update_point(&mut self, t: f32, dt: f32, i: usize) -> f32 {
        let p = self.params.get();
        if i < POINTS {
            self.data[i] = p.get_offset() + p.get_amplitude()*libm::sinf(2.0*PI*p.get_freq() * (t + (i as f32)*dt) + p.get_phase());
            self.data[i]
        }else{
            0.0
        }
	}

    pub fn update(&mut self, t: f32, dt: f32) {
        for i in 0..POINTS {
            self.data[i] = self.update_point(t, dt, i);
        }
    }

	pub fn reset(&mut self) {
		for i in 0..POINTS {
            self.mask[i] = true;
        }
        self.mask_i = 0;
        self.points_fetched = 0;
	}  		
}

mod tests {
    use super::*;

    #[test]
    fn new_waveform() {
        let waveform = Waveform::<16>::new(1.0, 1.0, 0.0,0.0);
        assert_eq!(waveform.get_params().get_amplitude(), 1.0);
        assert_eq!(waveform.get_params().get_freq(), 1.0);
        assert_eq!(waveform.get_params().get_phase(), 0.0);
        assert_eq!(waveform.get_params().get_offset(), 0.0);
    }

    #[test]
    fn new_default_waveform() {
        let waveform = Waveform::<16>::default();
        assert_eq!(waveform.get_params().get_amplitude(), 0.0);
        assert_eq!(waveform.get_params().get_freq(), 0.0);
        assert_eq!(waveform.get_params().get_phase(), 0.0);
        assert_eq!(waveform.get_params().get_offset(), 0.0);
    }   

    #[test]
    fn update() {
        let mut waveform = Waveform::<10>::new(1.0, 1.0, 0.0, 0.0);
        waveform.update(0.0, 0.1);
        assert_eq!(waveform.data[0], 0.0);
        assert_eq!(waveform.data[1], 0.58778524);
        assert_eq!(waveform.data[2], 0.95105654);
        assert_eq!(waveform.data[3], 0.9510565);
        assert_eq!(waveform.data[4], 0.5877852);
        assert_eq!(waveform.data[5], -8.742278e-8);
        assert_eq!(waveform.data[6], -0.58778554);
        assert_eq!(waveform.data[7], -0.9510565);
        assert_eq!(waveform.data[8], -0.9510565);
        assert_eq!(waveform.data[9], -0.58778495);
    }
}