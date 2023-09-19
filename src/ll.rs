use rand::Rng;
use rug::Integer;
use std::collections::btree_set::Intersection;

pub fn is_prime_lucal_lehmer(p: Integer) -> bool {
    if &p <= &Integer::from(1) || !(&p.get_bit(0)) {
        return false;
    }

    if p == Integer::from(2) {
        return true;
    }

    let mut i = Integer::from(1);

    loop {
        let x: bool = gen_prime_num(&i);
        if x {
            let m = pw(&Integer::from(2), &i) - Integer::from(1);
            if m == p {
                return true;
            }

            if m > p {
                return false;
            }
        }
        i += Integer::from(1);
    }
}

pub fn gen_prime_num(p: &Integer) -> bool {
    let mut s = Integer::from(4);
    let m = pw(&Integer::from(2), p) - Integer::from(1);
    let mut i = Integer::from(2);

    loop {
        s = (s.clone() * s.clone() - Integer::from(2)) % &m;

        i += Integer::from(1);
        if i >= *p {
            break;
        }
    }
    s == Integer::from(0)
}

pub fn pw(a: &Integer, b: &Integer) -> Integer {
    let mut count: Integer = Integer::from(0);
    let mut memory: Integer = Integer::from(1);
    let res = loop {
        if count == *b {
            break memory;
        }
        memory *= a;
        count += Integer::from(1);
    };
    res
}
