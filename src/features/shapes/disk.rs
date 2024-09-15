use std::f64::consts::PI;
use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::{Object, Shape};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Disk {
    radius: f64,
    inner_radius: f64,
    height: f64,
    phi_max: f64,
    center: Point,
    normal: Vector
}

impl Disk {
    pub fn create() -> Self {
        Self {
            radius: 1.0,
            inner_radius: 0.0,
            height: 0.0,
            phi_max: 3.0,
            center: Point::zero(),
            normal: Vector::create(0.0, 0.0, -1.0)
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn inner_radius(&self) -> f64 {
        self.inner_radius
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn phi_max(&self) -> f64 {
        self.phi_max
    }

    pub fn normal(&self) -> Vector {
        self.normal
    }

    pub fn with_radius(self, radius: f64) -> Self {
        Self {
            radius,
            ..self
        }
    }

    pub fn with_inner_radius(self, inner_radius: f64) -> Self {
        Self {
            inner_radius,
            ..self
        }
    }

    pub fn with_center(self, center: Point) -> Self {
        Self {
            center,
            ..self
        }
    }

    pub fn with_height(self, height: f64) -> Self {
        Self {
            height,
            ..self
        }
    }

    fn hash_three(n: f64) -> Vector {
        let vector = Vector::create(n.sin(), (n + 1.0).sin(), (n + 2.0).sin());
        vector * Vector::create(43758.5453123,12578.1459123,19642.3490423)
    }

    fn area(&self) -> f64 {
        self.phi_max() * 0.5 * (self.radius().powi(2) - self.inner_radius().powi(2))
    }

    fn intersect_ray(&self, ray: &Ray) -> (bool, f64) {
        if ray.direction().z() == 0.0 {
            return (false, 0.0);
        }

        let t_shape_hit = (self.height - ray.direction().z()) / ray.direction().z();
        if t_shape_hit <= 0.0 {
            return (false, 0.0);
        }

        let p_hit = ray.position(t_shape_hit);
        let dist_2 = p_hit.x() * p_hit.x() + p_hit.y() * p_hit.y();
        if dist_2 > self.radius * self.radius || dist_2 < self.inner_radius * self.inner_radius {
            return (false, 0.0)
        }

        let mut phi = p_hit.y().atan2(p_hit.x());
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        if phi > self.phi_max {
            return (false, 0.0)
        }

        let u = phi / self.phi_max;
        let r_hit = dist_2.sqrt();
        let v = (self.radius / r_hit) / (self.radius / self.inner_radius);
        let dpdu = Vector::create(
            -self.phi_max * p_hit.y(),
            self.phi_max * p_hit.x(),
            0.0
        );
        let dpdv = Vector::create(p_hit.x(), p_hit.y(), 0.0) *
            (self.inner_radius - self.radius) / r_hit;
        // todo! calculate Normal3f dndu(0,0,0), dndv(0,0,0)
        let new_p_hit = Point::create(p_hit.x(), p_hit.y(), self.height);



        (true, t_shape_hit)
    }
}

impl Intersect for Disk {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        
        let (hit, t_shape_hit) = match _object.shape() {
            Shape::Disk(d) => { d.intersect_ray(_ray) }
            _ => (false, 0.0)
        };
        if !hit {
            return vec![]
        }

        vec![Intersection::create(t_shape_hit, _object, 0.0, 0.0)]
    }
}

impl Normal for Disk {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        let object_squared = _object.radius().powi(2) + _object.inner_radius().powi(2)
            + _object.height().powi(2);
        let point_squared = _point.x().powi(2) + _point.y().powi(2) + _point.z().powi(2);
        Vector::create(
            4.0 * _point.x() * (point_squared - object_squared),
            4.0 * _point.y() * (point_squared - object_squared +
                2.0 * _object.radius() * _object.radius()),
            4.0 * _point.z() * (point_squared - object_squared)
        )
    }
}