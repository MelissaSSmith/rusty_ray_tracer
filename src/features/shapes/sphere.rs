use crate::features::intersection::Intersection;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::ray::Ray;
use crate::features::primitives::vector::Vector;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::Object;

#[derive(Clone, Copy)]
pub struct Sphere {}

impl Intersect for Sphere {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = _ray.origin - Point::create(0.0,0.0,0.0);
        let a = _ray.direction ^ _ray.direction;
        let b = 2.0 * (_ray.direction ^ sphere_to_ray);
        let c = (sphere_to_ray ^ sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - (4.0 * a * c);
        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::create(t1, _object, 0.0, 0.0), Intersection::create(t2, _object, 0.0, 0.0)]
    }
}

impl Normal for Sphere {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        *_point - Point::zero()
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::{FRAC_1_SQRT_2, PI};
    use crate::features::intersection::Intersection;
    use crate::features::material::Material;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::ray::Ray;
    use crate::features::shapes::sphere::Sphere;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::shapes::{Intersect, NormalAt};
    use crate::features::shapes::shape::{Object, Shape};

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let origin = Point::create(0.0, 0.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create();

        let intersections = Sphere::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 6.0);
    }

    #[test]
    fn test_ray_intersects_sphere_at_a_tangent() {
        let origin = Point::create(0.0, 1.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create();

        let intersections = Sphere::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
    }

    #[test]
    fn test_ray_misses_sphere() {
        let origin = Point::create(0.0, 2.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create();

        let intersections = Sphere::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_a_sphere() {
        let origin = Point::zero();
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create();

        let intersections = Sphere::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);
    }

    #[test]
    fn test_sphere_is_behind_ray() {
        let origin = Point::create(0.0, 0.0, 5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create();

        let intersections = Sphere::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[1].t, -4.0);
    }

    #[test]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let origin = Point::create(0.0, 0.0, 5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create();

        let intersections = Object::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn test_sphere_default_transformation_is_the_identity_matrix() {
        let sphere = Shape::Sphere.create();
        let identity_matrix = Matrix::identity();

        assert_eq!(sphere.transformation(), identity_matrix);
    }

    #[test]
    fn test_change_transformation_in_a_sphere() {
        let mut sphere = Shape::Sphere.create();
        let transform = Matrix::translate(2.0, 3.0, 4.0);

        sphere.set_transform(transform);

        assert_eq!(sphere.transformation(), Matrix::translate(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_intersect_a_scaled_sphere_with_a_ray() {
        let origin = Point::create(0.0, 0.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));

        let intersections = Object::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn test_intersect_a_translated_sphere_with_a_ray() {
        let origin = Point::create(0.0, 0.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(5.0, 0.0, 0.0));

        let intersections = Object::intersect(&sphere, &ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let sphere = Shape::Sphere.create();
        let intersection = Intersection::create(0.0, &sphere, 0.0, 0.0);

        let normal = Object::normal(&sphere, &Point::create(1.0, 0.0, 0.0), &intersection);

        assert_eq!(normal, Vector::create(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let sphere = Shape::Sphere.create();
        let intersection = Intersection::create(0.0, &sphere, 0.0, 0.0);

        let normal = Object::normal(&sphere, &Point::create(0.0, 1.0, 0.0), &intersection);

        assert_eq!(normal, Vector::create(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let sphere = Shape::Sphere.create();
        let intersection = Intersection::create(0.0, &sphere, 0.0, 0.0);

        let normal = Object::normal(&sphere, &Point::create(0.0, 0.0, 1.0), &intersection);

        assert_eq!(normal, Vector::create(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = Shape::Sphere.create();
        let intersection = Intersection::create(0.0, &sphere, 0.0, 0.0);

        let normal = Object::normal(&sphere, &Point::create(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0), &intersection);

        assert_eq!(normal, Vector::create(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0));
    }

    #[test]
    fn test_normal_is_a_normalized_vector() {
        let sphere = Shape::Sphere.create();
        let intersection = Intersection::create(0.0, &sphere, 0.0, 0.0);

        let normal = Object::normal(&sphere, &Point::create(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0), &intersection);

        assert_eq!(normal, normal.normalize());
    }

    #[test]
    fn test_compute_normal_on_a_translated_sphere() {
        let sphere = Shape::Sphere.create()
            .with_transform(Matrix::translate(0.0, 1.0, 0.0));
        let intersection = Intersection::create(0.0, &sphere, 0.0, 0.0);

        let normal = Object::normal(&sphere, &Point::create(0.0, 1.70711, -FRAC_1_SQRT_2), &intersection);

        assert_eq!(normal, Vector::create(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn test_compute_normal_on_a_transformed_sphere() {
        let matrix = Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate_z(PI/5.0);
        let sphere = Shape::Sphere.create().with_transform(matrix);
        let intersection = Intersection::create(0.0, &sphere, 0.0, 0.0);

        let normal = Object::normal(&sphere, &Point::create(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0), &intersection);

        assert_eq!(normal, Vector::create(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn test_sphere_has_a_default_material() {
        let sphere = Shape::Sphere.create();

        let material = Material::create();

        assert!(material.equals(sphere.material()));
    }

    #[test]
    fn test_sphere_may_be_assigned_a_material() {
        let mut sphere = Shape::Sphere.create();
        let mut material = Material::create();
        material.set_ambient(1.0);

        sphere.set_material(material.clone());

        assert!(material.equals(sphere.material()));
    }

    #[test]
    fn test_helper_for_producing_a_glass_sphere() {
        let sphere = Shape::Sphere.glass();

        assert_eq!(sphere.material().transparency(), 1.0);
        assert_eq!(sphere.material().refractive_index(), 1.5);
    }

    #[test]
    fn test_sphere_has_a_bounding_box() {
        let sphere = Shape::Sphere.create();

        assert_eq!(sphere.bounds().minimum(), Point::create(-1.0, -1.0, -1.0));
        assert_eq!(sphere.bounds().maximum(), Point::create(1.0, 1.0, 1.0));
    }
}