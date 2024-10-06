mod algorithms;
mod curve;
mod hashing;
mod modulo;

use curve::{Point, Curve};
use algorithms::{
    ecdsa as Ecdsa,
    ecies as Ecies,
    ecdh as Ecdh,
    pedersen as Pedersen,
};

fn main() {
    let p = 7i16;

    println!("{}", 4i16 % p);  // Expected output: 4
    println!("{}", 13i16 % p); // Expected output: 6
    println!("{}", 0i16 % p);  // Expected output: 0
    println!("{}", -5i16 % p); // Expected output: 2

    //
    // Curve definition
    // let gen = Point::Coords(2693i128, 4312i128);
    // let curve = Curve::new(
    //     0i128,
    //     1i128,
    //     7691i128,
    //     gen.clone(),
    //     641,
    // );

    // // Algorithms to run
    // println!(".\n.\n.\n.\n.\n.\n.\n.");
    // ecdsa(&curve);
    // ecies(&curve);
    // ecdh(&curve);
    // pedersen(&curve);
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

    let sig = Ecdsa::sign(message, curve.clone(), sk);
    println!("Calculated signature: {:?}", sig.clone());

    let result = Ecdsa::verify(message, sig.clone(), curve.clone(), pk.clone());
    println!("Is signature valid? {}", result);

    let forged_sig = Ecdsa::Signature {
        challenge: sig.challenge,
        verifier: 6192i128,
    };
    println!("Forged (simulated verifier) signature: {:?}", forged_sig.clone());
    let result = Ecdsa::verify(message, forged_sig.clone(), curve.clone(), pk.clone());
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
    let (key, masked) = Ecies::encrypt(message_bytes, curve.clone(), pk);
    
    println!("Original message is: {:?}", message);
    println!("Message bytes are: {:?}", message_bytes);
    println!("Encrypted message is {:?}", masked.clone());
    println!("Key for decryption is {:?}", key.clone());

    let decrypted = Ecies::decrypt(&masked, curve.clone(), &key, sk);

    println!("Decrypted bytes are: {:?}", decrypted);
    println!("Decrypted message is: {:?}", String::from_utf8(decrypted.to_vec()).unwrap());

    println!(".\n.\n.\n.\n.\n.\n.\n.");
}

//
//
// ECDH (Elliptic Curve Diffie-Hellman) key exchange
fn ecdh(curve: &Curve) {
    println!("ECDH algorithm");
    println!("================================================");
    println!("================================================");

    let a = Ecdh::gen_secret(curve);
    let b = Ecdh::gen_secret(curve);

    println!("Alice's secret is: {}", a);
    println!("Bob's secret is: {}", b);

    let share_a = Ecdh::partial_key(curve, a);
    let share_b = Ecdh::partial_key(curve, b);

    println!("Alice calculates share and sends to Bob: {:?}", share_a);
    println!("Bob calculates share and sends to Alice: {:?}", share_b);

    let secret_a = Ecdh::shared_secret(curve, &share_b, a);
    let secret_b = Ecdh::shared_secret(curve, &share_a, b);

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
// Pedersen commitment scheme
fn pedersen(curve: &Curve) {
    println!("Pedersen commitment scheme");
    println!("================================================");
    println!("================================================");

    let gens = Pedersen::setup(curve);

    println!("Setup outputs the following curve generators: {:?}, {:?}", &gens.0, &gens.1);

    let value = 120i128;
    let (com, r) = Pedersen::commit(curve, &gens, value);
    
    println!("Commitment is {:?}", com.clone());

    let valid_1 = Pedersen::open(curve, &gens, &com, value, r);
    let valid_2 = Pedersen::open(curve, &gens, &com, 121i128, r);
    let valid_3 = Pedersen::open(curve, &gens, &com, value, 10i128);

    println!("Is the commitment opened with value {} and randomness {}? {}", value, r, valid_1);
    println!("Is the commitment opened with value {} and randomness {}? {}", 121i128, r, valid_2);
    println!("Is the commitment opened with value {} and randomness {}? {}", value, 10i128, valid_3);
        
    println!(".\n.\n.\n.\n.\n.\n.\n.");
}