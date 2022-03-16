use crate::features::intersection::Intersection;
use crate::features::primitives::operations::consts::EPSILON;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, NormalAt};
use crate::features::shapes::shape::{Object, Shape};

#[derive(Clone, Copy)]
pub struct SmoothTriangle {
    point1: Point,
    point2: Point,
    point3: Point,
    normal1: Vector,
    normal2: Vector,
    normal3: Vector,
    edge1: Vector,
    edge2: Vector,
}

impl SmoothTriangle {
    pub fn create(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> SmoothTriangle {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        SmoothTriangle {
            point1: p1,
            point2: p2,
            point3: p3,
            normal1: n1,
            normal2: n2,
            normal3: n3,
            edge1: e1,
            edge2: e2,
        }
    }

    pub(crate) fn point1(&self) -> Point {
        self.point1
    }

    pub(crate) fn point2(&self) -> Point {
        self.point2
    }

    pub(crate) fn point3(&self) -> Point {
        self.point3
    }

    pub(crate) fn normal1(&self) -> Vector {
        self.normal1
    }

    pub(crate) fn normal2(&self) -> Vector {
        self.normal2
    }

    pub(crate) fn normal3(&self) -> Vector {
        self.normal3
    }

    fn edge1(&self) -> Vector {
        self.edge1
    }

    fn edge2(&self) -> Vector {
        self.edge2
    }

    fn intersects(&self, _ray: &Ray) -> Vec<(f64, f64, f64)> { //todo: abstract out into trait
        let dir_cross_edge2 = _ray.direction() * self.edge2();
        let determinant = self.edge1() ^ dir_cross_edge2;
        if determinant.abs() < EPSILON {
            return vec![];
        }
        let f = 1.0 / determinant;
        let point1_to_origin = _ray.origin() - self.point1();
        let u = f * (point1_to_origin ^ dir_cross_edge2);
        if u < 0.0 || u > 1.0 {
            return vec![];
        }
        let origin_cross_edge1 = point1_to_origin * self.edge1();
        let v = f * (_ray.direction() ^ origin_cross_edge1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }
        let t = f * (self.edge2() ^ origin_cross_edge1);
        vec![(t, u, v)]
    }
}

impl Intersect for SmoothTriangle {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        match _object.shape() {
            Shape::SmoothTriangle(t) => {
                let t_values = t.intersects(_ray);
                let mut intersections = Vec::<Intersection>::new();
                for (t, u, v) in t_values {
                    intersections.push(Intersection::create(t, _object, u, v));
                }
                intersections
            },
            _ => { vec![] }
        }
    }
}

impl NormalAt for SmoothTriangle {
    fn normal(_object: &Object, _point: &Point, _hit: &Intersection) -> Vector {
        match _object.shape() {
            Shape::SmoothTriangle(t) => {
                t.normal2() * _hit.u + t.normal3() * _hit.v + t.normal1() * (1.0 - _hit.u - _hit.v)
            },
            _ => { Vector::zero() }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::intersection::Intersection;
    use crate::features::primitives::operations::Operations;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::{Intersect, NormalAt};
    use crate::features::shapes::shape::{Object, Shape};
    use crate::features::shapes::smooth_triangle::SmoothTriangle;

    #[test]
    fn test_constructing_a_smooth_triangle() {
        let p1 = Point::create(0.0, 1.0, 2.0);
        let p2 = Point::create(3.0, 4.0, 5.0);
        let p3 = Point::create(6.0, 7.0, 8.0);
        let n1 = Vector::create(0.0, 1.0, 2.0);
        let n2 = Vector::create(3.0, 4.0, 5.0);
        let n3 = Vector::create(6.0, 7.0, 8.0);

        let triangle = SmoothTriangle::create(p1, p2, p3, n1, n2, n3);

        assert!(triangle.point1().equals(p1));
        assert!(triangle.point2().equals(p2));
        assert!(triangle.point3().equals(p3));
        assert!(triangle.normal1().equals(n1));
        assert!(triangle.normal2().equals(n2));
        assert!(triangle.normal3().equals(n3));
    }

    #[test]
    fn test_intersection_with_a_smooth_triangle_stores_u_v() {
        let p1 = Point::create(0.0, 1.0, 0.0);
        let p2 = Point::create(-1.0, 0.0, 0.0);
        let p3 = Point::create(1.0, 0.0, 0.0);
        let n1 = Vector::create(0.0, 1.0, 0.0);
        let n2 = Vector::create(-1.0, 0.0, 0.0);
        let n3 = Vector::create(1.0, 0.0, 0.0);
        let triangle = SmoothTriangle::create(p1, p2, p3, n1, n2, n3);
        let shape = Shape::SmoothTriangle(triangle).create();
        let ray = Ray::create(Point::create(-0.2, 0.3, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = SmoothTriangle::intersect(&shape, &ray);

        assert!(intersections[0].u.equals(0.45));
        assert!(intersections[0].v.equals(0.25));
    }

    #[test]
    fn test_smooth_triangle_uses_u_v_to_interpolate_the_normal() {
        let p1 = Point::create(0.0, 1.0, 0.0);
        let p2 = Point::create(-1.0, 0.0, 0.0);
        let p3 = Point::create(1.0, 0.0, 0.0);
        let n1 = Vector::create(0.0, 1.0, 0.0);
        let n2 = Vector::create(-1.0, 0.0, 0.0);
        let n3 = Vector::create(1.0, 0.0, 0.0);
        let triangle = SmoothTriangle::create(p1, p2, p3, n1, n2, n3);
        let shape = Shape::SmoothTriangle(triangle).create();
        let intersection = Intersection::create(1.0, &shape, 0.45, 0.25);
        let ray = Ray::create(Point::create(-0.2, 0.3, -2.0), Vector::create(0.0, 0.0, 1.0));

        let normal = Object::normal(&shape, &Point::zero(), &intersection);

        assert!(normal.equals(Vector::create(-0.5547, 0.83205, 0.0)));
    }
}