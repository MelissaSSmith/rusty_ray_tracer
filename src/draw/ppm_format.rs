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

        OpenOptions::new().write(true).truncate(true).open(path).unwrap()
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
    use crate::features::color::Color;
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

    #[test]
    fn test_convert_canvas_to_ppm_pixel_data() {
        let width = 5;
        let height = 3;
        let mut canvas = Canvas::create(width, height);

        let color_a = Color::create(1.5, 0.0, 0.0);
        let color_b = Color::create(0.0, 0.5, 0.0);
        let color_c = Color::create(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, color_a);
        canvas.write_pixel(2, 1, color_b);
        canvas.write_pixel(4, 2, color_c);

        //let pixel_data = canvas.create_pixel_data();

        let expected = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";
        //assert_eq!(expected, pixel_data);
    }

    #[test]
    fn test_splitting_long_lines_when_convert_canvas_to_ppm_pixel_data() {
        let width = 10;
        let height = 2;
        let mut canvas = Canvas::create(width, height);

        for x in 0..width {
            for y in 0..height {
                let color = Color::create(1.0,0.8,0.6);
                canvas.write_pixel(x, y, color);
            }
        }

        //let pixel_data = canvas.create_pixel_data();

        let expected = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 \
        255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \
        255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153";
        //assert_eq!(expected, pixel_data);
    }

    #[test]
    fn test_convert_canvas_to_ppm_termination_line() {
        let result = Canvas::create_termination();

        let expected = String::from("\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_to_ppm() {
        let width = 10;
        let height = 2;
        let mut canvas = Canvas::create(width, height);

        for x in 0..width {
            for y in 0..height {
                let color = Color::create(1.0,0.8,0.6);
                canvas.write_pixel(x, y, color);
            }
        }

        //let ppm = canvas.convert_to_ppm();

        let expected = "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 \
        255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \
        255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        //assert_eq!(expected, ppm);
    }

}