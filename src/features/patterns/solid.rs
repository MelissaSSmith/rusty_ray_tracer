use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, Patterns};
use crate::features::primitives::point::Point;

#[derive(Clone)]
pub struct SolidPattern {
    color: Color
}

impl SolidPattern {
    pub fn create(color: Color) -> SolidPattern {
        SolidPattern {
            color
        }
    }
}

impl OneColorCreate for SolidPattern {
    fn create(color: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = SolidPattern {
            color
        };
        Pattern {
            pattern: Patterns::Solid(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl PatternAt for SolidPattern {
    fn pattern_at(&self, _: &Point) -> Color {
        self.color
    }
}