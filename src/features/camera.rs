use crate::features::canvas::Canvas;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::ray::Ray;
use crate::features::world::World;

use rayon::prelude::*;

pub struct Camera {
    h_size: i32,
    v_size: i32,
    field_of_view: f64,
    transform: Matrix,
    inverse_transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    origin: Point
}

struct PixelSize {
    half_width: f64,
    half_height: f64,
    pixel_size: f64
}

impl Camera {
    pub fn create(h_size: i32, v_size: i32, field_of_view: f64) -> Camera {
        let pixel_size = Camera::calculate_pixel_size(h_size, v_size, field_of_view);
        let transform = Matrix::identity();
        Camera {
            h_size,
            v_size,
            field_of_view,
            transform,
            inverse_transform: transform.inverse(),
            half_width: pixel_size.half_width,
            half_height: pixel_size.half_height,
            pixel_size: pixel_size.pixel_size,
            origin: transform.inverse() * Point::zero()
        }
    }

    fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.inverse_transform * Point::create(world_x, world_y, -1.0);
        let direction = (pixel - self.origin).normalize();

        Ray::create(self.origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let canvas = Canvas::create(self.h_size, self.v_size);

        canvas.pixels
            .par_iter_mut()
            .for_each(|mut map_value| {
                let x_y = canvas.get_x_y(map_value.key());
                let x = x_y.0;
                let y = x_y.1;

                let ray = self.ray_for_pixel(x, y);
                *map_value = world.color_at(&ray);
            });

        canvas
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.inverse_transform = self.transform.inverse();
        self.origin = self.inverse_transform * Point::zero();
    }

    pub fn with_transform(self, transform: Matrix) -> Camera {
        Camera {
            transform,
            inverse_transform: transform.inverse(),
            origin: transform.inverse() * Point::zero(),
            ..self
        }
    }

    fn calculate_pixel_size(h_size: i32, v_size: i32, field_of_view: f64) -> PixelSize {
        let half_view = (field_of_view/2.0).tan();
        let aspect = h_size as f64 / v_size as f64;
        let mut pixel_size = PixelSize::create();
        if aspect >= 1.0 {
            pixel_size.half_width = half_view;
            pixel_size.half_height = half_view / aspect;
        } else {
            pixel_size.half_width = half_view * aspect;
            pixel_size.half_height = half_view;
        }
        pixel_size.pixel_size = (pixel_size.half_width * 2.0) / h_size as f64;
        pixel_size
    }
}

impl PixelSize {
    fn create() -> PixelSize {
        PixelSize {
            half_width: 0.0,
            half_height: 0.0,
            pixel_size: 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;
    use crate::features::camera::Camera;
    use crate::features::color::Color;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::operations::Operations;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::world::World;

    #[test]
    fn test_constructing_a_camera() {
        let camera = Camera::create(160, 120, PI/2.0);

        assert_eq!(camera.h_size, 160);
        assert_eq!(camera.v_size, 120);
        assert_eq!(camera.field_of_view, PI/2.0);
        assert_eq!(camera.transform, Matrix::identity());
    }

    #[test]
    fn test_pixel_size_for_a_horizontal_canvas() {
        let camera = Camera::create(200, 125, PI/2.0);

        assert!(camera.pixel_size.equals(0.01));
    }

    #[test]
    fn test_pixel_size_for_a_vertical_canvas() {
        let camera = Camera::create(125, 200, PI/2.0);

        assert!(camera.pixel_size.equals(0.01));
    }

    #[test]
    fn test_construct_a_ray_through_the_center_of_the_canvas() {
        let camera = Camera::create(201, 101, PI/2.0);

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.direction, Vector::create(0.0, 0.0, -1.0));
        assert_eq!(ray.origin, Point::zero());
    }

    #[test]
    fn test_construct_ray_through_corner_of_the_canvas() {
        let camera = Camera::create(201, 101, PI/2.0);

        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(ray.direction, Vector::create(0.66519, 0.33259, -0.66851));
        assert_eq!(ray.origin, Point::zero());
    }

    #[test]
    fn test_construct_ray_when_the_camera_is_transformed() {
        let mut camera = Camera::create(201, 101, PI/2.0);
        camera.set_transform(Matrix::rotate_y(PI/4.0) * Matrix::translate(0.0, -2.0, 5.0));

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.direction, Vector::create(2.0_f64.sqrt()/2.0, 0.0, -2.0_f64.sqrt()/2.0));
        assert_eq!(ray.origin, Point::create(0.0, 2.0, -5.0));
    }

    #[test]
    fn test_render_a_world_with_a_camera() {
        let world = World::create_default();
        let mut camera = Camera::create(11, 11, PI/2.0);
        let from = Point::create(0.0, 0.0, -5.0);
        let to = Point::zero();
        let up = Vector::create(0.0, 1.0, 0.0);
        camera.set_transform(Matrix::view_transform(from, to, up));

        let image = camera.render(world);

        let color = image.get_pixel(5, 5);
        assert!(color.equals(Color::create(0.38066, 0.47583, 0.2855)));
    }
}