use std::any::Any;
use crate::features::color::Color;
use crate::features::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::point::consts::ORIGIN;
use crate::features::point::Point;

#[derive(Clone)]
pub struct RadialGradientPattern {
    pattern_a: Box<dyn Pattern>,
    pattern_b: Box<dyn Pattern>,
    transformation: Matrix
}

impl RadialGradientPattern {
    pub fn create(color_a: Color, color_b: Color) -> RadialGradientPattern {
        RadialGradientPattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b)),
            transformation: Matrix::create_identity()
        }
    }

    pub fn create_with_patterns(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> RadialGradientPattern {
        RadialGradientPattern{
            pattern_a,
            pattern_b,
            transformation: Matrix::create_identity() }
    }
}

impl Pattern for RadialGradientPattern {
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
        let new_transform = self.transformation.multiply(&transform);
        self.transformation = new_transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        let tp = self.transformation.inverse().multiply_point(point);
        let distance = (tp.value().x.powi(2) + tp.value().z.powi(2)).sqrt();
        let fraction = distance - distance.floor();

        let color_a = self.pattern_a.pattern_at(tp);
        let color_b = self.pattern_b.pattern_at(tp).subtract(color_a);

        color_a.add(color_b.multiply(fraction))
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::Pattern;
    use crate::features::patterns::radial_gradient::RadialGradientPattern;
    use crate::features::point::Point;

    #[test]
    fn gradient_both_x_and_z_interpolates_between_colors() {
        let pattern = RadialGradientPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.25, 0.0, 0.0)).equals(Color::create(0.75, 0.75, 0.75)));
        assert!(pattern.pattern_at(Point::create(0.5, 0.0, 0.0)).equals(Color::create(0.5, 0.5, 0.5)));
        assert!(pattern.pattern_at(Point::create(0.75, 0.0, 0.0)).equals(Color::create(0.25, 0.25, 0.25)));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.25)).equals(Color::create(0.75, 0.75, 0.75)));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.5)).equals(Color::create(0.5, 0.5, 0.5)));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.75)).equals(Color::create(0.25, 0.25, 0.25)));
    }
}