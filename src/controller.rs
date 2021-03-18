use crate::{ Minewreeper, Model, render };

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
        self.flip_around(center);
        render(&format!("{}", self)[..])
    }

    fn check(&mut self, center: Vec<&(u8, u8)>) {
        for point in center {
            self.flip_around(point);
        }
        render(&format!("{}", self)[..])
    }
}