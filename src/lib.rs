#![no_std]

#[cfg(test)]
pub extern crate std;

pub mod waveform;
pub mod chart;

//#[cfg(feature = "wasm")]
pub mod wasm;

const F_MIN: f32 = -1.0;
const F_MAX: f32 = 1.0;

#[repr(u32)]
pub enum Frames {
    StartFrame = 0x0000_0000,
    EndFrame = 0xFFFF_FFFF,
}

/// Convert a float value to Alpha, White, White, White u32 value.
/// This function also sets the 3 highest bits to 1
pub fn to_awww(value: f32, chart: usize) -> u32 {
    let mut v = value;
    v = v.clamp(F_MIN, F_MAX);
    let v = (v - F_MIN) / (F_MAX - F_MIN);
    let v = (v*255.0) as u8;
    let mut v = v.clamp(0, 255) as u32;
    v <<= (chart*8) as u32;
    v
}

#[cfg(test)]
mod tests {

}
