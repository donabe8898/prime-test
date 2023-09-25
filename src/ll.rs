// use rand::Rng;
use rug::Integer;

// use std::collections::btree_set::Intersection;
// use std::collections::HashSet;
// use std::hash::Hash;

// 1: メルセンヌ数かどうかの判定 t,f
// 2: sの値の列挙

#[allow(dead_code)]
pub fn is_prime_lucal_lehmer(c: Integer, lucas: Vec<Integer>) -> bool {
    if &c <= &Integer::from(1) || !(&c.get_bit(0)) {
        return false;
    }

    if c == Integer::from(2) {
        return true;
    }

    // 1: そもそもメルセンヌ数かどうかの判定
    let next_pow = c.clone().next_power_of_two();
    let m = &next_pow - Integer::from(1);
    if m != c {
        return false;
    }

    // 2: s値の列挙
    let p = next_pow.clone().find_one(0).unwrap(); // 証明では2^pのp
    let lucasu = lucas;
    // println!("{}", lucasu.len());
    if &lucasu[p as usize - 2] % m == Integer::from(0) {
        return true;
    } else {
        return false;
    }

    // loop {
    //     let x = gen_prime_num(&i);
    //     if x {
    //         let m = pw(&Integer::from(2), &i) - Integer::from(1);
    //         if m == p {
    //             return true;
    //         }

    //         if m > p {
    //             return false;
    //         }
    //     }
    //     i += Integer::from(1);
    // }
}

#[allow(dead_code)]
pub fn gen_lucas(p: u32) -> Vec<Integer> {
    let mut s: Integer = Integer::from(4);
    // let m: Integer = pw(&Integer::from(2), &Integer::from(p)) - Integer::from(1);
    let mut i: Integer = Integer::from(0);

    let mut lucas: Vec<Integer> = Vec::new();
    lucas.push(s.clone());
    loop {
        s = &s * &s - Integer::from(2);
        // println!("{:?}", s);
        lucas.push(s.clone());

        i += Integer::from(1);
        if i >= Integer::from(p) {
            break;
        }
    }
    lucas
}

// pub fn gen_prime_num(p: &Integer) -> bool {
//     let mut s = Integer::from(4);
//     let m = pw(&Integer::from(2), p) - Integer::from(1);
//     let mut i = Integer::from(2);

//     loop {
//         s = (s.clone() * s.clone() - Integer::from(2)) % &m;

//         i += Integer::from(1);
//         if i >= *p {
//             break;
//         }
//     }
//     s == Integer::from(0)
// }

#[allow(dead_code)]
pub fn pw(a: &Integer, b: &Integer) -> Integer {
    let mut ret: Integer = Integer::from(1);
    let mut x: Integer = a.clone();
    let mut n: Integer = b.clone();
    let res = loop {
        if n <= Integer::from(0) {
            break ret;
        }
        let p = &n & Integer::from(1);
        if p == Integer::from(1) {
            ret *= &x;
        }
        x *= x.clone();
        n >>= 1;
    };
    res
}
