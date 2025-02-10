use canvas_draw::color::Color;
use linear_algebra::matrix::Matrix;
use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use crate::features::patterns::{OneColorCreate, Pattern, PatternAt, PatternAtWithInverse, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;

#[derive(Clone, Debug)]
pub struct GradientPattern {
    pattern_a: Box<Pattern>,
    pattern_b: Box<Pattern>
}

impl TwoColorCreate for GradientPattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let transform = Matrix::identity();
        let pattern = GradientPattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b))
        };
        Pattern {
            pattern: Box::new(Patterns::Gradient(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl TwoPatternCreate for GradientPattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let pattern = GradientPattern {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b)
        };
        Pattern::create(Box::new(Patterns::Gradient(pattern)))
    }
}

impl PatternAtWithInverse for GradientPattern {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color {
        let tp = inverse.clone() * *point;
        let color_a = self.pattern_a.pattern_at(&tp);
        let distance = self.pattern_b.pattern_at(&tp) - color_a;
        let fraction = tp.x() - tp.x().floor();

        color_a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use canvas_draw::color::Color;
    use canvas_draw::color::consts::{BLACK, WHITE};
    use linear_algebra::point::Point;
    use linear_algebra::tuple_trait::Tuple;
    use crate::features::patterns::gradient::GradientPattern;
    use crate::features::patterns::{PatternAt, TwoColorCreate};

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(0.25, 0.0, 0.0)).equals(Color::create(0.75, 0.75, 0.75)));
        assert!(pattern.pattern_at(&Point::create(0.5, 0.0, 0.0)).equals(Color::create(0.5, 0.5, 0.5)));
        assert!(pattern.pattern_at(&Point::create(0.75, 0.0, 0.0)).equals(Color::create(0.25, 0.25, 0.25)));
    }
}