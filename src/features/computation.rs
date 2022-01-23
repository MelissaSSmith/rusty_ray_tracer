use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::shapes::shape::Object;

#[derive(Clone)]
pub struct Computation {
    t: f64,
    object: Object,
    point: Point,
    over_point: Point,
    under_point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    inside: bool,
    reflect_vector: Vector,
    n1: f64,
    n2: f64,
    n_ratio: f64,
    cos_i: f64,
    sin2_t: f64
}

impl Computation {
    pub fn create(t: f64, object: &Object) -> Computation {
        Computation {
            t,
            object: object.clone(),
            point: Point::zero(),
            over_point: Point::zero(),
            under_point: Point::zero(),
            eye_vector: Vector::zero(),
            normal_vector: Vector::zero(),
            inside: false,
            reflect_vector: Vector::zero(),
            n1: 0.0,
            n2: 0.0,
            n_ratio: 0.0,
            cos_i: 0.0,
            sin2_t: 0.0
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(self) -> Object {
        self.object
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn over_point(&self) -> Point {
        self.over_point
    }

    pub fn under_point(&self) -> Point {
        self.under_point
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

    pub fn n_ratio(&self) -> f64 {
        self.n_ratio
    }

    pub fn cos_i(&self) -> f64 {
        self.cos_i
    }

    pub fn sin2_t(&self) -> f64 {
        self.sin2_t
    }

    pub fn set_point(&mut self, _point: Point) {
        self.point = _point;
    }

    pub fn set_over_point(&mut self, _point: Point) {
        self.over_point = _point;
    }

    pub fn set_under_point(&mut self, _point: Point) {
        self.under_point = _point;
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

    pub fn set_n_ratio(&mut self, _n_ratio:f64) {
        self.n_ratio = _n_ratio;
    }

    pub fn set_cos_i(&mut self, _cos_i: f64) {
        self.cos_i = _cos_i;
    }

    pub fn set_sin2_t(&mut self, _sin2_t: f64) {
        self.sin2_t = _sin2_t;
    }
}