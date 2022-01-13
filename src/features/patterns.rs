use std::any::Any;
use crate::features::color::Color;
use crate::features::matrix::Matrix;
use crate::features::point::Point;
use crate::features::shapes::Shape;

pub mod stripe;

pub trait Pattern: Any {
    fn equals(&self, other: &dyn Any) -> bool;
    fn box_clone(&self) -> Box<dyn Pattern>;
    fn as_any(&self) -> &dyn Any;
    fn set_pattern_transformation(&mut self, transform: Matrix);
    fn pattern_at(&self, point: Point) -> Color;
    fn pattern_at_object(&self, object: Box<dyn Shape>, point: Point) -> Color;
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Box<dyn Pattern>) -> bool {
        self.equals(other.as_any())
    }
}

impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}