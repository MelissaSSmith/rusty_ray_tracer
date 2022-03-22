use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{MultiColorCreate, OneColorCreate, Pattern, PatternAt, PatternAtWithInverse, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct RingPattern {
    patterns: Vec<Pattern>
}

impl TwoColorCreate for RingPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = RingPattern {
            patterns: vec![SolidPattern::create(color_a), SolidPattern::create(color_b)]
        };
        Pattern {
            pattern: Box::new(Patterns::Ring(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl MultiColorCreate for RingPattern {
    fn create(colors: Vec<Color>) -> Pattern {
        let transform = Matrix::identity();
        let patterns: Vec<Pattern> = colors.iter()
            .map(|c| SolidPattern::create(*c))
            .collect();
        let pattern = RingPattern {
            patterns
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
            patterns: vec![pattern_a, pattern_b]
        };
        Pattern::create(Box::new(Patterns::Ring(pattern)))
    }
}

impl PatternAtWithInverse for RingPattern {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color {
        let tp = *inverse* *point;
        let value = (tp.x().powi(2) + tp.z().powi(2)).sqrt();

        let index = (value.floor() as usize) % self.patterns.len();

        self.patterns[index].pattern_at(&tp)
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

        assert_eq!(pattern.pattern_at(&Point::zero()), WHITE);
        assert_eq!(pattern.pattern_at(&Point::create(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(&Point::create(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.pattern_at(&Point::create(0.708, 0.0, 0.708)), BLACK);
    }
}