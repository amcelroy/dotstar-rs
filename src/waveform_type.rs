#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Different types of waveforms that can be generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[repr(u8)]
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

impl From<u16> for WaveformType {
    fn from(value: u16) -> Self {
        match value as u16 {
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

impl From<WaveformType> for u16 {
    fn from(value: WaveformType) -> Self {
        match value {
            WaveformType::Sine => 0,
            WaveformType::Square => 1,
            WaveformType::Triangle => 2,
            WaveformType::Sawtooth => 3,
            WaveformType::Noise => 4,
            WaveformType::Bounce => 5,
        }
    }
}