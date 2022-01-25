use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct GradientPattern {
    pattern_a: Pattern,
    pattern_b: Pattern
}

impl TwoColorCreate for GradientPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = GradientPattern {
            pattern_a: SolidPattern::create(color_a),
            pattern_b: SolidPattern::create(color_b)
        };
        Pattern {
            pattern: Patterns::Gradient(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl TwoPatternCreate for GradientPattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let transform = Matrix::identity();
        let pattern = GradientPattern {
            pattern_a,
            pattern_b
        };
        Pattern {
            pattern: Patterns::Gradient(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl PatternAt for GradientPattern {
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
    use crate::features::patterns::{PatternAt, TwoColorCreate};
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