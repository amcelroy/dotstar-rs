use crate::waveform::Waveform;

pub struct Chart<const CHARTS: usize, const POINTS: usize> {
	t: f32,
    dt: f32,
	waveforms: [Waveform<POINTS>; CHARTS],
	buffer: [f32; POINTS],
	rgb: [u32; POINTS],
}

impl<const CHARTS: usize, const POINTS: usize> Chart<CHARTS, POINTS> {
    pub fn new(waveforms: [Waveform<POINTS>; CHARTS]) -> Self {
        Chart {
            t: 0.0,
            dt: 0.1,
            waveforms,
            buffer: [0.0; POINTS],
            rgb: [0; POINTS],
        }
    }

	pub fn update(&mut self) {
        for i in 0..POINTS {
            let mut results = 0.0;
            for j in 0..CHARTS {
                results = self.waveforms[j].update_point(self.t, self.dt, i).unwrap_or(0.0);
            }
            self.buffer[i] = results;
        }
	}
	
	// fn RGB(&self) -> &[u32] {
        
	// }

	fn bytes(&self) -> &[u8] {
        let len = 4 * self.rgb.len();
        let ptr = self.rgb.as_ptr() as *const u8;
        unsafe {
            std::slice::from_raw_parts(ptr, len)
        }
	}
}