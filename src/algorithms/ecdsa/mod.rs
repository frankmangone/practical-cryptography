use crate::modulo::{modulo, inverse};
use crate::curve::{Point, Curve};
use crate::hashing::hash;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Signature {
  pub challenge: i128,
  pub verifier: i128,
}

pub fn sign(message: &str, curve: Curve, sk: i128) -> Signature {
  // We should really be using Euler's Totient Function, but whatever...
  let ord = curve.order;

  // Select random nonce
  let mut rng = rand::thread_rng();
  let random_number: i128 = rng.gen();
  let k = modulo(random_number.abs(), ord);

  // Compute R = [k]G
  let r_ = curve.multiply(&curve.gen, k);
  
  match r_ {
    Point::Identity => panic!("Signature produces identity"),
    Point::Coords(r, _) => {
      // Compute s
      let a = modulo(inverse(k, ord), ord);
      let b = modulo(hash(message, ord) + sk * r, ord); // FIXME: This could cause overflow problems

      let s = modulo(a * b, ord);
    
      Signature {
        challenge: r,
        verifier: s,
      }
    }
  }
}

pub fn verify(message: &str, signature: Signature, curve: Curve, pk: Point) -> bool {
  let Signature { challenge: r, verifier: s } = signature;
  let ord = curve.order;

  let w = inverse(s, ord);

  let u_1 = hash(message, ord) * w;
  let u_2 = r * w;

  let u_1 = curve.multiply(&curve.gen, u_1);
  let u_2 = curve.multiply(&pk, u_2);
  let v_ = curve.add(&u_1, &u_2);

  match v_ {
    Point::Identity => false,
    Point::Coords(v, _) => {
      if v == r {
        true
      } else {
        false
      }
    }
  }
}