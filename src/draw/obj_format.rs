use std::io::{BufRead, BufReader};
use crate::draw::file_operations::open_file;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::shapes::group::Group;
use crate::features::shapes::shape::{Object, Shape};
use crate::features::shapes::triangle::Triangle;

#[derive(Clone)]
struct OBJParser {
    ignored_lines: i32,
    vertices: Vec<Point>,
    default_group: Object
}


impl OBJParser {
    fn create(ignored_lines: i32, vertices: Vec<Point>, group: Object)  -> OBJParser {
        OBJParser {
            ignored_lines,
            vertices,
            default_group: group
        }
    }

    fn parse_obj_file(file_name: String) -> OBJParser {
        let file = open_file(file_name);
        let reader = BufReader::new(file);

        let mut ignored_lines = 0;
        let mut vertices = vec![Point::zero()];
        let mut group = Shape::Group(Group::create()).create();
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    match l.get(..1) {
                        Some("v") => {
                            vertices.push(OBJParser::create_vertex(l));
                        }
                        Some("f") => {
                            let triangle = OBJParser::create_triangle(l, &vertices);
                            group.add_child(Shape::Triangle(triangle).create());
                        }
                        _ => { ignored_lines += 1; }
                    }
                }
                Err(_) => {
                    ignored_lines += 1;
                }
            }
        }
        OBJParser::create(ignored_lines, vertices, group)
    }

    fn create_vertex(line: String) -> Point {
        let tokens: Vec<&str> = line.split(" ").collect();

        let x: f64 = tokens[1].parse().unwrap();
        let y: f64 = tokens[2].parse().unwrap();
        let z: f64 = tokens[3].parse().unwrap();

        Point::create(x, y, z)
    }

    fn create_triangle(line: String, vertices: &Vec<Point>) -> Triangle {
        let tokens: Vec<&str> = line.split(" ").collect();

        let index1 = tokens[1].parse::<usize>().unwrap();
        let index2 = tokens[2].parse::<usize>().unwrap();
        let index3 = tokens[3].parse::<usize>().unwrap();
        Triangle::create(vertices[index1], vertices[index2], vertices[index3])
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::file_operations::write_to_file;
    use crate::draw::obj_format::OBJParser;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_ignore_unrecognized_lines() {
        let file_contents = "There was a young lady named Bright \r
        who traveled much faster than light. \r
        She set out one day \r
        in a relative way \r
        and came back the previous night.";
        write_to_file(String::from("unrecognized_line.txt"), String::from(file_contents));

        let parser = OBJParser::parse_obj_file(String::from("unrecognized_line.txt"));

        assert_eq!(5, parser.ignored_lines);
    }

    #[test]
    fn test_vertex_records() {
        let file_contents = "\
        v -1 1 0 \nv -1.0000 0.5000 0.0000 \nv 1 0 0 \nv 1 1 0 \
        ";
        write_to_file(String::from("vertex_records.txt"), String::from(file_contents));

        let parser = OBJParser::parse_obj_file(String::from("vertex_records.txt"));

        assert_eq!(0, parser.ignored_lines);
        assert!(parser.vertices[1].equals(Point::create(-1.0, 1.0, 0.0)));
        assert!(parser.vertices[2].equals(Point::create(-1.0, 0.5, 0.0)));
        assert!(parser.vertices[3].equals(Point::create(1.0, 0.0, 0.0)));
        assert!(parser.vertices[4].equals(Point::create(1.0, 1.0, 0.0)));
    }

    #[test]
    fn test_file_with_triangle_data() {
        let file_contents = "\
        v -1 1 0 \nv -1 0 0 \nv 1 0 0 \nv 1 1 0 \n\nf 1 2 3 \nf 1 3 4 \
        ";
        write_to_file(String::from("triangle_data.txt"), String::from(file_contents));

        let parser = OBJParser::parse_obj_file(String::from("triangle_data.txt"));

        assert_eq!(1, parser.ignored_lines);

        let t1 = parser.default_group.children()[0].clone();
        let t1_shape = match t1.shape() {
            Shape::Triangle(t) => t,
            _ => panic!("Object is not a triangle!")
        };
        assert!(parser.vertices[1].equals(t1_shape.point1()));
        assert!(parser.vertices[2].equals(t1_shape.point2()));
        assert!(parser.vertices[3].equals(t1_shape.point3()));

        let t2 = parser.default_group.children()[1].clone();
        let t2_shape = match t2.shape() {
            Shape::Triangle(t) => t,
            _ => panic!("Object is not a triangle!")
        };
        assert!(parser.vertices[1].equals(t2_shape.point1()));
        assert!(parser.vertices[3].equals(t2_shape.point2()));
        assert!(parser.vertices[4].equals(t2_shape.point3()));
    }
}