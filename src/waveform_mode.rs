#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[repr(u8)]
pub enum WaveformMode {
    Dynamic = 0,
    InPlace = 1,
}

impl Default for WaveformMode {
    fn default() -> Self {
        WaveformMode::Dynamic
    }
}

impl From<u16> for WaveformMode {
    fn from(value: u16) -> Self {
        match value as u16 {
            0 => WaveformMode::Dynamic,
            1 => WaveformMode::InPlace,
            _ => WaveformMode::Dynamic,
        }
    }
}

impl From<WaveformMode> for u16 {
    fn from(value: WaveformMode) -> Self {
        match value {
            WaveformMode::Dynamic => 0,
            WaveformMode::InPlace => 1,
        }
    }
}