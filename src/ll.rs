use rand::Rng;
use rug::Integer;

pub fn is_prime_lucal_lehmer(p: Integer) -> bool {
    if p <= Integer::from(1) {
        return false;
    }

    if p == Integer::from(2) {
        return true;
    }

    let mut s = Integer::from(4);
    let m = Integer::from(2).pow_mod(&p, &Integer::from(1)).unwrap() - Integer::from(1);
    let mut i = Integer::from(2);

    loop {
        s = (s.clone() * s.clone() - Integer::from(2)) % &m;
        i += Integer::from(1);
        println!("{}", i);
        if i > p {
            break;
        }
    }
    s == Integer::from(0)
}
