use canvas_draw::color::Color;
use linear_algebra::point::Point;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, Patterns};

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