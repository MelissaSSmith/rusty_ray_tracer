use crate::features::point::Point;
use crate::features::shapes::Shape;
use crate::features::vector::Vector;

#[derive(Clone)]
pub struct Computation {
    t: f64,
    object: Box<dyn Shape>,
    point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    inside: bool
}

impl Computation {
    pub fn create(t: f64, object: Box<dyn Shape>) -> Computation {
        Computation {
            t,
            object,
            point: Point::create(0.0, 0.0, 0.0),
            eye_vector: Vector::create(0.0, 0.0, 0.0),
            normal_vector: Vector::create(0.0, 0.0, 0.0),
            inside: false
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(self) -> Box<dyn Shape> {
        self.object
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn eye_vector(&self) -> Vector {
        self.eye_vector
    }

    pub fn normal_vector(&self) -> Vector {
        self.normal_vector
    }

    pub fn inside(&self) -> bool {
        self.inside
    }

    pub fn set_point(&mut self, _point: Point) {
        self.point = _point;
    }

    pub fn set_eye_vector(&mut self, _vector: Vector) {
        self.eye_vector = _vector;
    }

    pub fn set_normal_vector(&mut self, _vector: Vector) {
        self.normal_vector = _vector;
    }

    pub fn set_inside(&mut self, _inside:bool) {
        self.inside = _inside;
    }
}