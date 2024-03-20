mod tests;

use crate::modulo::{modulo, inverse};

#[derive(Debug, PartialEq)]
pub enum Point {
  Identity,
  Coords(i128, i128),
}

// Elliptic curve params
const a: i128 = 0i128;
const b: i128 = 1i128;
pub const q: i128 = 7691i128;
pub const g: Point = Point::Coords(2693i128, 4312i128);

pub fn add(p_: Point, q_: Point) -> Point {
  if p_ == q_ {
    return double(p_);
  }

  match p_ {
    Point::Identity => q_,
    Point::Coords(x_p, y_p) => {
      match q_ {
        Point::Identity => p_,
        Point::Coords(x_q, y_q) => add_points((x_p, y_p), (x_q, y_q))
      }
    }
  }
}

fn add_points(p_: (i128, i128), q_: (i128, i128)) -> Point {
  let (x_p, y_p) = p_;
  let (x_q, y_q) = q_;
  
  let slope = modulo((y_p - y_q) * inverse(x_p - x_q, q), q);
	let ord = modulo((y_q * x_p - y_p * x_q) * inverse(x_p - x_q, q), q);

	let x_r = modulo(x_p + x_q - slope * slope, q);
	let y_r = modulo(0 - slope * x_r - ord, q);

	Point::Coords(x_r, y_r)
}

pub fn double(p_: Point) -> Point {
  match p_ {
    Point::Identity => Point::Identity,
    Point::Coords(x_p, y_p) => {
      let slope = modulo((3 * x_p + a) * inverse(2 * y_p, q), q);
      let ord = modulo(y_p - slope * x_p, q);

      let x_r = modulo(slope * slope - 2 * x_p, q);
      let y_r = modulo(0 - slope * x_r - ord, q);

      Point::Coords(x_r, y_r)
    }
  }
}
