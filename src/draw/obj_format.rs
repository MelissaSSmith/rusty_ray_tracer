use std::collections::HashMap;
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
        let mut default_group = Shape::Group(Group::create()).create();
        let mut groups: HashMap<String, Object> = HashMap::new();
        let mut last_group_touched = String::from("Default");
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    match l.get(..1) {
                        Some("v") => {
                            vertices.push(OBJParser::create_vertex(l));
                        }
                        Some("f") => {
                            let triangles = OBJParser::fan_triangulation(l, &vertices);
                            if last_group_touched.eq(&String::from("Default")) {
                                for triangle in triangles {
                                    default_group.add_child(Shape::Triangle(triangle).create());
                                }
                            } else {
                                let last_group = groups.get_mut(last_group_touched.as_str());
                                if let Some(group) = last_group {
                                    for triangle in triangles {
                                        group.add_child(Shape::Triangle(triangle).create());
                                    }
                                }
                            }
                        }
                        Some("g") => {
                            let name = OBJParser::parse_group_name(l);
                            groups.insert(name.clone(), Shape::Group(Group::create()).create());
                            last_group_touched = name;
                        }
                        _ => { ignored_lines += 1; }
                    }
                }
                Err(_) => {
                    ignored_lines += 1;
                }
            }
        }
        let final_groups = groups.values().cloned().collect::<Vec<Object>>();
        for final_group in final_groups {
            default_group.add_child(final_group);
        }
        OBJParser::create(ignored_lines, vertices, default_group)
    }

    fn create_vertex(line: String) -> Point {
        let tokens: Vec<&str> = line.split(" ").collect();

        let x: f64 = tokens[1].parse().unwrap();
        let y: f64 = tokens[2].parse().unwrap();
        let z: f64 = tokens[3].parse().unwrap();

        Point::create(x, y, z)
    }

    fn fan_triangulation(line: String, vertices: &Vec<Point>) -> Vec<Triangle> {
        let tokens: Vec<&str> = line.split(" ").collect();
        let mut triangles = vec![];

        let mut range = tokens.len()-1;
        if tokens.len() > 4 {
            range = tokens.len()-2;
        }
        for index in 2..range {
            if index >= vertices.len() {
                break;
            }
            let index1 = tokens[1].parse::<usize>().unwrap();
            let index2 = tokens[index].parse::<usize>().unwrap();
            let index3 = tokens[index+1].parse::<usize>().unwrap();
            let triangle = Triangle::create(vertices[index1], vertices[index2], vertices[index3]);
            triangles.push(triangle);
        }

        triangles
    }

    fn parse_group_name(line: String) -> String {
        let tokens: Vec<&str> = line.split(" ").collect();
        tokens[1].to_string()
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

    #[test]
    fn test_file_with_polygon_data() {
        let file_contents = "\
        v -1 1 0 \nv -1 0 0 \nv 1 0 0 \nv 1 1 0 \nv 0 2 0 \n\nf 1 2 3 4 5 \
        ";
        write_to_file(String::from("polygon_data.txt"), String::from(file_contents));

        let parser = OBJParser::parse_obj_file(String::from("polygon_data.txt"));

        assert_eq!(1, parser.ignored_lines);
        assert_eq!(3, parser.default_group.children().len());

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

        let t3 = parser.default_group.children()[2].clone();
        let t3_shape = match t3.shape() {
            Shape::Triangle(t) => t,
            _ => panic!("Object is not a triangle!")
        };
        assert!(parser.vertices[1].equals(t3_shape.point1()));
        assert!(parser.vertices[4].equals(t3_shape.point2()));
        assert!(parser.vertices[5].equals(t3_shape.point3()));
    }

    #[test]
    fn test_named_groups_in_obj_files() {
        let parser = OBJParser::parse_obj_file(String::from("triangles.obj"));

        assert_eq!(1, parser.ignored_lines);
        assert_eq!(2, parser.default_group.children().len());

        let t1 = parser.default_group.children()[0].children()[0].clone();
        let t1_shape = match t1.shape() {
            Shape::Triangle(t) => t,
            _ => panic!("Object is not a triangle!")
        };
        assert!(parser.vertices[1].equals(t1_shape.point1()));
        assert!(parser.vertices[2].equals(t1_shape.point2()));
        assert!(parser.vertices[3].equals(t1_shape.point3()));

        let t2 = parser.default_group.children()[1].children()[0].clone();
        let t2_shape = match t2.shape() {
            Shape::Triangle(t) => t,
            _ => panic!("Object is not a triangle!")
        };
        assert!(parser.vertices[1].equals(t2_shape.point1()));
        assert!(parser.vertices[3].equals(t2_shape.point2()));
        assert!(parser.vertices[4].equals(t2_shape.point3()));
    }
}