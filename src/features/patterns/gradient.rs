use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct GradientPattern {
    pattern_a: Box<dyn Pattern>,
    pattern_b: Box<dyn Pattern>,
    transformation: Matrix,
    inverse_transformation: Matrix
}

impl GradientPattern {
    pub fn create(color_a: Color, color_b: Color) -> GradientPattern {
        let transform = Matrix::identity();
        GradientPattern{
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }

    pub fn create_with_patterns(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> GradientPattern {
        let transform = Matrix::identity();
        GradientPattern{
            pattern_a,
            pattern_b,
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl Pattern for GradientPattern {
    fn box_clone(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn inverse_transformation(&self) -> Matrix {
        self.inverse_transformation.clone()
    }

    fn transform(&mut self, transform: Matrix) {
        self.transformation = self.transformation.clone() * transform;
        self.inverse_transformation = self.transformation.inverse();
    }

    fn pattern_at(&self, point: &Point) -> Color {
        let tp = self.inverse_transformation() * *point;
        let color_a = self.pattern_a.pattern_at(&tp);
        let distance = self.pattern_b.pattern_at(&tp) - color_a;
        let fraction = tp.x() - tp.x().floor();

        color_a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::gradient::GradientPattern;
    use crate::features::patterns::Pattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.25, 0.0, 0.0)).equals(Color::create(0.75, 0.75, 0.75)));
        assert!(pattern.pattern_at(&Point::create(0.5, 0.0, 0.0)).equals(Color::create(0.5, 0.5, 0.5)));
        assert!(pattern.pattern_at(&Point::create(0.75, 0.0, 0.0)).equals(Color::create(0.25, 0.25, 0.25)));
    }
}