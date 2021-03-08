mod utils;
mod mine_lib;

use wasm_bindgen::prelude::*;
use mine_lib::{ Minewreeper };
use rand::{ thread_rng, Rng };

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = document)]
    fn write(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let s = &format!("Hello, {}", name)[..];
    alert(s);
}

#[wasm_bindgen]
pub fn init() {
    let mut mine = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..=10 {
        let x: u8 = rng.gen_range(0..9);
        let y: u8 = rng.gen_range(0..9);
        mine.push((x, y));
    }
    let mut board = Minewreeper::init(mine);
    board.crutalmovment();
    write(&format!("{}", board)[..]);
}
