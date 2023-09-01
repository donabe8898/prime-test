mod ll;
mod mr;

use ll::is_prime_lucal_lehmer;
use mr::is_prime_miller_rabin;
use rug::{Assign, Integer};

fn main() {
    let mut int = Integer::new();
    let decimal = "531137992816767098689588206552468627329593117727031923199444138200403559860852242739162502265229285668889329486246501015346579337652707239409519978766587351943831270835393219031728127";
    int.assign(Integer::parse(decimal).unwrap());

    let p = Integer::from(int);
    let k = Integer::from(4);

    match is_prime_miller_rabin(p.clone(), k.clone()) {
        true => println!("=====\n{} is Prime\n=====", &p),
        false => println!("=====\n{} is Not Prime\n=====", &p),
    };
    match is_prime_lucal_lehmer(p.clone()) {
        true => println!("=====\n{} is Prime\n=====", &p),
        false => println!("=====\n{} is Not Prime\n=====", &p),
    }
}
