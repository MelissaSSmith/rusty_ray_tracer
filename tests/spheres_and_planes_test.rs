use std::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLACK, WHITE};
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::gradient::GradientPattern;
use rusty_ray_tracer::features::patterns::Pattern;
use rusty_ray_tracer::features::patterns::perturb::PerturbedPattern;
use rusty_ray_tracer::features::patterns::radial_gradient::RadialGradientPattern;
use rusty_ray_tracer::features::patterns::ring::RingPattern;
use rusty_ray_tracer::features::patterns::stripe::StripePattern;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::shapes::plane::Plane;
use rusty_ray_tracer::features::shapes::Shape;
use rusty_ray_tracer::features::shapes::sphere::Sphere;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn sphere_scene_test() {
    let mut material = Material::create();
    material.set_color(Color::create(1.0, 0.9, 0.9));
    material.set_specular(0.0);

    let mut middle = Sphere::create();
    middle.set_transform(Matrix::translate(-0.5, 1.0, 0.5).multiply(&Matrix::rotate_x(PI/2.0)));
    let mut mid_pattern = RingPattern::create(Color::create(0.0, 0.0, 1.0), Color::create(0.1, 1.0, 0.5));
    mid_pattern.transform(Matrix::scale(0.25, 0.25, 0.25));
    let mid_perturb_pattern = PerturbedPattern::create(Box::new(mid_pattern), Some(2.5));
    let mid_material = Material::create_with_attributes(0.1, 0.7, 0.3, None, None, None, Some(Box::new(mid_perturb_pattern)));
    middle.set_material(mid_material);

    let mut right = Sphere::create();
    let r_transform = Matrix::translate(1.5, 0.5, -0.5)
        .multiply(&Matrix::scale(0.5, 0.5, 0.5));
    right.set_transform(r_transform.clone());
    let mut r_pattern = RadialGradientPattern::create(Color::create(1.0, 0.0, 1.0), Color::create(0.5, 1.0, 0.1));
    r_pattern.transform(Matrix::translate(-1.5, -0.5, 0.5).multiply(&Matrix::scale(0.5, 0.5, 0.5)));
    let r_perturb_pattern = PerturbedPattern::create(Box::new(r_pattern), Some(4.0));
    let r_material = Material::create_with_attributes(0.1, 0.7, 0.3, None, None, None, Some(Box::new(r_perturb_pattern)));
    right.set_material(r_material);

    let mut left = Sphere::create();
    let l_transform = Matrix::translate(-1.5, 0.33, -0.75)
        .multiply(&Matrix::scale(0.4, 0.4, 0.4));
    left.set_transform(l_transform);
    let mut l_pattern = StripePattern::create(Color::create(0.0, 0.01, 1.0), Color::create(1.0, 0.0, 0.0));
    l_pattern.transform(Matrix::scale(0.1, 0.1, 0.1));
    let l_perturb_pattern = PerturbedPattern::create(Box::new(l_pattern), None);
    let l_material = Material::create_with_attributes(0.1, 0.7, 0.3, None, None,None, Some(Box::new(l_perturb_pattern)));
    left.set_material(l_material);

    let pattern = CheckerPattern::create(Color::create(0.906, 0.329, 0.502), WHITE);
    let perturb_pattern = PerturbedPattern::create(Box::new(pattern), Some(4.0));
    let mut floor_material = material.clone();
    floor_material.set_pattern(Box::new(perturb_pattern));

    let mut backdrop = Plane::create();
    let backdrop_transform = Matrix::rotate_x(PI/2.0)
        .multiply(&Matrix::translate(0.0, 0.0, 4.0));
    backdrop.set_transform(backdrop_transform);
    backdrop.set_material(floor_material.clone());

    let mut floor = Plane::create();
    floor.set_material(floor_material);

    let light_source = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
    let objects: Vec<Box<dyn Shape>> = vec![Box::new(floor), Box::new(backdrop),
                                            Box::new(middle), Box::new(right), Box::new(left)];
    let world = World::create_world(light_source, objects);

    let mut camera = Camera::create(900, 900, PI/3.0);
    camera.set_transform(Matrix::view_transform(Point::create(0.0, 1.5, -5.0), Point::create(0.0, 1.0, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("sphere_and_planes_test.ppm"));
}