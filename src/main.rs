mod algorithms;
mod curve;
mod hashing;
mod modulo;

use curve::{Point, Curve};
use algorithms::{
    ecdsa::{Signature, sign, verify},
    ecies::{encrypt, decrypt},
    ecdh as ECDH,
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
    ecdh(&curve);
    ecdsa(&curve);
    ecies(&curve);
}

//
//
// ECDH (Elliptic Curve Diffie-Hellman) key exchange
fn ecdh(curve: &Curve) {
    println!("ECDH algorithm");
    println!("================================================");
    println!("================================================");

    let a = ECDH::gen_secret(curve);
    let b = ECDH::gen_secret(curve);

    println!("Alice's secret is: {}", a);
    println!("Bob's secret is: {}", b);

    let share_a = ECDH::partial_key(curve, a);
    let share_b = ECDH::partial_key(curve, b);

    println!("Alice calculates share and sends to Bob: {:?}", share_a);
    println!("Bob calculates share and sends to Alice: {:?}", share_b);

    let secret_a = ECDH::shared_secret(curve, &share_b, a);
    let secret_b = ECDH::shared_secret(curve, &share_a, b);

    println!("Alice calculates secret with Bob's share: {:?}", secret_a);
    println!("Bob calculates secret with Alice's share: {:?}", secret_b);

    if secret_a == secret_b {
        println!("The secrets match; key exchange successful.");
    } else {
        println!("The secrets don't match; key exchange failed.");
    }

    println!(".\n.\n.\n.\n.\n.\n.\n.");
}

//
//
// ECDSA (Elliptic Curve Digital Signature Algorithm) scheme
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
// ECIES (Elliptic Curve Integrated Encryption Scheme)
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