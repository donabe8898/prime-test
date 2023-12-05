use super::euler::is_prime_euler_lagrange;
use super::mr::is_prime_miller_rabin;
use rand::Rng;
use rug::Integer;
use std::time;

/*  MR+MR */
#[allow(dead_code)]
pub fn mr_mr_bench(test_num: Integer) -> (bool, bool) {
    let now: time::Instant = time::Instant::now();
    match is_prime_miller_rabin(test_num.clone(), 6) {
        true => {
            match is_prime_miller_rabin(test_num.clone() * Integer::from(2) + Integer::from(1), 6) {
                true => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {}, {:?}", 0, 0, end);
                    (false, false)
                }
                false => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {:?}, {}", 0, end, 0);
                    (false, true)
                }
            }
        }
        false => {
            let end = now.elapsed().as_secs_f64();
            println!("{:?}, {}, {}", end, 0, 0);
            (true, false)
        }
    }
}

/* MR+EEL */
#[allow(dead_code)]
pub fn mr_eel_bench(test_num: Integer) -> (bool, bool) {
    let now: time::Instant = time::Instant::now();
    match is_prime_miller_rabin(test_num.clone(), 6) {
        true => {
            match is_prime_euler_lagrange(test_num.clone() * Integer::from(2) + Integer::from(1)) {
                true => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {}, {:?}", 0, 0, end);
                    (false, false)
                }
                false => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {:?}, {}", 0, end, 0);
                    (false, true)
                }
            }
        }
        false => {
            let end = now.elapsed().as_secs_f64();
            println!("{:?}, {}, {}", end, 0, 0);
            (true, false)
        }
    }
}
/* EEL+MR */
#[allow(dead_code)]
pub fn eel_mr_bench(test_num: Integer) -> (bool, bool) {
    let now: time::Instant = time::Instant::now();
    match is_prime_euler_lagrange(test_num.clone()) {
        true => {
            match is_prime_miller_rabin(test_num.clone() * Integer::from(2) + Integer::from(1), 6) {
                true => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {}, {:?}", 0, 0, end);
                    (false, false)
                }
                false => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {:?}, {}", 0, end, 0);
                    (false, true)
                }
            }
        }
        false => {
            let end = now.elapsed().as_secs_f64();
            println!("{:?}, {}, {}", end, 0, 0);
            (true, false)
        }
    }
}
/* EEL+EEL */
#[allow(dead_code)]
pub fn eel_eel_bench(test_num: Integer) -> (bool, bool) {
    let now: time::Instant = time::Instant::now();
    match is_prime_euler_lagrange(test_num.clone()) {
        true => {
            match is_prime_euler_lagrange(test_num.clone() * Integer::from(2) + Integer::from(1)) {
                true => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {}, {:?}", 0, 0, end);
                    (false, false)
                }
                false => {
                    let end = now.elapsed().as_secs_f64();
                    println!("{}, {:?}, {}", 0, end, 0);
                    (false, true)
                }
            }
        }
        false => {
            let end = now.elapsed().as_secs_f64();
            println!("{:?}, {}, {}", end, 0, 0);
            (true, false)
        }
    }
}

#[allow(dead_code)]
pub fn mr_bench(keta: u64) -> u64 {
    let mut miss_mr = 0u64;
    loop {
        let i = random_num(keta);

        if &i % Integer::from(2) == Integer::from(0) {
            // 偶数は弾く
            continue;
        }
        // 1回目
        match is_prime_miller_rabin(i.clone(), 12) {
            true => {
                break;
            }
            false => {
                miss_mr += 1;
                continue;
            }
        };
    }
    println!("{}", miss_mr);
    miss_mr
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
