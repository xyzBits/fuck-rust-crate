extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[wasm_bindgen]
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> f64 {
    a / b
}