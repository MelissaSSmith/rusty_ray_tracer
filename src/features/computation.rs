use crate::features::primitives::point::Point;
use crate::features::shapes::Shape;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;

#[derive(Clone)]
pub struct Computation {
    t: f64,
    object: Box<dyn Shape>,
    point: Point,
    over_point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    inside: bool,
    reflect_vector: Vector,
    n1: f64,
    n2: f64
}

impl Computation {
    pub fn create(t: f64, object: Box<dyn Shape>) -> Computation {
        Computation {
            t,
            object,
            point: Point::zero(),
            over_point: Point::zero(),
            eye_vector: Vector::zero(),
            normal_vector: Vector::zero(),
            inside: false,
            reflect_vector: Vector::zero(),
            n1: 0.0,
            n2: 0.0
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

    pub fn over_point(&self) -> Point {
        self.over_point
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

    pub fn reflect_vector(&self) -> Vector { self.reflect_vector }

    pub fn n1(&self) -> f64 {
        self.n1
    }

    pub fn n2(&self) -> f64 {
        self.n2
    }

    pub fn set_point(&mut self, _point: Point) {
        self.point = _point;
    }

    pub fn set_over_point(&mut self, _point: Point) {
        self.over_point = _point;
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

    pub fn set_reflect_vector(&mut self, _reflect: Vector) {
        self.reflect_vector = _reflect;
    }

    pub fn set_n1(&mut self, _n1: f64) {
        self.n1 = _n1;
    }

    pub fn set_n2(&mut self, _n2: f64) {
        self.n2 = _n2;
    }
}