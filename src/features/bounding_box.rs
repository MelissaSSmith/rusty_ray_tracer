use core::ops::Add;
use linear_algebra::matrix::Matrix;
use linear_algebra::point::Point;
use linear_algebra::transformations::Transform;
use linear_algebra::tuple_trait::Tuple;
use crate::features::ray::Ray;
use crate::features::shapes::shape::Object;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    minimum: Point,
    maximum: Point
}

impl BoundingBox {
    pub fn create() -> BoundingBox {
        BoundingBox {
            minimum: Point::create(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            maximum: Point::create(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
        }
    }

    pub fn minimum(&self) -> Point {
        self.minimum
    }

    pub fn maximum(&self) -> Point {
        self.maximum
    }

    pub fn with_minimum(self, minimum: Point) -> BoundingBox {
        BoundingBox {
            minimum,
            ..self
        }
    }

    pub fn with_maximum(self, maximum: Point) -> BoundingBox {
        BoundingBox {
            maximum,
            ..self
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.minimum().x() <= point.x() && point.x() <= self.maximum().x() &&
            self.minimum().y() <= point.y() && point.y() <= self.maximum().y() &&
            self.minimum().z() <= point.z() && point.z() <= self.maximum().z()
    }

    pub fn contains_box(&self, bounding_box: BoundingBox) -> bool {
        self.contains_point(bounding_box.minimum()) && self.contains_point(bounding_box.maximum())
    }

    pub fn intersects(_bounds: &BoundingBox, _ray: &Ray) -> bool {
        let (xtmin, xtmax) = Object::check_axis(&_ray.origin().x(), &_ray.direction().x(), _bounds.minimum().x(), _bounds.maximum().x());
        let (ytmin, ytmax) = Object::check_axis(&_ray.origin().y(), &_ray.direction().y(), _bounds.minimum().y(), _bounds.maximum().y());
        let (ztmin, ztmax) = Object::check_axis(&_ray.origin().z(), &_ray.direction().z(), _bounds.minimum().z(), _bounds.maximum().z());

        let tmin = vec![xtmin, ytmin, ztmin].iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let tmax = vec![xtmax, ytmax, ztmax].iter().fold(f64::INFINITY, |a, &b| a.min(b));

        if tmax < 0.0 {
            return false;
        }

        tmin <= tmax
    }

    pub(crate) fn split(&self) -> (BoundingBox, BoundingBox) {
        let dx = f64::max(self.minimum().x(), self.maximum().x());
        let dy = f64::max(self.minimum().y(), self.maximum().y());
        let dz = f64::max(self.minimum().z(), self.maximum().z());
        let greatest_value = f64::max(dx, f64::max(dy, dz));

        let (mut x0, mut y0, mut z0) = (self.minimum().x(), self.minimum().y(), self.minimum().z());
        let (mut x1, mut y1, mut z1) = (self.maximum().x(), self.maximum().y(), self.maximum().z());

        if greatest_value == dx {
            x0 = (x0 + dx) / 2.0;
            x1 = x0;
        } else if greatest_value == dy {
            y0 = (y0 + dy) / 2.0;
            y1 = y0;
        } else {
            z0 = (z0 + dz) / 2.0;
            z1 = z0;
        }

        let mid_min = Point::create(x0, y0, z0);
        let mid_max = Point::create(x1, y1, z1);

        let left = BoundingBox::create()
            .with_minimum(self.minimum())
            .with_maximum(mid_max);
        let right = BoundingBox::create()
            .with_minimum(mid_min)
            .with_maximum(self.maximum());
        (left, right)
    }
}

impl Transform<BoundingBox> for BoundingBox {
    type Output = BoundingBox;
    fn transform(self, matrix: Matrix) -> Self::Output {
        let mut points = Vec::<Point>::new();
        points.push(self.minimum());
        points.push(Point::create(self.minimum().x(), self.minimum().y(), self.maximum().z()));
        points.push(Point::create(self.minimum().x(), self.maximum().y(), self.minimum().z()));
        points.push(Point::create(self.minimum().x(), self.maximum().y(), self.maximum().z()));
        points.push(Point::create(self.maximum().x(), self.minimum().y(), self.minimum().z()));
        points.push(Point::create(self.maximum().x(), self.minimum().y(), self.maximum().z()));
        points.push(Point::create(self.maximum().x(), self.maximum().y(), self.minimum().z()));
        points.push(self.maximum());

        let mut transformed_box = BoundingBox::create();
        for point in points {
            transformed_box = transformed_box + (matrix.clone() * point);
        }

        transformed_box
    }
}

impl Add<BoundingBox> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        let min_x = f64::min(self.minimum().x(), rhs.minimum().x());
        let min_y = f64::min(self.minimum().y(), rhs.minimum().y());
        let min_z = f64::min(self.minimum().z(), rhs.minimum().z());

        let max_x = f64::max(self.maximum().x(), rhs.maximum().x());
        let max_y = f64::max(self.maximum().y(), rhs.maximum().y());
        let max_z = f64::max(self.maximum().z(), rhs.maximum().z());

        BoundingBox {
            minimum: Point::create(min_x, min_y, min_z),
            maximum: Point::create(max_x, max_y, max_z)
        }
    }
}

impl Add<Point> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Point) -> Self::Output {
        let min_x = if rhs.x() < self.minimum().x() { rhs.x() } else { self.minimum().x() };
        let min_y = if rhs.y() < self.minimum().y() { rhs.y() } else { self.minimum().y() };
        let min_z = if rhs.z() < self.minimum().z() { rhs.z() } else { self.minimum().z() };

