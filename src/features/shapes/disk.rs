use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use linear_algebra::vector::Vector;
use std::f64::consts::PI;
use crate::features::intersection::Intersection;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

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
            phi_max: PI * 2.0,
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

    pub fn with_phi_max(self, phi_max: f64) -> Self {
        Self {
            phi_max,
            ..self
        }
    }

    fn area(&self) -> f64 {
        self.phi_max() * 0.5 * (self.radius().powi(2) - self.inner_radius().powi(2))
    }

    fn intersect_disk(t_hit: f64, object: &Object, ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        let p_hit = ray.position(t_hit);
        let dist_2 = p_hit.x() * p_hit.x() + p_hit.y() * p_hit.y();
        if dist_2 <= object.radius().powi(2) && dist_2 >= object.inner_radius().powi(2) {
            let mut phi = p_hit.y().atan2(p_hit.x());
            if phi < 0.0 {
                phi += 2.0 * PI;
            }
            if phi > object.phi_max() {
                return intersections
            }

            let u = phi / object.phi_max();
            let r_hit = dist_2.sqrt();
            let one_minus_v = (object.radius() / r_hit) / (object.radius() / object.inner_radius());
            let v = 1.0 - one_minus_v;
            intersections.push(Intersection::create(t_hit, object, u, v));
        }

        intersections
    }
}

impl Intersect for Disk {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        if _ray.direction().z() == 0.0 {
            return intersections;
        }
        let t_shape_hit = (_object.height() - _ray.origin().z()) / _ray.direction().z();
        if t_shape_hit > 0.0 {
            intersections.append(&mut Disk::intersect_disk(t_shape_hit, _object, _ray));
        }
        let t_shape_hit = (-_object.height() - _ray.origin().z()) / _ray.direction().z();
        if t_shape_hit <= 0.0 {
            intersections.append(&mut Disk::intersect_disk(t_shape_hit, _object, _ray));
        }

        intersections
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