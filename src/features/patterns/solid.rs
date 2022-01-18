use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::primitives::point::Point;

#[derive(Clone)]
pub struct SolidPattern {
    color: Color,
    transformation: Matrix
}

impl SolidPattern {
    pub fn create(color: Color) -> SolidPattern {
        SolidPattern { color, transformation: Matrix::identity() }
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

    fn transform(&mut self, transform: Matrix) {
        self.transformation = self.transformation.clone() * transform;
    }

    fn pattern_at(&self, _: Point) -> Color {
        self.color
    }
}