use crate::features::color::Color;

pub struct Pixel {
    x: i32,
    y: i32,
    pub(crate) color: Color
}

impl Pixel {
    pub fn create(x: i32, y: i32, color: Color) -> Pixel {
        Pixel {x, y, color}
    }
}