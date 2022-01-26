use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, PatternAtWithInverse, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct RingPattern {
    pattern_a: Box<Pattern>,
    pattern_b: Box<Pattern>
}

impl TwoColorCreate for RingPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = RingPattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b))
        };
        Pattern {
            pattern: Box::new(Patterns::Ring(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl TwoPatternCreate for RingPattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let pattern = RingPattern {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b)
        };
        Pattern::create(Box::new(Patterns::Ring(pattern)))
    }
}

impl PatternAtWithInverse for RingPattern {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color {
        let tp = inverse.clone() * *point;
        let value = (tp.x().powi(2) + tp.z().powi(2)).sqrt();
        if value.floor() % 2.0 == 0.0 {
            return self.pattern_a.pattern_at(&tp);
        }

        self.pattern_b.pattern_at(&tp)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::ring::RingPattern;
    use crate::features::patterns::{PatternAt, TwoColorCreate};
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