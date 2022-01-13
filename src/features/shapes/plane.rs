use std::any::Any;
use crate::features::intersection::Intersection;
use crate::features::material::Material;
use crate::features::matrix::Matrix;
use crate::features::operations::consts::EPSILON;
use crate::features::point::Point;
use crate::features::ray::Ray;
use crate::features::shapes::Shape;
use crate::features::vector::Vector;

#[derive(Clone, PartialEq)]
pub struct Plane {
    transformation: Matrix,
    material: Material
}

impl Plane {
    pub fn create() -> Plane {
        Plane {
            transformation: Matrix::create_identity(),
            material: Material::create()
        }
    }
}

impl Shape for Plane {
    fn equals(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_transform(&mut self, _transformation: Matrix) {
        self.transformation = _transformation;
    }

    fn set_material(&mut self, _material: Material) {
        self.material = _material;
    }

    fn normal(&self, _point: Point) -> Vector {
        Vector::create(0.0, 1.0, 0.0)
    }

    fn intersect(&self, _ray: Ray) -> Vec<Intersection> {
        if _ray.direction.value().y.abs() < EPSILON {
            return vec![]
        }
        let t = -_ray.origin.value().y / _ray.direction.value().y;
        vec![Intersection::create(t, Box::new(self.clone()))]
    }
}

#[cfg(test)]
mod tests {
    use crate::features::point::Point;
    use crate::features::ray::Ray;
    use crate::features::shapes::plane::Plane;
    use crate::features::shapes::Shape;
    use crate::features::vector::Vector;

    #[test]
    fn test_normal_of_a_plane_is_constant_everywhere() {
        let plane = Plane::create();

        let n1 = plane.normal(Point::create(0.0, 0.0, 0.0));
        let n2 = plane.normal(Point::create(10.0, 0.0, -10.0));
        let n3 = plane.normal(Point::create(-5.0, 0.0, 150.0));

        assert!(n1.equals(Vector::create(0.0, 1.0, 0.0)));
        assert!(n2.equals(Vector::create(0.0, 1.0, 0.0)));
        assert!(n3.equals(Vector::create(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_intersect_with_a_ray_parallel_to_the_plane() {
        let plane = Plane::create();
        let ray = Ray::create(Point::create(0.0, 10.0, 0.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = plane.intersect(ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_intersect_with_a_coplanar_ray() {
        let plane = Plane::create();
        let ray = Ray::create(Point::create(0.0, 0.0, 0.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = plane.intersect(ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn test_ray_intersecting_a_plane_from_above() {
        let plane = Plane::create();
        let ray = Ray::create(Point::create(0.0, 1.0, 0.0), Vector::create(0.0, -1.0, 0.0));

        let intersections = plane.intersect(ray);

        assert_eq!(1, intersections.len());
        assert_eq!(1.0, intersections[0].t);
    }

    #[test]
    fn test_ray_intersection_a_plane_from_below() {
        let plane = Plane::create();
        let ray = Ray::create(Point::create(0.0, -1.0, 0.0), Vector::create(0.0, 1.0, 0.0));

        let intersections = plane.intersect(ray);

        assert_eq!(1, intersections.len());
        assert_eq!(1.0, intersections[0].t);
    }
}