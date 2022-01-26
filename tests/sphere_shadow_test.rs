use std::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::canvas::Canvas;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::WHITE;
use rusty_ray_tracer::features::intersection::Intersection;
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::TwoColorCreate;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::ray::Ray;
use rusty_ray_tracer::features::shapes::shape::{Object, Shape};
use rusty_ray_tracer::features::shapes::{Intersect, Normal};
use rusty_ray_tracer::features::shapes::sphere::Sphere;

#[test]
#[ignore]
fn sphere_shadow_test() {
    let ray_origin = Point::create(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 28.0;
    let canvas_pixels = 200;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::create(canvas_pixels, canvas_pixels);
    let pattern = CheckerPattern::create(Color::create(1.0, 0.0, 1.0), Color::create(1.0, 0.5, 0.0));
    let mut material = Material::create();
    material.set_pattern_box(Box::new(pattern));
    let shape = Shape::Sphere.create()
        .with_material(material)
        .with_transform(Matrix::scale(2.0, 2.0, 2.0));

    //light source
    let light_position = Point::create(-10.0, 10.0, -10.0);
    let light = PointLight::create(WHITE, light_position);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::create(world_x, world_y, wall_z);
            let direction = (position - ray_origin).normalize();
            let ray = Ray::create(ray_origin, direction);

            match Intersection::hit(Object::intersect(&shape, &ray)) {
                None => {}
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = Object::normal(&hit.object, &point);
                    let eye = -ray.direction();
                    let color = hit.object.material().lighting(&light, &hit.object, &point, &eye, &normal, false);
                    canvas.write_pixel(x, y, color);
                }
            }
        }
    }

    canvas.convert_to_ppm_and_save(String::from("sphere_shadow_test.ppm"));
}