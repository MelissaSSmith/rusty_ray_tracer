use std::io::{BufRead, BufReader};
use crate::draw::file_operations::open_file;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
struct OBJParser {
    ignored_lines: i32,
    vertices: Vec<Point>
}


impl OBJParser {
    fn create(ignored_lines: i32, vertices: Vec<Point>)  -> OBJParser {
        OBJParser {
            ignored_lines,
            vertices
        }
    }

    fn parse_obj_file(file_name: String) -> OBJParser {
        let file = open_file(file_name);
        let reader = BufReader::new(file);

        let mut ignored_lines = 0;
        let mut vertices = vec![];
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    match l.get(..1) {
                        Some("v") => {
                            vertices.push(OBJParser::create_vertex(l));
                        }
                        _ => { ignored_lines += 1; }
                    }
                }
                Err(_) => {
                    ignored_lines += 1;
                }
            }
        }
        OBJParser::create(ignored_lines, vertices)
    }

    fn create_vertex(line: String) -> Point {
        let tokens: Vec<&str> = line.split(" ").collect();

        let x: f64 = tokens[1].parse().unwrap();
        let y: f64 = tokens[2].parse().unwrap();
        let z: f64 = tokens[3].parse().unwrap();

        Point::create(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::file_operations::write_to_file;
    use crate::draw::obj_format::OBJParser;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

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
        assert!(parser.vertices[0].equals(Point::create(-1.0, 1.0, 0.0)));
        assert!(parser.vertices[1].equals(Point::create(-1.0, 0.5, 0.0)));
        assert!(parser.vertices[2].equals(Point::create(1.0, 0.0, 0.0)));
        assert!(parser.vertices[3].equals(Point::create(1.0, 1.0, 0.0)));
    }
}