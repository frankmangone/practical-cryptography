mod algorithms;
mod curve;
mod hashing;
mod modulo;

use hashing::hash;
use curve::{Point, Curve};
use algorithms::ecdsa::{Signature, sign, verify};

fn main() {
    // ECDSA Signature scheme
    let gen = Point::Coords(2693i128, 4312i128);
    let curve = Curve::new(
        0i128,
        1i128,
        7691i128,
        gen.clone(),
        641,
    );
    let (sk, pk) = curve.keypair();

    let message = "Some message to sign";
    let sig = sign(message, curve.clone(), sk);

    let result = verify(message, sig.clone(), curve.clone(), pk.clone());
    println!("Is signature valid? {}", result);

    let forged_sig = Signature {
        challenge: sig.challenge,
        verifier: 6192i128,
    };
    let result = verify(message, forged_sig.clone(), curve, pk);
    println!("Is forged signature valid? {}", result);
}
