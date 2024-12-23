use libm;

pub struct WaveformParams {
    amplitude: f32,
    freq: f32,
    phase: f32,
}

impl WaveformParams {
    pub fn new(amplitude: f32, freq: f32, phase: f32) -> Self {
        WaveformParams {
            amplitude,
            freq,
            phase,
        }
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

    pub fn set_params(&mut self, w: WaveformParams) {
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
}

pub struct Waveform<const POINTS: usize> {
    params: WaveformParams,
    data: [f32; POINTS],
    mask: [bool; POINTS],
    mask_i: usize,
    points_fetched: usize,
    loops: Option<u8>,
}

const PI: f32 = core::f32::consts::PI;

impl<const POINTS: usize> Waveform<POINTS> {
	pub fn default() -> Self {
		Waveform::new(1.0, 1.0, 0.0)		
	}

	pub fn new(amplitude: f32, freq: f32, phase: f32) -> Self {
        Waveform {
            params: WaveformParams {
                amplitude,
                freq,
                phase,
            },
            data: [0.0; POINTS],
            mask: [true; POINTS],
            mask_i: 0,
            points_fetched: 0,
            loops: None,
        }
    }

    pub fn set_params(&mut self, p: WaveformParams) {
        self.params.set_params(p);
    }

    pub fn get_amplitude(&self) -> f32 {
        self.params.get_amplitude()
    }

    pub fn get_freq(&self) -> f32 {
        self.params.get_freq()
    }

    pub fn get_phase(&self) -> f32 {
        self.params.get_phase()
    }

	pub fn update(&mut self, t: f32, dt: f32) {
		for i in 0..POINTS {
            self.data[i] = self.get_amplitude()*libm::sinf(2.0*PI*self.get_freq() * (t + (i as f32)*dt) + self.get_phase());
        }
	}

    pub fn update_point(&mut self, t: f32, dt: f32, i: usize) -> Option<f32> {
        if i < POINTS {
            self.data[i] = self.get_amplitude()*libm::sinf(2.0*PI*self.get_freq() * (t + (i as f32)*dt) + self.get_phase());
            Some(self.data[i])
        }else{
            None
        }
	}

	pub fn get(&self, i: usize) -> Option<f32> {
		if i < POINTS {
            // points_fetched += 1;
            // if points_fetched < loops*POINTS {
            // don’t’ mask,
            // value * mask
            // }else{
            // mask[mask_i] = 0;		
            // mask_i += 1;
            // }
            None
		}else{
			None
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
        let waveform = Waveform::<16>::new(1.0, 1.0, 0.0);
        assert_eq!(waveform.amplitude, 1.0);
        assert_eq!(waveform.freq, 1.0);
        assert_eq!(waveform.phase, 0.0);
    }

    #[test]
    fn new_default_waveform() {
        let waveform = Waveform::<16>::default();
        assert_eq!(waveform.amplitude, 1.0);
        assert_eq!(waveform.freq, 1.0);
        assert_eq!(waveform.phase, 0.0);
    }   

    #[test]
    fn update() {
        let mut waveform = Waveform::<10>::new(1.0, 1.0, 0.0);
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