        let max_x = if rhs.x() > self.maximum().x() { rhs.x() } else { self.maximum().x() };
        let max_y = if rhs.y() > self.maximum().y() { rhs.y() } else { self.maximum().y() };
        let max_z = if rhs.z() > self.maximum().z() { rhs.z() } else { self.maximum().z() };

        BoundingBox {
            minimum: Point::create(min_x, min_y, min_z),
            maximum: Point::create(max_x, max_y, max_z)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;
    use crate::features::bounding_box::BoundingBox;
    use linear_algebra::matrix::Matrix;
    use linear_algebra::point::Point;
    use linear_algebra::transformations::Transform;
    use linear_algebra::tuple_trait::Tuple;
    use linear_algebra::vector::Vector;
    use crate::features::ray::Ray;

    #[test]
    fn test_create_an_empty_bounding_box() {
        let bound_box = BoundingBox::create();

        assert_eq!(bound_box.minimum(), Point::create(f64::INFINITY, f64::INFINITY, f64::INFINITY));
        assert_eq!(bound_box.maximum(), Point::create(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY));
    }

    #[test]
    fn test_create_a_bounding_box_with_volume() {
        let bound_box = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -2.0, -3.0))
            .with_maximum(Point::create(3.0, 2.0, 1.0));

        assert_eq!(bound_box.minimum(), Point::create(-1.0, -2.0, -3.0));
        assert_eq!(bound_box.maximum(), Point::create(3.0, 2.0, 1.0));
    }

    #[test]
    fn test_add_points_to_an_empty_bounding_box() {
        let point_1 = Point::create(-5.0, 2.0, 0.0);
        let point_2 = Point::create(7.0, 0.0, -3.0);

        let bound_box = BoundingBox::create() + point_1 + point_2;

        assert_eq!(bound_box.minimum(), Point::create(-5.0, 0.0, -3.0));
        assert_eq!(bound_box.maximum(), Point::create(7.0, 2.0, 0.0));
    }

    #[test]
    fn test_add_one_bounding_to_another() {
        let bound_box_1 = BoundingBox::create()
            .with_minimum(Point::create(-5.0, -2.0, 0.0))
            .with_maximum(Point::create(7.0, 4.0, 4.0));
        let bound_box_2 = BoundingBox::create()
            .with_minimum(Point::create(8.0, -7.0, -2.0))
            .with_maximum(Point::create(14.0, 2.0, 8.0));

        let bound_box = bound_box_1 + bound_box_2;

        assert_eq!(bound_box.minimum(), Point::create(-5.0, -7.0, -2.0));
        assert_eq!(bound_box.maximum(), Point::create(14.0, 4.0, 8.0));
    }

    #[test]
    fn test_check_to_see_if_a_box_contains_a_given_point() {
        let bound_box = BoundingBox::create()
            .with_minimum(Point::create(5.0, -2.0, 0.0))
            .with_maximum(Point::create(11.0, 4.0, 7.0));

        let tests = vec![
            (Point::create(5.0, -2.0, 0.0), true),
            (Point::create(11.0, 4.0, 7.0), true),
            (Point::create(8.0, 1.0, 3.0), true),
            (Point::create(3.0, 0.0, 3.0), false),
            (Point::create(8.0, -4.0, 3.0), false),
            (Point::create(8.0, 1.0, -1.0), false),
            (Point::create(13.0, 1.0, 3.0), false),
            (Point::create(8.0, 5.0, 3.0), false),
            (Point::create(8.0, 1.0, 8.0), false)
        ];

        for test in tests {
            let result = bound_box.contains_point(test.0);

            assert_eq!(result, test.1);
        }
    }

    #[test]
    fn test_check_to_see_if_a_box_contains_a_given_box() {
        let bound_box = BoundingBox::create()
            .with_minimum(Point::create(5.0, -2.0, 0.0))
            .with_maximum(Point::create(11.0, 4.0, 7.0));

        let tests = vec![
            (Point::create(5.0, -2.0, 0.0), Point::create(11.0, 4.0, 7.0), true),
            (Point::create(6.0, -1.0, 1.0), Point::create(10.0, 3.0, 6.0), true),
            (Point::create(4.0, -3.0, -1.0), Point::create(10.0, 3.0, 6.0), false),
            (Point::create(6.0, -1.0, 1.0), Point::create(12.0, 5.0, 8.0), false)
        ];

        for test in tests {
            let test_box = BoundingBox::create()
                .with_maximum(test.1)
                .with_minimum(test.0);

            let result = bound_box.contains_box(test_box);

            assert_eq!(result, test.2);
        }
    }

    #[test]
    fn test_transforming_a_bounding_box() {
        let bounds = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -1.0, -1.0))
            .with_maximum(Point::create(1.0, 1.0, 1.0));
        let matrix = Matrix::rotate_x(PI/4.0) * Matrix::rotate_y(PI/4.0);

        let new_bounds = bounds.transform(matrix);

        assert_eq!(new_bounds.minimum(), Point::create(-1.414214, -1.707107, -1.707107));
        assert_eq!(new_bounds.maximum(), Point::create(1.414214, 1.707107, 1.707107));
    }

    #[test]
    fn test_intersect_a_ray_with_a_bounding_box_at_the_origin() {
        let bounds = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -1.0, -1.0))
            .with_maximum(Point::create(1.0, 1.0, 1.0));

        let tests = vec![
            (Point::create(5.0, 0.5, 0.0), Vector::create(-1.0, 0.0, 0.0), true),
            (Point::create(-5.0, 0.5, 0.0), Vector::create(1.0, 0.0, 0.0), true),
            (Point::create(0.5, 5.0, 0.0), Vector::create(0.0, -1.0, 0.0), true),
            (Point::create(0.5, -5.0, 0.0), Vector::create(0.0, 1.0, 0.0), true),
            (Point::create(0.5, 0.0, 5.0), Vector::create(0.0, 0.0, -1.0), true),
            (Point::create(0.5, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0), true),
            (Point::create(0.0, 0.5, 0.0), Vector::create(0.0, 0.0, 1.0), true),
            (Point::create(-2.0, 0.0, 0.0), Vector::create(2.0, 4.0, 6.0), false),
            (Point::create(0.0, -2.0, 0.0), Vector::create(6.0, 2.0, 4.0), false),
            (Point::create(0.0, 0.0, -2.0), Vector::create(4.0, 6.0, 2.0), false),
            (Point::create(2.0, 0.0, 2.0), Vector::create(0.0, 0.0, -1.0), false),
            (Point::create(0.0, 2.0, 2.0), Vector::create(0.0, -1.0, 0.0), false),
            (Point::create(2.0, 2.0, 0.0), Vector::create(-1.0, 0.0, 0.0), false)
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);

            let result = BoundingBox::intersects(&bounds, &ray);

            assert_eq!(result, test.2);
        }
    }

    #[test]
    fn test_intersect_a_ray_with_a_non_cubic_bounding_box() {
        let bounds = BoundingBox::create()
            .with_minimum(Point::create(5.0, -2.0, 0.0))
            .with_maximum(Point::create(11.0, 4.0, 7.0));

        let tests = vec![
            (Point::create(15.0, 1.0, 2.0), Vector::create(-1.0, 0.0, 0.0), true),
            (Point::create(-5.0, -1.0, 4.0), Vector::create(1.0, 0.0, 0.0), true),
            (Point::create(7.0, 6.0, 5.0), Vector::create(0.0, -1.0, 0.0), true),
            (Point::create(9.0, -5.0, 6.0), Vector::create(0.0, 1.0, 0.0), true),
            (Point::create(8.0, 2.0, 12.0), Vector::create(0.0, 0.0, -1.0), true),
            (Point::create(6.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0), true),
            (Point::create(8.0, 1.0, 3.5), Vector::create(0.0, 0.0, 1.0), true),
            (Point::create(9.0, -1.0, -8.0), Vector::create(2.0, 4.0, 6.0), false),
            (Point::create(8.0, 3.0, -4.0), Vector::create(6.0, 2.0, 4.0), false),
            (Point::create(9.0, -1.0, -2.0), Vector::create(4.0, 6.0, 2.0), false),
            (Point::create(4.0, 0.0, 9.0), Vector::create(0.0, 0.0, -1.0), false),
            (Point::create(8.0, 6.0, -1.0), Vector::create(0.0, -1.0, 0.0), false),
            (Point::create(12.0, 5.0, 4.0), Vector::create(-1.0, 0.0, 0.0), false)
        ];

        for test in tests {
            let direction = test.1.normalize();
            let ray = Ray::create(test.0, direction);

            let result = BoundingBox::intersects(&bounds, &ray);

            assert_eq!(result, test.2);
        }
    }

    #[test]
    fn test_splitting_a_perfect_cube() {
        let bounds = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -4.0, -5.0))
            .with_maximum(Point::create(9.0, 6.0, 5.0));

        let (left, right) = bounds.split();

        assert_eq!(left.minimum(), Point::create(-1.0, -4.0, -5.0));
        assert_eq!(left.maximum(), Point::create(4.0, 6.0, 5.0));
        assert_eq!(right.minimum(), Point::create(4.0, -4.0, -5.0));
        assert_eq!(right.maximum(), Point::create(9.0, 6.0, 5.0));
    }

    #[test]
    fn test_splitting_an_x_wide_box() {
        let bounds = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -2.0, -3.0))
            .with_maximum(Point::create(9.0, 5.5, 3.0));

        let (left, right) = bounds.split();

        assert_eq!(left.minimum(), Point::create(-1.0, -2.0, -3.0));
        assert_eq!(left.maximum(), Point::create(4.0, 5.5, 3.0));
        assert_eq!(right.minimum(), Point::create(4.0, -2.0, -3.0));
        assert_eq!(right.maximum(), Point::create(9.0, 5.5, 3.0));
    }

    #[test]
    fn test_splitting_a_y_wide_box() {
        let bounds = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -2.0, -3.0))
            .with_maximum(Point::create(5.0, 8.0, 3.0));

        let (left, right) = bounds.split();

        assert_eq!(left.minimum(), Point::create(-1.0, -2.0, -3.0));
        assert_eq!(left.maximum(), Point::create(5.0, 3.0, 3.0));
        assert_eq!(right.minimum(), Point::create(-1.0, 3.0, -3.0));
        assert_eq!(right.maximum(), Point::create(5.0, 8.0, 3.0));
    }

    #[test]
    fn test_splitting_a_z_wide_box() {
        let bounds = BoundingBox::create()
            .with_minimum(Point::create(-1.0, -2.0, -3.0))
            .with_maximum(Point::create(5.0, 3.0, 7.0));

        let (left, right) = bounds.split();

        assert_eq!(left.minimum(), Point::create(-1.0, -2.0, -3.0));
        assert_eq!(left.maximum(), Point::create(5.0, 3.0, 2.0));
        assert_eq!(right.minimum(), Point::create(-1.0, -2.0, 2.0));
        assert_eq!(right.maximum(), Point::create(5.0, 3.0, 7.0));
    }
}