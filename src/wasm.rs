use core::cell::RefCell;
use spin::Mutex;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{chart::{Chart, to_awww}, waveform::{Waveform, WaveformParams}};
use lazy_static::lazy_static;

const WAVEFORMS: usize = 3;
const POINTS: usize = 32;

lazy_static!{
    static ref CHART: Mutex<Option<RefCell<Chart<WAVEFORMS, POINTS>>>> = Mutex::new(None);
}

/// Initializes the chart with default waveforms and configures the mutex.
#[wasm_bindgen]
pub fn init() {
    let waveforms = [Waveform::<POINTS>::new(0.5, 1.0, 0.0, 0.0); WAVEFORMS];

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


/// Sets a chart to be dynamic or not. Dynamic means that time is update for the waveform
/// and it will appear to move. If not dynamic, the waveform will be fixed in time.
#[wasm_bindgen]
pub fn set_dynamic(waveform: js_sys::Number, dynamic: js_sys::Boolean) {
    if let Some(g) = CHART.try_lock() {
        g.as_ref().unwrap().borrow_mut().dynamic(dynamic.as_bool().unwrap(), waveform.as_f64().unwrap() as usize);
    }
}

#[wasm_bindgen]
pub fn set_dt(waveform: js_sys::Number, dt: js_sys::Number) {
    if let Some(g) = CHART.try_lock() {
        g.as_ref().unwrap().borrow_mut().set_dt(dt.as_f64().unwrap() as f32, waveform.as_f64().unwrap() as usize);
    }
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
        g.as_ref().unwrap().borrow_mut().clear_mapped();
        g.as_ref().unwrap().borrow_mut().map(0, to_awww); // Map channels to colors
        g.as_ref().unwrap().borrow_mut().map(1, to_awww);
        g.as_ref().unwrap().borrow_mut().map(2, to_awww);
        g.as_ref().unwrap().borrow_mut().finalize()
    }
}

