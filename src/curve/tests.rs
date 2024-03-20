use crate::curve::{Curve, Point};

fn test_curve() -> Curve {
  let gen = Point::Coords(2693i128, 4312i128);
  let curve = Curve::new(
    0i128,
    1i128,
    7691i128,
    gen.clone(),
  );

  curve
}

#[test]
fn doubling_works() {
  let curve = test_curve();
  let gen = &curve.gen;

  assert_eq!(curve.double(gen), Point::Coords(4641, 1628));
}

#[test]
fn add_works() {
  let curve = test_curve();
  let gen = &curve.gen;

  assert_eq!(curve.add(gen, &Point::Coords(4641, 1628)), Point::Coords(2473, 5808));
}

#[test]
fn multiple_additions_equals_multiplication() {
  let curve = test_curve();
  let gen = &curve.gen;

  let g4 = curve.double(&curve.double(gen));
  let g4_prime = curve.add(&curve.add(&curve.add(gen, gen), gen), gen);

  assert_eq!(g4, g4_prime);
}