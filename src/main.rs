mod algorithms;
mod curve;
mod hashing;
mod modulo;

use curve::{Point, Curve};
use algorithms::{
    ecdsa::{Signature, sign, verify},
    ecies::{encrypt, decrypt},
};

fn main() {
    //
    // Curve definition
    let gen = Point::Coords(2693i128, 4312i128);
    let curve = Curve::new(
        0i128,
        1i128,
        7691i128,
        gen.clone(),
        641,
    );

    // Algorithms to run
    println!(".\n.\n.\n.\n.\n.\n.\n.");
    ecdsa(&curve);
    ecies(&curve);
}

//
//
// ECDSA Signature scheme
fn ecdsa(curve: &Curve) {
    println!("ECDSA algorithm");
    println!("================================================");
    println!("================================================");

    let (sk, pk) = curve.keypair();

    let message = "Hola mama!";
    println!("Message to sign: {}", message);

    let sig = sign(message, curve.clone(), sk);
    println!("Calculated signature: {:?}", sig.clone());

    let result = verify(message, sig.clone(), curve.clone(), pk.clone());
    println!("Is signature valid? {}", result);

    let forged_sig = Signature {
        challenge: sig.challenge,
        verifier: 6192i128,
    };
    println!("Forged (simulated verifier) signature: {:?}", forged_sig.clone());
    let result = verify(message, forged_sig.clone(), curve.clone(), pk.clone());
    println!("Is forged signature valid? {}", result);

    println!(".\n.\n.\n.\n.\n.\n.\n.");
}

//
//
// ECIES encryption scheme
fn ecies(curve: &Curve) {
    println!("ECIES algorithm");
    println!("================================================");
    println!("================================================");

    let message = "Hola mundo!";
    let message_bytes = message.as_bytes();

    let (sk, pk) = curve.keypair();
    let (key, masked) = encrypt(message_bytes, curve.clone(), pk);
    
    println!("Original message is: {:?}", message);
    println!("Message bytes are: {:?}", message_bytes);
    println!("Encrypted message is {:?}", masked.clone());
    println!("Key for decryption is {:?}", key.clone());

    let decrypted = decrypt(&masked, curve.clone(), &key, sk);

    println!("Decrypted bytes are: {:?}", decrypted);
    println!("Decrypted message is: {:?}", String::from_utf8(decrypted.to_vec()).unwrap());

    println!(".\n.\n.\n.\n.\n.\n.\n.");
}