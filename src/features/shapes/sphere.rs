use crate::features::intersection::Intersection;
use crate::features::matrix::Matrix;
use crate::features::point::Point;
use crate::features::ray::Ray;
use crate::features::shapes::Shape;
use crate::features::vector::Vector;

#[derive(Clone)]
pub struct Sphere {
    transformation: Matrix
}

impl Sphere {
    pub fn create() -> Sphere {
        Sphere{transformation: Matrix::create_identity()}
    }

    pub fn transformation(self) -> Matrix {
        self.transformation
    }

    pub fn set_transform(&mut self, _transformation: Matrix) {
        self.transformation = _transformation;
    }
}

impl Shape for Sphere {
    type Item = self::Sphere;

    fn intersect(&self, _ray: Ray) -> Vec<Intersection<Sphere>> {
        let transformed_ray = _ray.transform(self.clone().transformation().inverse());
        let sphere_to_ray = transformed_ray.origin.subtract_point(Point::create(0.0,0.0,0.0));
        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::create(t1, self.clone()), Intersection::create(t2, self.clone())]
    }

    fn normal(&self, world_point: Point) -> Vector {
        let object_point = self.clone().transformation().inverse().multiply_point(world_point);
        let object_normal = object_point.subtract_point(Point::create(0.0, 0.0, 0.0));
        let world_normal = self.clone().transformation().inverse().transpose().multiply_vector(object_normal);
        world_normal.normalize()
    }

    fn equals(&self, _: Sphere) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};
    use crate::features::matrix::Matrix;
    use crate::features::point::Point;
    use crate::features::ray::Ray;
    use crate::features::shapes::Shape;
    use crate::features::shapes::sphere::Sphere;
    use crate::features::vector::Vector;

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let origin = Point::create(0.0, 0.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Sphere::create();

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 6.0);
    }

    #[test]
    fn test_ray_intersects_sphere_at_a_tangent() {
        let origin = Point::create(0.0, 1.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Sphere::create();

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
    }

    #[test]
    fn test_ray_misses_sphere() {
        let origin = Point::create(0.0, 2.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Sphere::create();

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_a_sphere() {
        let origin = Point::create(0.0, 0.0, 0.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Sphere::create();

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);
    }

    #[test]
    fn test_sphere_is_behind_ray() {
        let origin = Point::create(0.0, 0.0, 5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Sphere::create();

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[1].t, -4.0);
    }

    #[test]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let origin = Point::create(0.0, 0.0, 5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let sphere = Sphere::create();

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn test_sphere_default_transformation_is_the_identity_matrix() {
        let sphere = Sphere::create();
        let identity_matrix = Matrix::create_identity();

        assert!(sphere.transformation().equals(identity_matrix));
    }

    #[test]
    fn test_change_transformation_in_a_sphere() {
        let mut sphere = Sphere::create();
        let transform = Matrix::translate(2.0, 3.0, 4.0);

        sphere.set_transform(transform);

        assert!(sphere.transformation().equals(Matrix::translate(2.0, 3.0, 4.0)));
    }

    #[test]
    fn test_intersect_a_scaled_sphere_with_a_ray() {
        let origin = Point::create(0.0, 0.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let mut sphere = Sphere::create();
        sphere.set_transform(Matrix::scale(2.0, 2.0, 2.0));

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn test_intersect_a_translated_sphere_with_a_ray() {
        let origin = Point::create(0.0, 0.0, -5.0);
        let direction = Vector::create(0.0, 0.0, 1.0);
        let ray = Ray::create(origin, direction);
        let mut sphere = Sphere::create();
        sphere.set_transform(Matrix::translate(5.0, 0.0, 0.0));

        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let sphere = Sphere::create();

        let normal = sphere.normal(Point::create(1.0, 0.0, 0.0));

        assert!(normal.equals(Vector::create(1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let sphere = Sphere::create();

        let normal = sphere.normal(Point::create(0.0, 1.0, 0.0));

        assert!(normal.equals(Vector::create(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let sphere = Sphere::create();

        let normal = sphere.normal(Point::create(0.0, 0.0, 1.0));

        assert!(normal.equals(Vector::create(0.0, 0.0, 1.0)));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = Sphere::create();

        let normal = sphere.normal(Point::create(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0));

        assert!(normal.equals(Vector::create(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0)));
    }

    #[test]
    fn test_normal_is_a_normalized_vector() {
        let sphere = Sphere::create();

        let normal = sphere.normal(Point::create(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0));

        assert!(normal.equals(normal.normalize()));
    }

    #[test]
    fn test_compute_normal_on_a_translated_sphere() {
        let mut sphere = Sphere::create();
        sphere.set_transform(Matrix::translate(0.0, 1.0, 0.0));

        let normal = sphere.normal(Point::create(0.0, 1.70711, -FRAC_1_SQRT_2));

        assert!(normal.equals(Vector::create(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)));
    }

    #[test]
    fn test_compute_normal_on_a_transformed_sphere() {
        let mut sphere = Sphere::create();
        let matrix = Matrix::scale(1.0, 0.5, 1.0).multiply(&Matrix::rotate_z(PI/5.0));
        sphere.set_transform(matrix);

        let normal = sphere.normal(Point::create(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0));

        assert!(normal.equals(Vector::create(0.0, 0.97014, -0.24254)));
    }
}