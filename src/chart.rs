use crate::{waveform::{Waveform, WaveformParams}, Dotstar};

const MAPPED_BUFFER: usize = 256;

struct ChartCommand {
    pub waveform: usize,
    pub params: WaveformParams,
}

// TODO: Heapless vec of commands that users can enqueu and repeat
// TODO: Save commands to flash?

pub struct Chart<const CHARTS: usize, const POINTS: usize> {
	t: f32,
    dt: f32,
	waveforms: [Waveform<POINTS>; CHARTS],
	buffer: [f32; POINTS],
	mapped: [u32; MAPPED_BUFFER],
    dt_mode: bool,
}

impl<const CHARTS: usize, const POINTS: usize> Chart<CHARTS, POINTS> {
    pub fn new(waveforms: [Waveform<POINTS>; CHARTS]) -> Self {
        Chart {
            t: 0.0,
            dt: 1.0/POINTS as f32,
            waveforms,
            buffer: [0.0; POINTS],
            mapped: [0; MAPPED_BUFFER],
            dt_mode: true,
        }
    }

    pub fn dynamic(&mut self, dynamic: bool) {
        self.dt_mode = dynamic;
    }

	pub fn update(&mut self) {
        for i in 0..POINTS {
            let mut results = 0.0;
            for j in 0..CHARTS {
                results = self.waveforms[j].update_point(self.t, self.dt, i).unwrap_or(0.0);
            }
            self.buffer[i] = results;
        }

        if self.dt_mode{
            self.t += self.dt;
        }
	}
	
	pub fn bytes(&self) -> &[u8] {
        let len = 4 * (POINTS + 2);
        let ptr = self.mapped.as_ptr() as *const u8;
        unsafe {
            core::slice::from_raw_parts(ptr, len)
        }
	}

    pub fn map(&mut self) {
        for (i, v) in self.buffer.iter().enumerate() {
            let mut x= to_awww(*v, -1.0, 1.0);
            self.mapped[i + 1] = x.to_be();
        }
        self.mapped[0] = 0x0000_0000;
        self.mapped[POINTS + 1] = 0xFFFF_FFFF;
    }
   
    pub fn get_waveform(&mut self, i: usize) -> Option<&mut Waveform<POINTS>> {
        if i < CHARTS {
            return Some(&mut self.waveforms[i])
        }else{
            None
        }
    }
}

impl<const CHARTS: usize, const POINTS: usize> Dotstar for Chart<CHARTS, POINTS> {
    fn generate_frame(&self) -> &[u8] {
        self.bytes()
    }
}

/// Convert a float value to Alpha, White, White, White u32 value.
/// This function also sets the 3 highest bits to 1
pub fn to_awww(value: f32, fmin: f32, fmax: f32) -> u32 {
    let mut v = value;
    if v < fmin {
        v = fmin;
    } else if v > fmax {
        v = fmax;
    }
    let v = (v - fmin) / (fmax - fmin);
    let v = (v*255.0) as u8;
    let v = v as u32;
    v | 0xF000_0000
}