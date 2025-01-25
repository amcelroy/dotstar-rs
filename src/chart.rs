use crate::waveform::Waveform;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u8)]
pub enum DynamicMode {
    Stopped = 0,
    Moving = 1,
    InPlace = 2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum Endian {
    Big,
    Little,
}

// TODO: Heapless vec of commands that users can enqueu and repeat
// TODO: Save commands to flash?

pub struct Chart<const WAVEFORMS: usize, const POINTS: usize> {
	t: [f32; WAVEFORMS],
	waveforms: [Waveform<POINTS>; WAVEFORMS],
    buffer: [[f32; POINTS]; WAVEFORMS],
	mapped: [u32; POINTS],
    enabled: [bool; WAVEFORMS],
}

impl<const WAVEFORMS: usize, const POINTS: usize> Chart<WAVEFORMS, POINTS> {
    pub fn new(waveforms: [Waveform<POINTS>; WAVEFORMS]) -> Self {
        Chart {
            t: [0.0_f32; WAVEFORMS],
            waveforms,
            buffer: [[0.0; POINTS]; WAVEFORMS],
            mapped: [0; POINTS],
            enabled: [true; WAVEFORMS],
        }
    }

    pub fn reset(&mut self) {
        for j in 0..WAVEFORMS {
            self.t[j] = 0.0;
        }
    }

    // Update the charts and increment time
	pub fn update(&mut self) {
        for j in 0..WAVEFORMS {
            let dt = self.waveforms[j].params().dt;
            for i in 0..POINTS {
                let results = self.waveforms[j].update_point(self.t[j], dt, i);
                self.buffer[j][i] = results;
            }

            self.t[j] += dt;
        }

        // Reset the mapped buffer
        self.clear_mapped();
	}
	
    /// Get the mapped buffer in bytes
	pub fn bytes(&self) -> &[u8] {
        let len = 4 * POINTS;
        let ptr = self.mapped.as_ptr() as *const u8;
        unsafe {
            core::slice::from_raw_parts(ptr, len)
        }
	}

    /// Clear the mapped buffer
    pub fn clear_mapped(&mut self) {
        for i in 0..POINTS {
            self.mapped[i] = 0;
        }
    }

    /// Get the mapped buffer
    pub fn mapped(&self) -> &[u32] {
        &self.mapped
    }

    /// Copy the mapped buffer to a mutable slice
    pub fn mapped_from(&self, m: &mut [u32]) {
        for (i, v) in self.mapped.iter().enumerate() {
            m[i] = *v;
        }
    }

    /// Map a chart to a u32 value using a closure. The maps OR'd together for the different charts,
    /// so reset should be called on the first mapping to clear the mapping buffer.
    pub fn map(&mut self, chart: usize, mut map_algorithm: impl FnMut(f32, usize) -> u32) {
        // Make sure chart is within bounds
        if chart >= WAVEFORMS {
            return;
        }

        // Don't include disabled charts
        if !self.enabled[chart] {
            return;
        }
        
        let ref_buffer = self.buffer[chart];

        for (i, v) in ref_buffer.iter().enumerate() {
            let x = map_algorithm(*v, chart);
            self.mapped[i] |= x;
        }
    }

    /// Configure the 3 highest bits to 1 as per Dotstar protocol, convert to big or little endian
    pub fn finalize(&mut self, endian: Endian) {
        for i in 0..POINTS {
            self.mapped[i] |= 0xF000_0000; // Set the 3 highest bits to 1 as per Dotstar protocol
            match endian {
                Endian::Big => self.mapped[i] = self.mapped[i].to_be(),
                Endian::Little => self.mapped[i] = self.mapped[i].to_le(),
            }
        }
    }
   
    /// Get a waveform by index
    pub fn get_waveform(&mut self, i: usize) -> Option<&mut Waveform<POINTS>> {
        if i < WAVEFORMS {
            Some(&mut self.waveforms[i])
        }else{
            None
        }
    }
}
