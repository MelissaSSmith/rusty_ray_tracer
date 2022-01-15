use std::any::Any;
use crate::features::color::Color;
use crate::features::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::point::Point;
use crate::features::shapes::Shape;

#[derive(Clone, PartialEq)]
pub struct CheckerPattern {
    color_a: Color,
    color_b: Color,
    transformation: Matrix
}

impl CheckerPattern {
    pub fn create(color_a: Color, color_b: Color) -> CheckerPattern {
        CheckerPattern{ color_a, color_b, transformation: Matrix::create_identity() }
    }
}

impl Pattern for CheckerPattern {
    fn equals(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

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
        let value = point.value().x.floor() + point.value().y.floor() + point.value().z.floor();
        if value % 2.0 == 0.0 {
            return self.color_a;
        }

        self.color_b
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::checkers::CheckerPattern;
    use crate::features::patterns::Pattern;
    use crate::features::point::Point;

    #[test]
    fn test_checkers_should_repeat_in_x() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.99, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(1.01, 0.0, 0.0)).equals(BLACK));
    }

    #[test]
    fn test_checkers_should_repeat_in_y() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.99, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 1.01, 0.0)).equals(BLACK));
    }

    #[test]
    fn test_checkers_should_repeat_in_z() {
        let pattern = CheckerPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.99)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 1.01)).equals(BLACK));
    }
}