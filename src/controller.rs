use crate::{ Minewreeper, Model, render };
use wasm_bindgen::prelude::*;

pub trait Controller {
    fn renew_board(&mut self);
    fn dig(&mut self, center: &(u8, u8));
    fn check(&mut self, center: Vec<&(u8, u8)>);
}


impl Controller for Minewreeper {
    fn renew_board(&mut self) {
        self.renew();
        render(&format!("{}", self)[..])
    }

    fn dig(&mut self, center: &(u8, u8)) {
        self.turn_neighbor(center);
        render(&format!("{}", self)[..])
    }

    fn check(&mut self, center: Vec<&(u8, u8)>) {
        for point in center {
            self.turn_neighbor(point);
        }
        render(&format!("{}", self)[..])
    }
}