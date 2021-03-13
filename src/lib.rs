mod model;
mod utils;
mod controller;

use model::{ Minewreeper, Model };
use rand::{ thread_rng, Rng };
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = document)]
    pub fn write(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let s = &format!("Hello, {}", name)[..];
    unsafe {
        alert(s);
    }
}

#[wasm_bindgen]
pub fn init() {
    let mut board = Minewreeper::init();
    board.turn_neighbor(&(4, 4));
    unsafe {
        write(&format!("{}", board)[..]);
    }
}

pub fn render(str: &str) {
    write(str)
}