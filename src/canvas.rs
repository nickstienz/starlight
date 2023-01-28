use crate::primitives::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    buffers: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            buffers: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = y * self.width + x;
        self.buffers[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let index = self.width * y + x;
        self.buffers[index]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixel_at(x, y);
                ppm.push_str(&format!(
                    "{} {} {} ",
                    (color.r() * 255.0).round() as u8,
                    (color.g() * 255.0).round() as u8,
                    (color.b() * 255.0).round() as u8
                ));

                if x % 5 == 4 && x != self.width - 1 {
                    ppm.push_str("\n");
                }
            }
            ppm.push_str("\n");
        }
        ppm
    }

    pub fn to_ppm_file(&self, filename: &str) {
        let ppm = self.to_ppm();
        std::fs::write(filename, ppm).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        for y in 0..c.height() {
            for x in 0..c.width() {
                let color = c.pixel_at(x, y);
                assert_eq!(color.r(), 0.0);
                assert_eq!(color.g(), 0.0);
                assert_eq!(color.b(), 0.0);
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        let color = c.pixel_at(2, 3);
        assert_eq!(color.r(), 1.0);
        assert_eq!(color.g(), 0.0);
        assert_eq!(color.b(), 0.0);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split('\n').collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split('\n').collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 ");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 ");
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for y in 0..c.height() {
            for x in 0..c.width() {
                c.write_pixel(x, y, color);
            }
        }
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split('\n').collect();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 "
        );
        assert_eq!(
            lines[4],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 "
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 "
        );
        assert_eq!(
            lines[6],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 "
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split('\n').collect();
        assert_eq!(lines[lines.len() - 1], "");
    }
}
