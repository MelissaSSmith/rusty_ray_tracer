use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use crate::features::canvas::Canvas;

trait PPMFormat {
    fn create_header(&self) -> String;
    fn create_termination() -> String;
    fn write_to_ppm(&self, file: &File);
    fn write_to_file(file_name: String, file_data: String);
    fn open_new_file(file_name: String) -> File;
    fn write_line_to_file(file: &File, line: String);
}

pub trait PPMFile {
    fn convert_to_ppm_and_save(&self, file_name: String);
}

impl PPMFormat for Canvas {
    fn create_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height)
    }

    fn create_termination() -> String {
        String::from("\n")
    }

    fn write_to_ppm(&self, file: &File) {
        for h in 0..self.height {
            let mut line_array = Vec::<String>::new();
            for w in 0..self.width {
                let pixel = self.get_pixel(w, h);
                let scaled_color = pixel.scale_color();
                line_array.push(scaled_color.format_color_string());
            }

            Canvas::write_line_to_file(file, Canvas::format_pixel_line(line_array));
        }
    }

    fn write_to_file(file_name: String, file_data: String) {
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(file_data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        };
    }

    fn open_new_file(file_name: String) -> File {
        let path = Path::new(&file_name);

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path).unwrap()
    }

    fn write_line_to_file(mut file: &File, line: String) {
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to file: {}", why);
        }
    }
}

impl PPMFile for Canvas {
    fn convert_to_ppm_and_save(&self, file_name: String) {
        let file = Canvas::open_new_file(file_name);
        Canvas::write_line_to_file(&file, self.create_header());
        self.write_to_ppm(&file);
        Canvas::write_line_to_file(&file, Canvas::create_termination());
    }
}

#[cfg(test)]
mod tests {
    use crate::features::canvas::Canvas;
    use crate::draw::ppm_format::PPMFormat;

    #[test]
    fn test_convert_canvas_to_ppm_header() {
        let width = 5;
        let height = 3;
        let canvas = Canvas::create(width, height);

        let result = canvas.create_header();

        let expected = String::from("P3\n5 3\n255");
        assert_eq!(expected, result);
    }
}