use crate::modulo::{modulo, inverse};
use crate::curve::{Point, Curve};
use rand::Rng;

fn random() -> i128 {
  let mut rng = rand::thread_rng();
  let k: i128 = rng.gen();
  k.abs()
}

// Returns two curve generators
pub fn setup(curve: &Curve) -> (Point, Point) {
  let k: i128 = random();
  
  let gen_1 = &curve.gen;
  let gen_2 = curve.multiply(&gen_1, k);

  (gen_1.clone(), gen_2.clone())
}

// Commits a value, using some generated randomness, that is also an output
// of this function
pub fn commit(curve: &Curve, generators: &(Point, Point), value: i128) -> (Point, i128) {
  let r: i128 = random();
  let (g, h) = generators;

  let bind = curve.multiply(g, value);
  let hide = curve.multiply(h, r);

  let commitment = curve.add(&bind, &hide);
  (commitment, r)
}

// Commits a value, using some generated randomness, that is also an output
// of this function
pub fn open(
  curve: &Curve,
  generators: &(Point, Point),
  commitment: &Point,
  value: i128,
  randomness: i128
) -> bool {
  let (g, h) = generators;

  let bind = curve.multiply(g, value);
  let hide = curve.multiply(h, randomness);

  let calculated = curve.add(&bind, &hide);
  
  commitment == &calculated
}