use core::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLACK, WHITE};
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::gradient::GradientPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, OnePatternWithScale, TwoColorCreate};
use rusty_ray_tracer::features::patterns::perturb::PerturbedPattern;
use rusty_ray_tracer::features::patterns::radial_gradient::RadialGradientPattern;
use rusty_ray_tracer::features::patterns::ring::RingPattern;
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::patterns::stripe::StripePattern;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn sphere_scene_test() {
    let material = Material::create()
        .with_color(Color::create(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let mut mid_pattern = RingPattern::create(Color::create(0.0, 0.0, 1.0), Color::create(0.1, 1.0, 0.5));
    mid_pattern.transform(Matrix::scale(0.25, 0.25, 0.25));
    let mid_material = Material::create()
        .with_ambient(0.1)
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_reflective(0.25)
        .with_pattern(PerturbedPattern::create(mid_pattern, Some(2.5)));
    let middle = Shape::Sphere.create()
        .with_transform(Matrix::translate(-0.5, 1.0, 0.5) * Matrix::rotate_x(PI/2.0))
        .with_material(mid_material);

    let r_transform = Matrix::translate(1.5, 0.5, -0.5) * Matrix::scale(0.5, 0.5, 0.5);
    let mut r_pattern = RadialGradientPattern::create(Color::create(1.0, 0.0, 1.0), Color::create(0.5, 1.0, 0.1));
    r_pattern.transform(Matrix::translate(-1.5, -0.5, 0.5) * Matrix::scale(0.5, 0.5, 0.5));
    let r_perturb_pattern = PerturbedPattern::create(r_pattern, Some(4.0));
    let r_material = Material::create()
        .with_ambient(0.1)
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_pattern(r_perturb_pattern);
    let right = Shape::Sphere.create()
        .with_transform(r_transform)
        .with_material(r_material);

    let l_transform = Matrix::translate(-1.5, 0.33, -0.75) * Matrix::scale(0.4, 0.4, 0.4);
    let mut l_pattern = SolidPattern::create(Color::create(1.0, 0.75, 0.0));
    l_pattern.transform(Matrix::scale(0.1, 0.1, 0.1));
    let l_perturb_pattern = PerturbedPattern::create(l_pattern, None);
    let l_material = Material::create()
        .with_ambient(0.1)
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_pattern(l_perturb_pattern);
    let left = Shape::Sphere.create()
        .with_transform(l_transform)
        .with_material(l_material);

    let pattern = CheckerPattern::create(Color::create(0.906, 0.329, 0.502), WHITE);
    let perturb_pattern = PerturbedPattern::create(pattern, Some(4.0));
    let floor_material = material.clone().with_pattern(perturb_pattern);

    let _backdrop = Shape::Plane.create()
        .with_material(Material::create().with_ambient(1.0).with_color(Color::create(0.1, 0.1, 0.1)).with_reflective(1.0))
        .with_transform(Matrix::translate(0.0, 0.0, 4.0) * Matrix::rotate_x(PI/2.0));

    let floor = Shape::Plane.create().with_material(floor_material);

    let light_source = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
    let objects = vec![floor, middle, right, left];
    let world = World::create_world(light_source, objects);

    let mut camera = Camera::create(400, 350, PI/3.0);
    camera.set_transform(Matrix::view_transform(Point::create(0.0, 1.5, -5.0), Point::create(0.0, 1.0, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("sphere_and_planes_test.ppm"));
}