use crate::{ Minewreeper };

pub trait Controller {
    fn init() -> bool;
}

impl Controller for Minewreeper {}