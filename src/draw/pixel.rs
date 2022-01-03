use crate::features::color::Color;

pub struct Pixel {
    pub(crate) color: Color
}

impl Pixel {
    pub fn create(x: i32, y: i32, color: Color) -> Pixel {
        Pixel {color}
    }
}