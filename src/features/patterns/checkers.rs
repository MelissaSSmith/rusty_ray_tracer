use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, PatternAtWithInverse, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct CheckerPattern {
    pattern_a: Box<Pattern>,
    pattern_b: Box<Pattern>
}

impl TwoColorCreate for CheckerPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = CheckerPattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b))
        };
        Pattern {
            pattern: Box::new(Patterns::Checkers(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl TwoPatternCreate for CheckerPattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let pattern = CheckerPattern {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b)
        };
        Pattern::create(Box::new(Patterns::Checkers(pattern)))
    }
}

impl PatternAtWithInverse for CheckerPattern {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color {
        let tp = inverse.clone() * *point;
        let value = tp.x().floor() + tp.y().floor() + tp.z().floor();
        if value % 2.0 == 0.0 {
            return self.pattern_a.pattern_at(&tp);
        }

        self.pattern_b.pattern_at(&tp)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::checkers::CheckerPattern;
    use crate::features::patterns::{PatternAt, TwoColorCreate};
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_checkers_should_repeat_in_x() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.99, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(1.01, 0.0, 0.0)).equals(BLACK));
    }

    #[test]
    fn test_checkers_should_repeat_in_y() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.99, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.0, 1.01, 0.0)).equals(BLACK));
    }

    #[test]
    fn test_checkers_should_repeat_in_z() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.0, 0.99)).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.0, 1.01)).equals(BLACK));
    }
}