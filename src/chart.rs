use crate::{waveform::{Waveform, WaveformParams}, Dotstar};

const F_MIN: f32 = 0.0;
const F_MAX: f32 = 1.0;

struct ChartCommand {
    pub waveform: usize,
    pub params: WaveformParams,
}

#[repr(u8)]
pub enum DynamicMode {
    Stopped = 0,
    Moving = 1,
    InPlace = 2,
}

// TODO: Heapless vec of commands that users can enqueu and repeat
// TODO: Save commands to flash?

pub struct Chart<const CHARTS: usize, const POINTS: usize> {
	t: [f32; CHARTS],
    dt: [f32; CHARTS],
	waveforms: [Waveform<POINTS>; CHARTS],
    buffer: [[f32; POINTS]; CHARTS],
	mapped: [u32; POINTS],
    enabled: [bool; CHARTS],
    dt_mode: [bool; CHARTS],
}

impl<const CHARTS: usize, const POINTS: usize> Chart<CHARTS, POINTS> {
    pub fn new(waveforms: [Waveform<POINTS>; CHARTS]) -> Self {
        Chart {
            t: [0.0 as f32; CHARTS],
            dt: [1.0/POINTS as f32; CHARTS],
            waveforms,
            buffer: [[0.0; POINTS]; CHARTS],
            mapped: [0; POINTS],
            dt_mode: [true; CHARTS],
            enabled: [true; CHARTS],
        }
    }

    pub fn dynamic(&mut self, dynamic: bool, chart: usize) {
        if chart < CHARTS {
            self.dt_mode[chart] = dynamic;
        }
    }

    pub fn reset(&mut self) {
        for j in 0..CHARTS {
            self.t[j] = 0.0;
        }
    }

	pub fn update(&mut self) {
        for j in 0..CHARTS {
            for i in 0..POINTS {
                let results = self.waveforms[j].update_point(self.t[j], self.dt[j], i);
                self.buffer[j][i] = results;
            }

            if self.dt_mode[j]{
                self.t[j] += self.dt[j];
            }
        }
	}
	
	pub fn bytes(&self) -> &[u8] {
        let len = 4 * POINTS;
        let ptr = self.mapped.as_ptr() as *const u8;
        unsafe {
            core::slice::from_raw_parts(ptr, len)
        }
	}

    /// Enable or disable a chart.
    pub fn enable(&mut self, chart: usize, enable: bool) {
        if chart < CHARTS {
            self.enabled[chart] = enable;
        }
    }

    /// Map a chart to a u32 value using a closure. The maps OR'd together for the different charts,
    /// so reset should be called on the first mapping to clear the mapping buffer.
    pub fn map(&mut self, chart: usize, reset: bool, mut map_algorithm: impl FnMut(f32, usize) -> u32) {
        // Make sure chart is within bounds
        if chart >= CHARTS {
            return;
        }

        // Don't include disabled charts
        if self.enabled[chart] == false {
            return;
        }
        
        let ref_buffer = self.buffer[chart];

        for (i, v) in ref_buffer.iter().enumerate() {
            if reset {
                self.mapped[i] = 0;
            }
            let x = map_algorithm(*v, chart);
            self.mapped[i] |= x;
        }
    }

    /// Configure the 3 highest bits to 1 as per Dotstar protocol, convert to big endian
    pub fn finalize(&mut self) {
        for i in 0..POINTS {
            self.mapped[i] |= 0xF000_0000; // Set the 3 highest bits to 1 as per Dotstar protocol
            self.mapped[i] = self.mapped[i].to_be();
        }
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
pub fn to_awww(value: f32, chart: usize) -> u32 {
    let mut v = value;
    if v < F_MIN {
        v = F_MIN;
    } else if v > F_MAX {
        v = F_MAX;
    }
    let v = (v - F_MIN) / (F_MAX - F_MIN);
    let v = (v*255.0) as u8;
    let mut v = v.clamp(0, 255) as u32;
    v = v << (chart*8) as u32;
    v
}