use crate::curve::{g, double, add, Point};

#[test]
fn doubling_works() {
  assert_eq!(double(g), Point::Coords(4641, 1628));
}

#[test]
fn add_works() {
  assert_eq!(add(g, Point::Coords(4641, 1628)), Point::Coords(2473, 5808));
}