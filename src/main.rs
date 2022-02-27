use std::env;
use std::f64::consts::PI;
use std::path::PathBuf;
use std::time::Instant;
use crate::draw::obj_format::OBJParser;
use crate::draw::ppm_format::PPMFile;
use crate::features::camera::Camera;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::primitives::vector::Vector;
use crate::world_builder::create_default_world_with_group;

mod features;
mod draw;
mod world_builder;


// Command 1: generate image from obj file using a default world `create_object <filename>`

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let filename = &args[2];

    println!("Started {} {}", command, filename);

    match command.as_str() {
        "create_object" => {
            let group = OBJParser::parse(filename);
            println!("Parsed into group. Time {}", start.elapsed().as_secs());

            let world = create_default_world_with_group(group);
            println!("Created World. Time {}", start.elapsed().as_secs());

            let camera = Camera::create(300, 300, PI/3.0) //high def: 1080 1080
                .with_transform(Matrix::view_transform(
                    Point::create(0.0, 1.5, -60.0),
                    Point::create(0.0, 1.0, 0.0),
                    Vector::create(0.0, 1.0, 0.0)
                ));
            let canvas = camera.render(world);
            println!("Rendered World. Time {}", start.elapsed().as_secs());

            let mut path = PathBuf::from(filename);
            path.set_extension("ppm");
            canvas.convert_to_ppm_and_save(format!("{}", path.file_name().unwrap().to_str().unwrap()));
            println!("Finished {} {}. Final Time {}", command, filename, start.elapsed().as_secs());
        }
        _ => {
            println!("Finished {} {}. Final Time {}", command, filename, start.elapsed().as_secs());
        }
    }
}
