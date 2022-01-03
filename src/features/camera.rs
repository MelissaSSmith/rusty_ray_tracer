use crate::features::canvas::Canvas;
use crate::features::matrix::Matrix;
use crate::features::point::Point;
use crate::features::ray::Ray;
use crate::features::world::World;

pub struct Camera {
    h_size: i32,
    v_size: i32,
    field_of_view: f64,
    transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64
}

struct PixelSize {
    half_width: f64,
    half_height: f64,
    pixel_size: f64
}

impl Camera {
    pub fn create(h_size: i32, v_size: i32, field_of_view: f64) -> Camera {
        let pixel_size = Camera::calculate_pixel_size(h_size, v_size, field_of_view);
        Camera {
            h_size,
            v_size,
            field_of_view,
            transform: Matrix::create_identity(),
            half_width: pixel_size.half_width,
            half_height: pixel_size.half_height,
            pixel_size: pixel_size.pixel_size
        }
    }

    fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inverse_transform = self.transform.inverse();
        let pixel = inverse_transform.multiply_point(Point::create(world_x, world_y, -1.0));
        let origin = inverse_transform.multiply_point(Point::create(0.0, 0.0, 0.0));
        let direction = pixel.subtract_point(origin).normalize();

        Ray::create(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = Canvas::create(self.h_size, self.v_size);

        for y in 0..self.v_size {
            for x in 0..self.h_size {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                canvas.write_pixel(x, y, color);
            }
        }

        canvas
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
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
    use std::f64::consts::PI;
    use crate::features::camera::Camera;
    use crate::features::color::Color;
    use crate::features::matrix::Matrix;
    use crate::features::operations::Operations;
    use crate::features::point::Point;
    use crate::features::vector::Vector;
    use crate::features::world::World;

    #[test]
    fn test_constructing_a_camera() {
        let camera = Camera::create(160, 120, PI/2.0);

        assert_eq!(camera.h_size, 160);
        assert_eq!(camera.v_size, 120);
        assert_eq!(camera.field_of_view, PI/2.0);
        assert!(camera.transform.equals(Matrix::create_identity()));
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

        assert!(ray.direction.equals(Vector::create(0.0, 0.0, -1.0)));
        assert!(ray.origin.equals(Point::create(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_construct_ray_through_corner_of_the_canvas() {
        let camera = Camera::create(201, 101, PI/2.0);

        let ray = camera.ray_for_pixel(0, 0);

        assert!(ray.direction.equals(Vector::create(0.66519, 0.33259, -0.66851)));
        assert!(ray.origin.equals(Point::create(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_construct_ray_when_the_camera_is_transformed() {
        let mut camera = Camera::create(201, 101, PI/2.0);
        camera.transform = Matrix::rotate_y(PI/4.0).multiply(&Matrix::translate(0.0, -2.0, 5.0));

        let ray = camera.ray_for_pixel(100, 50);

        assert!(ray.direction.equals(Vector::create(2.0_f64.sqrt()/2.0, 0.0, -2.0_f64.sqrt()/2.0)));
        assert!(ray.origin.equals(Point::create(0.0, 2.0, -5.0)));
    }

    #[test]
    fn test_render_a_world_with_a_camera() {
        let world = World::create_default();
        let mut camera = Camera::create(11, 11, PI/2.0);
        let from = Point::create(0.0, 0.0, -5.0);
        let to = Point::create(0.0, 0.0, 0.0);
        let up = Vector::create(0.0, 1.0, 0.0);
        camera.transform = Matrix::view_transform(from, to, up);

        let image = camera.render(world);

        let color = image.get_pixel(5, 5);
        assert!(color.equals(Color::create(0.38066, 0.47583, 0.2855)));
    }
}