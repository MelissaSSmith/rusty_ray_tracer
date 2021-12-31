use crate::features::intersection::Intersection;
use crate::features::point::Point;
use crate::features::ray::Ray;
use crate::features::shapes::Shape;

#[derive(Clone, Copy)]
pub(crate) struct Sphere {

}

impl Sphere {
    pub fn create() -> Sphere {
        Sphere{}
    }
}

impl Shape for Sphere {
    type Item = self::Sphere;

    fn intersect(&self, _ray: Ray) -> Vec<Intersection<Sphere>> {
        let sphere_to_ray = _ray.origin.subtract_point(Point::create(0.0,0.0,0.0));
        let a = _ray.direction.dot(_ray.direction);
        let b = 2.0 * _ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::create(t1, *self), Intersection::create(t2, *self)]
    }

    fn equals(&self, _: Sphere) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
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
}