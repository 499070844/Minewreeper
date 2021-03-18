mod model;
mod utils;
mod controller;

use model::{ Minewreeper, Model };
use controller::{Controller};
use rand::{ thread_rng, Rng };
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn write(str: &str);
    /// We convert the board to string then pass it to js
    #[wasm_bindgen(js_namespace = window)]
    fn render(str: &str);
}

#[wasm_bindgen]
pub fn init() {
    let mut board = Minewreeper::init();
    unsafe {
        // Test: Dig a block
        board.dig(&(4,4));
        // Test: Dig several blocks
        board.check(vec![&(0, 0), &(8, 8)]);
    }
}

#[wasm_bindgen]
pub fn dig(x: u8, y: u8) {

}
