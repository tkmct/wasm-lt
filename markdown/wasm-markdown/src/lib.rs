#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
mod markdown;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let result = markdown::parse(input.to_string());
    result
}
