//! mr.rsとeuler.rsを組み合わせてテストするモジュール

use super::euler::is_prime_euler_lagrange;
use super::mr::is_prime_miller_rabin;
use rand::Rng;
use rug::Integer;
use std::time;

/// 素数候補`p`に対してMiller-Rabinテスト, 安全素数候補`2p+1`に対してもMiller-Rabinテストを行う.
///
///
/// 1回目および2回目のテストで合成数だと判定されたらその時点で終了.
///
///
/// 2回目のテストもクリアすれば **素数かもしれない** と判定して終了.
///
/// # 引数
///
/// * `test_num` - 安全素数かどうかを判定したい値
///
///
/// # 出力
///
/// 1つの素数候補を判定し終わった時間が標準出力される.
/// 出力結果を保存したい場合は`$ cargo run --release > hogehoge.txt`で渡す(UNIX系OSの場合)
///
/// # 戻り値
///
/// ```rust
/// (true, false)   // 1回目のテストで素数ではないと判定された（合成数）
/// (false, true)   // 2回目のテストで素数ではないと判定された(素数かもしれないが, 2p+1は素数ではない)
/// (false, false)  // p, 2p+1ともに素数かもしれないと判定された
/// ```
///
/// # 例
///
/// ```
/// let safe = match test::mr_mr_bench(Integer::from(5)) {
///     (false, false) => true
///     (true, false) => false,
///     (false, true) => false,
///     (_, _) => {false}
/// }
/// // 2*5+1=11は素数なので11は安全素数
/// assert_eq!(safe,true);
/// ```


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

/// 素数候補`p`に対してMiller-Rabinテスト, 安全素数候補`2p+1`に対してEuler-lagrangeテストを行う.
///
///
/// 1回目および2回目のテストで合成数だと判定されたらその時点で終了.
///
///
/// 2回目のテストもクリアすれば **素数かもしれない** と判定して終了.
///
///
/// # 引数
///
/// * `test_num` - 安全素数かどうかを判定したい値
///
///
/// # 出力
///
/// 1つの素数候補を判定し終わった時間が標準出力される.
/// 出力結果を保存したい場合は`$ cargo run --release > hogehoge.txt`で渡す(UNIX系OSの場合)
///
/// # 戻り値
///
/// ```rust
/// (true, false)   // 1回目のテストで素数ではないと判定された（合成数）
/// (false, true)   // 2回目のテストで素数ではないと判定された(素数かもしれないが, 2p+1は素数ではない)
/// (false, false)  // p, 2p+1ともに素数かもしれないと判定された
/// ```
///
/// # 例
///
/// ```
/// let not_safe = match test::mr_eel_bench(Integer::from(13)) {
///     (false, false) => false
///     (true, false) => false,
///     (false, true) => true,
///     (_, _) => {false}
/// }
///
/// assert_eq!(not_safe, true);
/// ```


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


/// 素数候補`p`に対してEuler-lagrangeテスト, 安全素数候補`2p+1`に対してMiller-Rabinテストを行う.
///
///
/// 1回目および2回目のテストで合成数だと判定されたらその時点で終了.
///
///
/// 2回目のテストもクリアすれば **素数かもしれない** と判定して終了.
///
///
/// # 引数
///
/// * `test_num` - 安全素数かどうかを判定したい値
///
///
/// # 出力
///
/// 1つの素数候補を判定し終わった時間が標準出力される.
/// 出力結果を保存したい場合は`$ cargo run --release > hogehoge.txt`で渡す(UNIX系OSの場合)
///
/// # 戻り値
///
/// ```rust
/// (true, false)   // 1回目のテストで素数ではないと判定された（合成数）
/// (false, true)   // 2回目のテストで素数ではないと判定された(素数かもしれないが, 2p+1は素数ではない)
/// (false, false)  // p, 2p+1ともに素数かもしれないと判定された
/// ```
///
/// # 例
///
/// ```
/// let not_prime = match test::ell_mr_bench(Integer::from(12)) {
///     (false, false) => false
///     (true, false) => true,
///     (false, true) => false,
///     (_, _) => {false}
/// }
///
/// assert_eq!(not_prime, true);
/// ```


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


/// 素数候補`p`に対してEuler-lagrangeテスト, 安全素数候補`2p+1`に対してもEuler-lagrangeテストを行う.
///
///
/// 1回目および2回目のテストで合成数だと判定されたらその時点で終了.
///
///
/// 2回目のテストもクリアすれば **素数かもしれない** と判定して終了.
///
/// # 引数
///
/// * `test_num` - 安全素数かどうかを判定したい値
///
///
/// # 出力
///
/// 1つの素数候補を判定し終わった時間が標準出力される.
/// 出力結果を保存したい場合は`$ cargo run --release > hogehoge.txt`で渡す(UNIX系OSの場合)
///
/// # 戻り値
///
/// ```rust
/// (true, false)   // 1回目のテストで素数ではないと判定された（合成数）
/// (false, true)   // 2回目のテストで素数ではないと判定された(素数かもしれないが, 2p+1は素数ではない)
/// (false, false)  // p, 2p+1ともに素数かもしれないと判定された
/// ```
///
/// # 例
///
/// ```
/// let not_prime = match test::ell_eel_bench(Integer::from(21)) {
///     (false, false) => false
///     (true, false) => true,
///     (false, true) => false,
///     (_, _) => {false}
/// }
///
/// assert_eq!(not_prime, true);
/// ```


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

/// miller-rabinテストのデバッグ用コード
///
/// 現在は使用されない
///
/// # 引数
///
/// * `keta` - 桁数
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

/// rug::Integer型の乱数を返すメソッド
///
/// # 引数
/// * `bit` - 乱数の上限桁数（2進数に変換したときの桁数）
///
/// # 例
/// ```
/// let m = random_num(3);
/// // 0b100 ~ 0b111
/// assert_eq!(true, (Integer::from(4) <= m && m <= Integer::from(7)))
/// ```
///

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

    // 2進数表記から10進数へ変換
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

/// べき乗の計算をするメソッド
///
/// rug::Integer型同士の累乗計算ができる.
///
/// # 引数
/// * `a` - 底
/// * `b` - 冪指数
///
/// # 例
///
/// ```
/// let p = pw(Integer::from(2), Integer::from(2));
/// assert_eq!(4, p);
/// ```
///

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
