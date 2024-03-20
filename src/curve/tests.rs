use crate::curve::{negative, Curve, Point};

fn test_curve() -> Curve {
  let gen = Point::Coords(2693i128, 4312i128);
  let curve = Curve::new(
    0i128,
    1i128,
    7691i128,
    gen.clone(),
    641,
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

#[test]
fn multiplication_works() {
  let curve = test_curve();
  let gen = &curve.gen;

  let g2_minus = curve.double(&negative(gen));
  let g2 = curve.double(gen);
  let g3 = curve.add(gen, &g2);
  let g4 = curve.add(gen, &g3);
  let g5 = curve.add(gen, &g4);

  assert_eq!(g2_minus, curve.multiply(gen, -2));
  assert_eq!(negative(gen), curve.multiply(gen, -1));
  assert_eq!(Point::Identity, curve.multiply(gen, 0));
  assert_eq!(gen, &curve.multiply(gen, 1));
  assert_eq!(g2, curve.multiply(gen, 2));
  assert_eq!(g3, curve.multiply(gen, 3));
  assert_eq!(g4, curve.multiply(gen, 4));
  assert_eq!(g5, curve.multiply(gen, 5));
}

#[test]
fn multiplication_by_group_order() {
  let curve = test_curve();
  let gen = &curve.gen;
  let order = curve.order;

  assert_eq!(*gen, curve.multiply(gen, order + 1));
}
