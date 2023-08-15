mod mr;

use mr::is_prime_miller_rabin;
use rug::{Assign, Integer};

fn main() {
    let mut int = Integer::new();
    let decimal = "13";
    int.assign(Integer::parse(decimal).unwrap());

    let p = Integer::from(int);
    let k = Integer::from(4);

    match is_prime_miller_rabin(p.clone(), k.clone()) {
        true => println!("=====\n{} is Prime\n=====", &p),
        false => println!("=====\n{} is Not Prime\n=====", &p),
    };
}
