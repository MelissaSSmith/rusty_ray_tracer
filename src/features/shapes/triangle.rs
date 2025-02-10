use linear_algebra::f64_operations::consts::EPSILON;
use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use linear_algebra::vector::Vector;
use crate::features::intersection::Intersection;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::{Object, Shape};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    point1: Point,
    point2: Point,
    point3: Point,
    edge1: Vector,
    edge2: Vector,
    normal: Vector
}

impl Triangle {
    pub fn create(point1: Point, point2: Point, point3: Point) -> Self {
        let e1 = point2 - point1;
        let e2 = point3 - point1;
        Self {
            point1,
            point2,
            point3,
            edge1: e1,
            edge2: e2,
            normal: (e2 * e1).normalize()
        }
    }

    pub fn point1(&self) -> Point {
        self.point1
    }

    pub fn point2(&self) -> Point {
        self.point2
    }

    pub fn point3(&self) -> Point {
        self.point3
    }

    fn edge1(&self) -> Vector {
        self.edge1
    }

    fn edge2(&self) -> Vector {
        self.edge2
    }

    pub fn normal_vector(&self) -> Vector {
        self.normal
    }

    pub fn intersects(&self, _ray: &Ray) -> Vec<f64> { //todo: abstract out into trait
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
        vec![t]
    }
}

impl Intersect for Triangle {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        match _object.shape() {
            Shape::Triangle(t) => {
                let t_values = t.intersects(_ray);
                let mut intersections = Vec::<Intersection>::new();
                for t in t_values {
                    intersections.push(Intersection::create(t, _object, 0.0, 0.0));
                }
                intersections
            },
            _ => { vec![] }
        }
    }
}

impl Normal for Triangle {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        match _object.shape() {
            Shape::Triangle(t) => { t.normal_vector() },
            _ => { Vector::zero() }
        }
    }
}

#[cfg(test)]
mod tests {
    use linear_algebra::point::Point;
    use linear_algebra::tuple_trait::Tuple;
    use linear_algebra::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::shapes::shape::Shape;
    use crate::features::shapes::triangle::Triangle;

    #[test]
    fn test_create_a_triangle() {
        let triangle = Triangle::create(
            Point::create(0.0, 1.0, 0.0),
            Point::create(-1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0)
        );

        assert_eq!(triangle.point1(), Point::create(0.0, 1.0, 0.0));
        assert_eq!(triangle.point2(), Point::create(-1.0, 0.0, 0.0));
        assert_eq!(triangle.point3(), Point::create(1.0, 0.0, 0.0));
        assert_eq!(triangle.edge1(), Vector::create(-1.0, -1.0, 0.0));
        assert_eq!(triangle.edge2(), Vector::create(1.0, -1.0, 0.0));
        assert_eq!(triangle.normal_vector(), Vector::create(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_triangle_has_a_bounding_box() {
        let triangle = Triangle::create(
            Point::create(-3.0, 7.0, 2.0),
            Point::create(6.0, 2.0, -4.0),
            Point::create(2.0, -1.0, -1.0)
        );
        let object = Shape::Triangle(triangle).create();

        let bounds = object.bounds();

        assert_eq!(bounds.minimum(), Point::create(-3.0, -1.0, -4.0));
        assert_eq!(bounds.maximum(), Point::create(6.0, 7.0, 2.0));
    }

    #[test]
    fn test_finding_the_normal_on_a_triangle() {
        let triangle = Triangle::create(
            Point::create(0.0, 1.0, 0.0),
            Point::create(-1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0)
        );
        let object = Shape::Triangle(triangle).create();

        let n1 = Triangle::normal(&object, &Point::create(0.0, 0.5, 0.0));
        let n2 = Triangle::normal(&object, &Point::create(-0.5, 0.75, 0.0));
        let n3 = Triangle::normal(&object, &Point::create(0.5, 0.25, 0.0));

        assert_eq!(triangle.normal_vector(), n1);
        assert_eq!(triangle.normal_vector(), n2);
        assert_eq!(triangle.normal_vector(), n3);
    }

    #[test]
    fn test_intersecting_a_ray_parallel_to_the_triangle() {
        let triangle = Triangle::create(
            Point::create(0.0, 1.0, 0.0),
            Point::create(-1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0)
        );
        let object = Shape::Triangle(triangle).create();
        let ray = Ray::create(Point::create(0.0, -1.0, -2.0), Vector::create(0.0, 1.0, 0.0));

        let intersections = Triangle::intersect(&object, &ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_ray_misses_the_point1_to_point3_edge() {
        let triangle = Triangle::create(
            Point::create(0.0, 1.0, 0.0),
            Point::create(-1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0)
        );
        let object = Shape::Triangle(triangle).create();
        let ray = Ray::create(Point::create(1.0, 1.0, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Triangle::intersect(&object, &ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_ray_misses_the_point1_to_point2_edge() {
        let triangle = Triangle::create(
            Point::create(0.0, 1.0, 0.0),
            Point::create(-1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0)
        );
        let object = Shape::Triangle(triangle).create();
        let ray = Ray::create(Point::create(-1.0, 1.0, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Triangle::intersect(&object, &ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_ray_misses_the_point2_to_point3_edge() {
        let triangle = Triangle::create(
            Point::create(0.0, 1.0, 0.0),
            Point::create(-1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0)
        );
        let object = Shape::Triangle(triangle).create();
        let ray = Ray::create(Point::create(0.0, -1.0, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Triangle::intersect(&object, &ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_ray_strikes_a_triangle() {
        let triangle = Triangle::create(
            Point::create(0.0, 1.0, 0.0),
            Point::create(-1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0)
        );
        let object = Shape::Triangle(triangle).create();
        let ray = Ray::create(Point::create(0.0, 0.5, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Triangle::intersect(&object, &ray);

        assert_eq!(1, intersections.len());
        assert_eq!(2.0, intersections[0].t);
    }
}