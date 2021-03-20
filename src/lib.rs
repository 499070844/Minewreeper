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
    #[wasm_bindgen(js_namespace = window)]
    fn test_closure(f: &Closure<dyn FnMut()>);
    #[wasm_bindgen(js_namespace = window)]
    fn test2_closure(f: &Closure<dyn FnMut()>);
}

#[wasm_bindgen]
pub struct BoardHandle {
    cb: Closure<dyn FnMut()>,
}
#[wasm_bindgen]
impl BoardHandle {
    pub fn test(&self) {
        test2_closure(&self.cb);
    }
}

#[wasm_bindgen]
pub fn init() -> BoardHandle {
    let mut board = Minewreeper::init();
    let mut counter = 1;
    let cb = Closure::wrap(Box::new(move || {
        if counter == 1 {
            board.dig(&(4, 4));
        } else if counter == 2 {
            board.check(vec![&(0,0), &(8, 8)]);
        } else if counter == 3 {
            board.check(vec![&(0, 8), &(8, 0)]);
        }
        counter+=1;
    }) as Box<dyn FnMut()>);
    test_closure(&cb);
    return BoardHandle {
        cb: cb,
    }
    // unsafe {
        // Test: Dig a block
        // board.dig(&(4,4));
        // Test: Dig several blocks
        //board.check(vec![&(0, 0), &(8, 8)]);
    // }
}

#[wasm_bindgen(start)]
pub fn main() {
    let mut board = Minewreeper::init();
}
