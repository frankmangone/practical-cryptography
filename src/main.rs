mod modulo;
mod curve;

use modulo::{modulo, inverse};
use curve::{add, double, g};

fn main() {
    println!("{}", modulo(90, 8));
    println!("{}", modulo(-90, 8));
    
    println!("{}", inverse(1, 7));
    println!("{}", inverse(2, 7));
    println!("{}", inverse(3, 7));
    println!("{}", inverse(4, 7));

    println!("{:?}", g);
    println!("{:?}", add(g, double(g)));
    println!("{:?}", add(g, add(g, double(g))));
    println!("{:?}", double(double(g)));
}
