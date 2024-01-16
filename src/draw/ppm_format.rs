use std::fs::File;
use crate::draw::file_operations::{open_new_file, write_line_to_file};
use crate::features::canvas::Canvas;

trait PPMFormat {
    fn create_header(&self) -> String;
    fn create_termination() -> String;
    fn write_to_ppm(&self, file: &File);
}

pub trait PPMFile {
    fn convert_to_ppm_and_save(&self, file_name: String);
}

impl PPMFormat for Canvas {
    fn create_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width(), self.height())
    }

    fn create_termination() -> String {
        String::from("\n")
    }

    fn write_to_ppm(&self, file: &File) {
        for h in 0..self.height() {
            let mut line_array = Vec::<String>::new();
            for w in 0..self.width() {
                let pixel = self.get_pixel(w, h);
                line_array.push(pixel.format_color_string());
            }

            write_line_to_file(file, Canvas::format_pixel_line(line_array));
        }
    }


}

impl PPMFile for Canvas {
    fn convert_to_ppm_and_save(&self, file_name: String) {
        let file = open_new_file(file_name);
        write_line_to_file(&file, self.create_header());
        self.write_to_ppm(&file);
        write_line_to_file(&file, Canvas::create_termination());
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