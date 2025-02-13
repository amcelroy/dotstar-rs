use core::cell::RefCell;
use spin::Mutex;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{chart::Chart, waveform::{Waveform, WaveformParams, WaveformType}, to_awww};
use lazy_static::lazy_static;

// Change for more colors or LEDS, don't forget to change the mapping function
// when calling map, it uses 
const WAVEFORMS: usize = 3; 
// Change to match your dotstar LED count
const POINTS: usize = 32;

const DT: f32 = 1.0 / POINTS as f32;

lazy_static!{
    static ref CHART: Mutex<Option<RefCell<Chart<WAVEFORMS, POINTS>>>> = Mutex::new(None);
}

/// Initializes the chart with default waveforms and configures the mutex.
#[wasm_bindgen]
pub fn init() {
    let waveforms = [
        Waveform::<POINTS>::new(DT, 0.5, 1.0, 0.0, 0.0, WaveformType::Sine), 
        Waveform::<POINTS>::new(DT, 0.5, 1.0, 1.0, 0.0, WaveformType::Sine),
        Waveform::<POINTS>::new(DT, 0.5, 1.0, 1.5, 0.0, WaveformType::Sine),
    ];

    let chart = Chart::new(waveforms);
    if let Some(mut g) = CHART.try_lock() {
        *g = Some(RefCell::new(chart));
    }
}

/// Returns a u32 array that represents ARGB values for the pixels on the Dotstar strip.
#[wasm_bindgen]
pub fn argb_array() -> js_sys::Uint32Array {
    let mut x = [0; POINTS];
    if let Some(g) = CHART.try_lock() {
        g.as_ref().unwrap().borrow().mapped_from(&mut x);
    }
    return js_sys::Uint32Array::from(&x[..]);
}

/// Update a waveforms
#[wasm_bindgen]
pub fn update_waveform(waveform: js_sys::Number, params: WaveformParams) {
    if let Some(g) = CHART.try_lock() {
       g.as_ref().unwrap().borrow_mut().get_waveform(waveform.as_f64().unwrap() as usize).unwrap().set_params(params);
    } 
}

/// Update the chart in time
#[wasm_bindgen]
pub fn tick() {
    if let Some(g) = CHART.try_lock() {
        g.as_ref().unwrap().borrow_mut().update();
        g.as_ref().unwrap().borrow_mut().map(0, to_awww); // Map channels to colors
        g.as_ref().unwrap().borrow_mut().map(1, to_awww);
        g.as_ref().unwrap().borrow_mut().map(2, to_awww);
        g.as_ref().unwrap().borrow_mut().finalize(crate::chart::Endian::Little)
    }
}

#[wasm_bindgen]
pub fn reset() {
    if let Some(g) = CHART.try_lock() {
       g.as_ref().unwrap().borrow_mut().reset();
    } 
}

