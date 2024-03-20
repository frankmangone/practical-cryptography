use crate::curve::{g, double, add, Point};

#[test]
fn doubling_works() {
  assert_eq!(double(g), Point::Coords(4641, 1628));
}

#[test]
fn add_works() {
  assert_eq!(add(g, Point::Coords(4641, 1628)), Point::Coords(2473, 5808));
}

#[test]
fn multiple_additions_equals_multiplication() {
  let g4 = double(double(g));
  let g4_prime = add(add(add(g, g), g), g);

  assert_eq!(g4, g4_prime);
}