mod algorithms;
mod curve;
mod hashing;
mod modulo;

use hashing::hash;

fn main() {
    println!("{}", hash("hola!", 7691i128));
}
