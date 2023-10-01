// TODO: テストベンチ用関数の作成

// pub mod ll;
// mod mr;

// use std::num::NonZeroI128;

// use crate::ll::gen_prime_num;
// use super::ll::is_prime_lucal_lehmer;
use super::mr::is_prime_miller_rabin;
// use ll::is_prime_lucal_lehmer;
// use mr::is_prime_miller_rabin;
// use rand::Rng;
use rand::Rng;
// use rug::rand::RandState;
use rug::Integer;

// use crate::ll::gen_lucas;
// use crate::test;
// use yaml::load_yaml;
// use yaml_rust::{YamlEmitter, YamlLoader};

#[allow(dead_code)]
pub fn mr_mr_bench(keta: u64) -> (u64, u64) {
    // let mut idx = 0;
    let mut miss_mr_one = 0u64;
    let mut miss_mr_two = 0u64;
    // 100個生成
    //while idx < 10 {
    loop {
        let i = random_num(keta);

        if &i % Integer::from(2) == Integer::from(0) {
            // 偶数は弾く
            continue;
        }
        // 1回目
        match is_prime_miller_rabin(i.clone(), 12) {
            true => match is_prime_miller_rabin(i * Integer::from(2) + Integer::from(1), 12) {
                true => {}
                false => {
                    miss_mr_two += 1;
                    continue;
                }
            },
            false => {
                miss_mr_one += 1;
                continue;
            }
        }
        // 上が成功したらbreak
        break;
        // print!("{}", is_prime_miller_rabin(i.clone(), Integer::from(4)));
    }
    // idx += 1;
    // }
    println!("{}, {}", miss_mr_one, miss_mr_two);
    (miss_mr_one, miss_mr_two)
}

pub fn random_num(bit: u64) -> Integer {
    let mut str_res: String = String::new();
    // 初期状態で最大bitを1にする
    str_res += "1";

    for _ in 1..bit {
        let num: i64 = rand::thread_rng().gen_range(1, 8);
        if num % 2 == 1 {
            str_res += "1";
        } else {
            str_res += "0";
        }
    }
    // 一旦逆順にする
    let rev_str: &String = &str_res;
    let rev_str = rev_str.chars().rev().collect::<String>();
    // 2^n計算 ラムダ式使える?
    let mut res = Integer::from(0);
    let mut i = 0;
    for c in rev_str.chars() {
        if c == '1' {
            res += pw(&Integer::from(2), &Integer::from(i));
        }
        i += 1;
    }
    res

    // 戻り値
}

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
