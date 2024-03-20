mod tests;

use rand::Rng;
use crate::modulo::{modulo, inverse};

#[derive(Clone, Debug, PartialEq)]
pub enum Point {
  Identity,
  Coords(i128, i128),
}

pub fn negative(p: &Point) -> Point {
  match p {
    Point::Identity => Point::Identity,
    Point::Coords(x, y) => Point::Coords(*x, -*y),
  }
}

#[derive(Clone)]
pub struct Curve {
  a: i128,
  b: i128,
  pub q: i128,
  pub gen: Point,
  pub order: i128,
}

impl Curve {
  pub fn new(a: i128, b: i128, q: i128, gen: Point, order: i128) -> Self {
    match gen {
      Point::Identity => panic!("Generator cannot be identity"),
      Point::Coords(_, _) => {
        Curve {
          a,
          b,
          q,
          gen,
          order,
        }
      }  
    }
  }

  pub fn keypair(&self) -> (i128, Point) {
    let mut rng = rand::thread_rng();
    let random_number: i64 = rng.gen();
    
    let sk = random_number.abs();
    let pk = self.multiply(&self.gen, sk.into());

    (sk.into(), pk)
  }

  pub fn add(&self, p_: &Point, q_: &Point) -> Point {
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

  pub fn double(&self, p_: &Point) -> Point {
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

  pub fn multiply(&self, p_: &Point, m: i128) -> Point {
    if m == 0 { return Point::Identity; }
    
    let _m = m.abs();
    let mut first_significant = true;
    
    let mut r_ = Point::Identity;

    for i in (0..128).rev() {
      let bit = (_m >> i) % 2;

      if bit == 1 && first_significant {
        first_significant = false;
        
        if m < 0 {
          r_ = negative(p_);
        } else {
          r_ = p_.clone();
        }
        continue;
      }

      r_ = self.double(&r_);

      if bit == 1 {
        r_ = self.add(&r_, &p_);
      }
    }

    r_
  }
}
