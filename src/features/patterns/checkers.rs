use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::patterns::{Pattern, PatternAt, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct CheckerPattern {
    pattern_a: Patterns,
    pattern_b: Patterns
}

impl CheckerPattern {
    pub fn create(color_a: Color, color_b: Color) -> CheckerPattern {
        CheckerPattern{
            pattern_a: Patterns::Solid(SolidPattern::create(color_a)),
            pattern_b: Patterns::Solid(SolidPattern::create(color_b))
        }
    }
}

impl TwoColorCreate for CheckerPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = CheckerPattern {
            pattern_a: Patterns::Solid(SolidPattern::create(color_a)),
            pattern_b: Patterns::Solid(SolidPattern::create(color_b))
        };
        Pattern {
            pattern: Patterns::Checkers(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl TwoPatternCreate for CheckerPattern {
    fn create(pattern_a: Patterns, pattern_b: Patterns) -> Pattern {
        let transform = Matrix::identity();
        let pattern = CheckerPattern {
            pattern_a,
            pattern_b
        };
        Pattern {
            pattern: Patterns::Checkers(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl PatternAt for CheckerPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let tp = self.inverse_transformation() * *point;
        let value = (tp.x() + EPSILON).floor()
            + (tp.y() + EPSILON).floor()
            + (tp.z() + EPSILON).floor();
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
    use crate::features::patterns::PatternAt;
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