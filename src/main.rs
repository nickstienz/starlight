#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
mod canvas;
mod primitives;
mod utils;

use canvas::Canvas;
use primitives::color::Color;
use primitives::point::Point;
use primitives::tuple::Tuple;
use primitives::vector::Vector;

fn main() {
    let mut c = Canvas::new(900, 550);

    let mut position = Point::new(0.0, 1.0, 0.0);
    let mut velocity = Vector::new(1.0, 1.5, 0.0).normalize() * 11.25;
    let gravity = Vector::new(0.0, -0.12, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);

    loop {
        position = position + velocity;
        velocity = velocity + gravity + wind;

        if position.y() < 0.0
            || position.x() < 0.0
            || position.x() > c.width() as f64
            || position.y() > c.height() as f64
        {
            println!("Position: {:?}", position);
            break;
        }

        c.write_pixel(
            position.x() as usize,
            c.height() - position.y() as usize,
            Color::new(1.0, 0.0, 0.0),
        );
    }

    c.to_ppm_file("render.ppm")
}
