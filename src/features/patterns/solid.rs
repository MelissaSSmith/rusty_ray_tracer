use canvas_draw::color::Color;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, Patterns};
use crate::features::primitives::point::Point;

#[derive(Clone, Debug)]
pub struct SolidPattern {
    color: Color
}

impl OneColorCreate for SolidPattern {
    fn create(color: Color) -> Pattern {
        let pattern = SolidPattern { color };
        Pattern::create(Box::new(Patterns::Solid(pattern)))
    }
}

impl PatternAt for SolidPattern {
    fn pattern_at(&self, _: &Point) -> Color {
        self.color
    }
}