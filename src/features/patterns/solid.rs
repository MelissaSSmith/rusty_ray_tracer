use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::primitives::point::Point;

#[derive(Clone)]
pub struct SolidPattern {
    color: Color,
    transformation: Matrix,
    inverse_transformation: Matrix
}

impl SolidPattern {
    pub fn create(color: Color) -> SolidPattern {
        let transform = Matrix::identity();
        SolidPattern {
            color,
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl Pattern for SolidPattern {
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

    fn pattern_at(&self, _: &Point) -> Color {
        self.color
    }
}