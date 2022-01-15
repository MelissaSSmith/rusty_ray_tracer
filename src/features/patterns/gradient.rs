use std::any::Any;
use crate::features::color::Color;
use crate::features::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::point::Point;
use crate::features::shapes::Shape;

#[derive(Clone, PartialEq)]
pub struct GradientPattern {
    color_a: Color,
    color_b: Color,
    transformation: Matrix
}

impl GradientPattern {
    pub fn create(color_a: Color, color_b: Color) -> GradientPattern {
        GradientPattern{ color_a, color_b, transformation: Matrix::create_identity() }
    }
}

impl Pattern for GradientPattern {
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
        let distance = self.color_b.subtract(self.color_a);
        let fraction = point.value().x - point.value().x.floor();

        self.color_a.add(distance.multiply(fraction))
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::gradient::GradientPattern;
    use crate::features::patterns::Pattern;
    use crate::features::point::Point;

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.25, 0.0, 0.0)).equals(Color::create(0.75, 0.75, 0.75)));
        assert!(pattern.pattern_at(Point::create(0.5, 0.0, 0.0)).equals(Color::create(0.5, 0.5, 0.5)));
        assert!(pattern.pattern_at(Point::create(0.75, 0.0, 0.0)).equals(Color::create(0.25, 0.25, 0.25)));
    }
}