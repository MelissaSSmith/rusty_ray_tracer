use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, PatternAtWithInverse, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone, Debug)]
pub struct RadialGradientPattern {
    pattern_a: Box<Pattern>,
    pattern_b: Box<Pattern>
}

impl TwoColorCreate for RadialGradientPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = RadialGradientPattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b))
        };
        Pattern {
            pattern: Box::new(Patterns::RadialGradient(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl TwoPatternCreate for RadialGradientPattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let pattern = RadialGradientPattern {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b)
        };
        Pattern::create(Box::new(Patterns::RadialGradient(pattern)))
    }
}

impl PatternAtWithInverse for RadialGradientPattern {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color {
        let tp = inverse.clone() * *point;
        let distance = (tp.x().powi(2) + tp.z().powi(2)).sqrt();
        let fraction = distance - distance.floor();

        let color_a = self.pattern_a.pattern_at(&tp);
        let color_b = self.pattern_b.pattern_at(&tp) - color_a;

        color_a + color_b * fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::{PatternAt, TwoColorCreate};
    use crate::features::patterns::radial_gradient::RadialGradientPattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn gradient_both_x_and_z_interpolates_between_colors() {
        let pattern = RadialGradientPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.25, 0.0, 0.0)).equals(Color::create(0.75, 0.75, 0.75)));
        assert!(pattern.pattern_at(&Point::create(0.5, 0.0, 0.0)).equals(Color::create(0.5, 0.5, 0.5)));
        assert!(pattern.pattern_at(&Point::create(0.75, 0.0, 0.0)).equals(Color::create(0.25, 0.25, 0.25)));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.0, 0.25)).equals(Color::create(0.75, 0.75, 0.75)));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.0, 0.5)).equals(Color::create(0.5, 0.5, 0.5)));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.0, 0.75)).equals(Color::create(0.25, 0.25, 0.25)));
    }
}