use std::f64::consts::{FRAC_PI_4, PI};
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLACK, WHITE};
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::patterns::blended::BlendedPattern;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::Pattern;
use rusty_ray_tracer::features::patterns::stripe::StripePattern;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::shapes::plane::Plane;
use rusty_ray_tracer::features::shapes::Shape;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn sphere_scene_test() {
    let mut pattern_a = StripePattern::create(Color::create(1.0, 0.7529, 0.7961), Color::create(0.906, 0.329, 0.502));
    let transform_a = Matrix::rotate_y(FRAC_PI_4).multiply(&Matrix::scale(0.25, 0.25, 0.25));
    pattern_a.transform(transform_a);
    let mut pattern_b = StripePattern::create(Color::create(0.0, 0.0, 0.3921), Color::create(0.0, 0.0, 0.967));
    let transform_b = Matrix::rotate_y(-FRAC_PI_4).multiply(&Matrix::scale(0.25, 0.25, 0.25));
    pattern_b.transform(transform_b);

    let pattern = CheckerPattern::create_with_patterns(Box::new(pattern_a), Box::new(pattern_b));
    let mut floor_material = Material::create();
    floor_material.set_pattern(Box::new(pattern));

    let mut floor = Plane::create();
    floor.set_material(floor_material);
    floor.set_transform(Matrix::translate(0.0, 1.0, 0.0));

    let light_source = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
    let objects: Vec<Box<dyn Shape>> = vec![Box::new(floor)];
    let world = World::create_world(light_source, objects);

    let mut camera = Camera::create(450, 350, PI/3.0);
    camera.set_transform(Matrix::view_transform(Point::create(0.0, 1.5, -5.0), Point::create(0.0, 1.0, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("plane_test.ppm"));
}