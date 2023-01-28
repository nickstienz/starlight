mod primitives;
mod utils;

use primitives::point::Point;
use primitives::tuple::Tuple;
use primitives::vector::Vector;

fn main() {
    let p = Point::new(3.0, 1.0, 8.0);
    let v = Vector::new(2.0, 3.0, -5.0);
    let t = v + p;
    println!("{}, {}, {}", t.x(), t.y(), t.z());
}
