use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::patterns::Pattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple::Tuple;

#[derive(Clone)]
pub struct CheckerPattern {
    pattern_a: Box<dyn Pattern>,
    pattern_b: Box<dyn Pattern>,
    transformation: Matrix
}

impl CheckerPattern {
    pub fn create(color_a: Color, color_b: Color) -> CheckerPattern {
        CheckerPattern{
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b)),
            transformation: Matrix::identity() }
    }

    pub fn create_with_patterns(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> CheckerPattern {
        CheckerPattern{
            pattern_a,
            pattern_b,
            transformation: Matrix::identity() }
    }
}

impl Pattern for CheckerPattern {
    fn box_clone(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn transform(&mut self, transform: Matrix) {
        self.transformation = transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        let tp = self.transformation.inverse() * point;
        let value = (tp.x() + EPSILON).floor()
            + (tp.y() + EPSILON).floor()
            + (tp.z() + EPSILON).floor();
        if value % 2.0 == 0.0 {
            return self.pattern_a.pattern_at(tp);
        }

        self.pattern_b.pattern_at(tp)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::checkers::CheckerPattern;
    use crate::features::patterns::Pattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple::Tuple;

    #[test]
    fn test_checkers_should_repeat_in_x() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.99, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(1.01, 0.0, 0.0)).equals(BLACK));
    }

    #[test]
    fn test_checkers_should_repeat_in_y() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.99, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 1.01, 0.0)).equals(BLACK));
    }

    #[test]
    fn test_checkers_should_repeat_in_z() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.99)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 1.01)).equals(BLACK));
    }
}