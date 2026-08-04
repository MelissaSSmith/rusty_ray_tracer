use crate::features::intersection::Intersection;
use linear_algebra::f64_operations::consts::EPSILON;
use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use linear_algebra::vector::Vector;
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
            phi_max: 1.0,
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

    pub fn normal_vector(&self) -> Vector {
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
}

impl Intersect for Disk {
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        let disk = match _object.shape() {
            Shape::Disk(d) => d,
            _ => return vec![]
        };

        // A ray parallel to the disk's plane never hits it.
        if _ray.direction().z().abs() < EPSILON {
            return vec![];
        }

        // Intersect the plane z = height, then keep the hit only if it falls
        // within the annulus inner_radius <= r <= radius around the centre.
        let t = (disk.height() - _ray.origin().z()) / _ray.direction().z();
        let point = _ray.position(t);
        let dx = point.x() - disk.center().x();
        let dy = point.y() - disk.center().y();
        let distance_squared = dx * dx + dy * dy;

        if distance_squared > disk.radius().powi(2) || distance_squared < disk.inner_radius().powi(2) {
            return vec![];
        }

        vec![Intersection::create(t, _object, 0.0, 0.0)]
    }
}

impl Normal for Disk {
    fn normal(_object: &Object, _point: &Point) -> Vector {
        // A disk is flat, so its normal is constant across the surface.
        match _object.shape() {
            Shape::Disk(d) => d.normal_vector(),
            _ => Vector::create(0.0, 0.0, -1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use linear_algebra::point::Point;
    use linear_algebra::tuple_trait::Tuple;
    use linear_algebra::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::disk::Disk;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_ray_perpendicular_to_disk_hits_the_centre() {
        let disk = Shape::Disk(Disk::create()).create();
        let ray = Ray::create(Point::create(0.0, 0.0, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Disk::intersect(&disk, &ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 2.0);
    }

    #[test]
    fn test_ray_parallel_to_disk_misses() {
        let disk = Shape::Disk(Disk::create()).create();
        let ray = Ray::create(Point::create(0.0, 0.0, -2.0), Vector::create(0.0, 1.0, 0.0));

        let intersections = Disk::intersect(&disk, &ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_ray_outside_the_radius_misses() {
        let disk = Shape::Disk(Disk::create()).create();
        let ray = Ray::create(Point::create(2.0, 0.0, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Disk::intersect(&disk, &ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_ray_just_inside_the_radius_hits() {
        let disk = Shape::Disk(Disk::create()).create();
        let ray = Ray::create(Point::create(0.9, 0.0, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Disk::intersect(&disk, &ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 2.0);
    }

    #[test]
    fn test_inner_radius_creates_a_hole() {
        let disk = Shape::Disk(Disk::create().with_inner_radius(0.5)).create();

        // Through the centre: inside the hole, should miss.
        let centre_ray = Ray::create(Point::create(0.0, 0.0, -2.0), Vector::create(0.0, 0.0, 1.0));
        assert_eq!(Disk::intersect(&disk, &centre_ray).len(), 0);

        // In the annulus: should hit.
        let annulus_ray = Ray::create(Point::create(0.75, 0.0, -2.0), Vector::create(0.0, 0.0, 1.0));
        assert_eq!(Disk::intersect(&disk, &annulus_ray).len(), 1);
    }

    #[test]
    fn test_height_shifts_the_disk_plane() {
        let disk = Shape::Disk(Disk::create().with_height(1.0)).create();
        let ray = Ray::create(Point::create(0.0, 0.0, -2.0), Vector::create(0.0, 0.0, 1.0));

        let intersections = Disk::intersect(&disk, &ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 3.0); // (1 - (-2)) / 1
    }

    #[test]
    fn test_disk_normal_is_constant() {
        let disk = Shape::Disk(Disk::create()).create();

        let n1 = Disk::normal(&disk, &Point::create(0.0, 0.0, 0.0));
        let n2 = Disk::normal(&disk, &Point::create(0.5, 0.5, 0.0));

        assert_eq!(n1, Vector::create(0.0, 0.0, -1.0));
        assert_eq!(n2, Vector::create(0.0, 0.0, -1.0));
    }
}