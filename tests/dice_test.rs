use core::f64::consts::{FRAC_PI_2, PI};
use rand::{Rng, thread_rng};
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLACK, WHITE};
use rusty_ray_tracer::features::lights::point_light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, TwoPatternCreate};
use rusty_ray_tracer::features::patterns::gradient::GradientPattern;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::csg::{CSG, CSGOperation};
use rusty_ray_tracer::features::shapes::group::Group;
use rusty_ray_tracer::features::shapes::shape::{Object, Shape};
use rusty_ray_tracer::features::shapes::shape::Shape::Sphere;
use rusty_ray_tracer::features::world::World;

fn dice(material1: Material, material2: Material) -> Object {
    let body = dice_body(material1);
    let body = CSG::create(CSGOperation::Difference, body, dice_side1(material2.clone()));
    let body = CSG::create(CSGOperation::Difference,
                           Shape::CSG(body).create(),
                           dice_side6(material2.clone()).with_transform(Matrix::scale(1.0, -1.0, 1.0)));
    let body = CSG::create(CSGOperation::Difference,
                           Shape::CSG(body).create(),
                           dice_side2(material2.clone()).with_transform(Matrix::rotate_x(PI/2.0)));
    let body = CSG::create(CSGOperation::Difference,
                           Shape::CSG(body).create(),
                           dice_side5(material2.clone()).with_transform(Matrix::rotate_x(-PI/2.0)));
    let body = CSG::create(CSGOperation::Difference,
                           Shape::CSG(body).create(),
                           dice_side3(material2.clone()).with_transform(Matrix::rotate_z(PI/2.0)));
    let body = CSG::create(CSGOperation::Difference,
                           Shape::CSG(body).create(),
                           dice_side4(material2.clone()).with_transform(Matrix::rotate_z(-PI/2.0)));

    Shape::CSG(body).create()
}

fn dice_side1(material: Material) -> Object {
    dice_point(0.0, 0.0, &material)
}

fn dice_side2(material: Material) -> Object {
    Shape::Group(Group::create()).create()
        .with_children(vec![
            dice_point(-0.8, -0.8, &material),
            dice_point(0.8, 0.8, &material)
        ])
}

fn dice_side3(material: Material) -> Object {
    Shape::Group(Group::create()).create()
        .with_children(vec![
            dice_point(0.0, 0.0, &material),
            dice_point(-1.0, -1.0, &material),
            dice_point(1.0, 1.0, &material)
        ])
}

fn dice_side4(material: Material) -> Object {
    Shape::Group(Group::create()).create()
        .with_children(vec![
            dice_point(-0.8, -0.8, &material),
            dice_point(-0.8, 0.8, &material),
            dice_point(0.8, -0.8, &material),
            dice_point(0.8, 0.8, &material)
        ])
}

fn dice_side5(material: Material) -> Object {
    Shape::Group(Group::create()).create()
        .with_children(vec![
            dice_point(0.0, 0.0, &material),
            dice_point(-1.0, -1.0, &material),
            dice_point(1.0, -1.0, &material),
            dice_point(-1.0, 1.0, &material),
            dice_point(1.0, 1.0, &material)
        ])
}

fn dice_side6(material: Material) -> Object {
    Shape::Group(Group::create()).create()
        .with_children(vec![
            dice_point(-1.0, -1.0, &material),
            dice_point(-0.8, 0.0, &material),
            dice_point(-1.0, 1.0, &material),
            dice_point(1.0, -1.0, &material),
            dice_point(1.0, 0.0, &material),
            dice_point(1.0, 1.0, &material)
        ])
}

fn dice_point(i: f64, j:f64, material: &Material) -> Object {
    Shape::Sphere.create()
        .with_material(material.clone())
        .with_transform(Matrix::translate(0.5 * i, 1.0, 0.5 * j) * Matrix::scale(0.2, 0.1, 0.2))
}

fn dice_body(material: Material) -> Object {
    let csg = CSG::create(
        CSGOperation::Intersection,
        Shape::Cube.create()
            .with_material(material.clone()),
        Shape::Sphere.create()
            .with_material(material)
            .with_transform(Matrix::scale(1.5, 1.5, 1.5))
    );

    Shape::CSG(csg).create()
}

#[test]
#[ignore]
fn dice_test() {
    let camera = Camera::create(1080, 1080, PI/3.0)
        .with_transform(Matrix::view_transform(
            Point::create(0.0, 0.0, -3.5),
            Point::create(0.0, 0.0, 0.0),
            Vector::create(0.0, 1.0, 0.0)
        ));

    let floor_material = Material::create()
        .with_pattern(CheckerPattern::create(
            SolidPattern::create(Color::create(0.75, 0.75, 0.75)),
            SolidPattern::create(Color::create(0.9, 0.9, 0.9))
        ).with_transform(Matrix::scale(0.1, 0.1, 0.1)))
        .with_diffuse(0.5)
        .with_specular(0.0);

    let floor = Shape::Plane.create()
        .with_material(floor_material)
        .with_transform(Matrix::translate(0.0, 0.0, 2.0) * Matrix::rotate_x(PI/2.0));

    let material = Material::create()
        .with_color(BLACK)
        .with_ambient(0.0)
        .with_diffuse(0.0)
        .with_specular(0.9)
        .with_shininess(500.0)
        .with_reflective(1.0)
        .with_transparency(1.0)
        .with_refractive_index(1.5);

    let a = Shape::Sphere.create()
        .with_material(material.clone())
        .with_transform(Matrix::translate(0.0, 0.0, -0.8));
    let b = Shape::Sphere.create()
        .with_material(material.clone())
        .with_transform(Matrix::translate(0.0, 0.0, -0.8));
    let lens = Shape::CSG(
        CSG::create(CSGOperation::Union, a, b)
    ).create()
        .with_has_shadow(false);

    let mut dices = vec![];
    for i in -2..=2 {
        for j in -4..=4 {
            let mut rnd = thread_rng();
            let hue = rnd.gen_range(0.0, 1.0);
            let material1 = Material::create()
                .with_color(Color::create(hue, 0.8, 1.0))
                .with_diffuse(1.0);
            let material2 = Material::create()
                .with_color(Color::create(hue, 0.8, 1.0))
                .with_diffuse(1.0);
            let size = rnd.gen_range(0.05, 0.1);
            let pos_x = i as f64 * 0.4 + rnd.gen_range(-0.1, 0.1);
            let pos_y = j as f64 * 0.4 + rnd.gen_range(-0.1, 0.1);

            let radians = rnd.gen_range(0.0, 2.0 * PI);
            //let rotax: [f64; 3] = rnd.gen();
            //let rotax: Vector = rotax.into();

            dices.push(dice(material1, material2).with_transform(
                Matrix::translate(pos_x, pos_y, 1.8)
                    * Matrix::rotate_x(radians)
                    * Matrix::scale(size, size, size),
            ));
        }
    }
    let dice_group = Shape::Group(Group::create()).create().with_children(dices);

    let light_source = PointLight::create(WHITE, Point::create(0.0, 8.0, -7.0));
    let objects = vec![floor, dice_group, lens];
    let world = World::create_world(light_source, objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("dice_test.ppm"));
}