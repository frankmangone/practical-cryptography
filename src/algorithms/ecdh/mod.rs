use crate::modulo::{modulo, inverse};
use crate::curve::{Point, Curve};
use rand::Rng;

pub fn gen_secret(curve: &Curve) -> i128 {
  let ord = &curve.order;
  let mut rng = rand::thread_rng();
  let random_number: i128 = rng.gen();
  modulo(random_number.abs(), *ord)
}

pub fn partial_key(curve: &Curve, sk: i128) -> Point {
  curve.multiply(&curve.gen, sk)
}

pub fn shared_secret(curve: &Curve, partial: &Point, sk: i128) -> Point {
  curve.multiply(partial, sk)
}