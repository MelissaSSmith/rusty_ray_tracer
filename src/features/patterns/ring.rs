use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{OneColorCreate, Pattern, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct RingPattern {
    pattern_a: Pattern,
    pattern_b: Pattern
}

impl TwoColorCreate for RingPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = RingPattern {
            pattern_a: SolidPattern::create(color_a),
            pattern_b: SolidPattern::create(color_b)
        };
        Pattern {
            pattern: Patterns::Ring(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl TwoPatternCreate for RingPattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let transform = Matrix::identity();
        let pattern = RingPattern {
            pattern_a,
            pattern_b
        };
        Pattern {
            pattern: Patterns::Ring(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::ring::RingPattern;
    use crate::features::patterns::TwoColorCreate;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_ring_should_extend_both_x_and_z() {
        let pattern = RingPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(1.0, 0.0, 0.0)).equals(BLACK));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.0, 1.0)).equals(BLACK));
        assert!(pattern.pattern_at(&Point::create(0.708, 0.0, 0.708)).equals(BLACK));
    }
}