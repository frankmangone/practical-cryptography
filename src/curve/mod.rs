mod tests;

use crate::modulo::{modulo, inverse};

#[derive(Clone, Debug, PartialEq)]
pub enum Point {
  Identity,
  Coords(i128, i128),
}

pub struct Curve {
  a: i128,
  b: i128,
  q: i128,
  gen: Point
}

impl Curve {
  fn new(a: i128, b: i128, q: i128, gen: Point) -> Self {
    match gen {
      Point::Identity => panic!("Generator cannot be identity"),
      Point::Coords(_, _) => {
        Curve {
          a,
          b,
          q,
          gen,
        }
      }  
    }
  }

  fn add(&self, p_: &Point, q_: &Point) -> Point {
    if p_ == q_ {
      return self.double(p_);
    }
  
    match p_ {
      Point::Identity => q_.clone(),
      Point::Coords(x_p, y_p) => {
        match q_ {
          Point::Identity => p_.clone(),
          Point::Coords(x_q, y_q) => {
            // Ensure we are not adding inverse points
            if x_p == x_q && y_p != y_q {
              return Point::Identity;
            }
            
            let slope = modulo((y_p - y_q) * inverse(x_p - x_q, self.q), self.q);
            
            let x_r = modulo(slope * slope - x_p - x_q, self.q);
            let y_r = modulo(slope * (x_p - x_r) - y_p, self.q);

            Point::Coords(x_r, y_r)
          }
        }
      }
    }
  }

  fn double(&self, p_: &Point) -> Point {
    match p_ {
      Point::Identity => Point::Identity,
      Point::Coords(x_p, y_p) => {
        // Check if the slope would be infinite (y_p == 0)
        if y_p == &0 {
          return Point::Identity;
        }
  
        let slope = modulo((3 * x_p * x_p + self.a) * inverse(2 * y_p, self.q), self.q);
  
        let x_r = modulo(slope * slope - 2 * x_p, self.q);
        let y_r = modulo(slope * (x_p - x_r) - y_p, self.q);
  
        Point::Coords(x_r, y_r)
      }
    }
  }
}

// Elliptic curve params
// const a: i128 = 0i128;
// const b: i128 = 1i128;
// pub const q: i128 = 7691i128;
// pub const g: Point = Point::Coords(2693i128, 4312i128);
