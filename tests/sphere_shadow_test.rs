use rusty_ray_tracer::draw::canvas::Canvas;
use rusty_ray_tracer::draw::color::Color;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::intersection::Intersection;
use rusty_ray_tracer::features::matrix::Matrix;
use rusty_ray_tracer::features::point::Point;
use rusty_ray_tracer::features::ray::Ray;
use rusty_ray_tracer::features::shapes::Shape;
use rusty_ray_tracer::features::shapes::sphere::Sphere;

#[test]
fn sphere_shadow_test() {
    let ray_origin = Point::create(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::create(canvas_pixels, canvas_pixels);
    let color = Color::create((1.0, 0.0, 0.0));
    let shape = Sphere::create();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::create(world_x, world_y, wall_z);
            let direction = position.subtract_point(ray_origin).normalize();
            let ray = Ray::create(ray_origin, direction);

            match Intersection::hit(shape.intersect(ray)) {
                None => {}
                Some(_) => { canvas.write_pixel(x, y, color) }
            }
        }
    }

    //canvas.convert_to_ppm_and_save(String::from("sphere_shadow_test.ppm"));
}