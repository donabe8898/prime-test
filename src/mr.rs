use std::collections::HashSet;

use rand::Rng;
use rug::Integer;

#[allow(dead_code)]
pub fn is_prime_miller_rabin(p: Integer, k: Integer) -> bool {
    let is_prime = |p: Integer, k: Integer| -> bool {
        if (p < Integer::from(2)) || (&p % Integer::from(2) == Integer::from(0)) {
            return false;
        }
        if &p <= &Integer::from(3) {
            return true;
        }

        let q: Integer = &p - Integer::from(1);

        let mut i = Integer::from(0);
        let mut qx = q.clone();
        let s = loop {
            if &qx % Integer::from(2) != Integer::from(0) {
                break i;
            }
            qx >>= 1;
            i += Integer::from(1);
        };

        let t = qx.clone();

        let mut x = k.pow_mod(&t, &p).unwrap();

        if x == Integer::from(1) || x == q {
            return true;
        }

        let mut r = Integer::from(0);
        let res = loop {
            if r > &s - Integer::from(1) {
                break false;
            }
            x = (x.clone() * x) % &p;
            if x == q {
                break true;
            }
            r += Integer::from(1);
        };

        res
    };

    let mut count = Integer::from(0);
    let mut set = HashSet::new();
    let res = loop {
        if count > k {
            break true;
        }
        let a = loop {
            let i = random_num(p.clone());
            if !set.contains(&i) {
                set.insert(i.clone());
                break i;
            }
        };
        // let a = random_num(p.clone());

        if !is_prime(p.clone(), a) {
            break false;
        }
        count += Integer::from(1);
    };
    res
}

pub fn random_num(p: Integer) -> Integer {
    let num: i64 = rand::thread_rng().gen_range(1, 9_213_372_036_854_775_001);
    let num = Integer::from(num);

    if &p > &num {
        (&p - &num).into()
    } else {
        if num.clone() % &p == Integer::from(0) {
            num.clone() % &p + Integer::from(1)
        } else {
            num % &p
        }
    }
}
