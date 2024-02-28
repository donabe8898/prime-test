//! # 「大きな安全素数の生成速度に関する研究」研究用プログラム
//!
//! #　備考
//! - GMPのインストールが必須です。UNIXであればパッケージマネージャーから落としてきてください
//!
//! - Apple Siliconのmacでは動作しない場合があります
mod euler;
mod mr;
mod test;

use rug::Integer;
use std::time;

/* アノテーションこめんと

TODO: 	あとで追加、修正するべき機能がある。
FIXME: 	既知の不具合があるコード。修正が必要。
HACK: 	あまりきれいじゃないコード。リファクタリングが必要。
XXX: 	危険！動くけどなぜうごくかわからない。
REVIEW: 	意図した通りに動くか、見直す必要がある。
OPTIMIZE: 	無駄が多く、ボトルネックになっている。
CHANGED: 	コードをどのように変更したか。
NOTE: 	なぜ、こうなったという情報を残す。
WARNING: 	注意が必要。

 */

// ============================================================
/// 「大きな安全素数の生成速度に関する研究」
/// 1024bit, 2048bit, 4096bitの素数候補（130個の素数で試行割算済み）を
/// 素数判定し、一番速い組み合わせと、素数候補の数を見る
fn main() {
    /*　生成個数 */
    let piece = 300000;
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733,
    ];

    // 1024 bit
    let test1024: Vec<Integer> = return_rands(1024, piece, primes);
    println!("1024 OK!");

    // 2048 bit
    let test2048: Vec<Integer> = return_rands(2048, piece, primes);
    println!("2048 OK!");

    // 4096 bit
    let test4096: Vec<Integer> = return_rands(4096, piece, primes);
    println!("4096 OK!");

    /* NOTE: MR_MR計測 */
    let mut safe_primes: Vec<Integer> = Vec::new();

    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::mr_mr_bench(test.clone()) {
                (false, false) => {
                    safe_primes.push(test.clone());
                    test_miss.2 += 1
                }
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end: time::Duration = now.elapsed();
        println!(
            "\n ========= \nMR_MR\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }

    println!("safe_primes ===================");
    if !safe_primes.is_empty() {
        for safe_prime in safe_primes {
            print!("-> ");
            println!("{}", safe_prime);
        }
    }
    println!("safe_primes end ===================");

    /* NOTE:  MR_EEL計測 */
    let mut safe_primes: Vec<Integer> = Vec::new();

    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::mr_eel_bench(test.clone()) {
                (false, false) => {
                    safe_primes.push(test.clone());
                    test_miss.2 += 1
                }
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end: time::Duration = now.elapsed();
        println!(
            "\n ========= \nMR_EEL\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }

    println!("safe_primes ===================");
    if !safe_primes.is_empty() {
        for safe_prime in safe_primes {
            print!("-> ");
            println!("{}", safe_prime);
        }
    }
    println!("safe_primes end ===================");

    /* NOTE: EEL_MR計測 */
    let mut safe_primes: Vec<Integer> = Vec::new();

    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::eel_mr_bench(test.clone()) {
                (false, false) => {
                    safe_primes.push(test.clone());
                    test_miss.2 += 1
                }
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end: time::Duration = now.elapsed();
        println!(
            "\n ========= \nEEL_MR\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }

    println!("safe_primes ===================");
    if !safe_primes.is_empty() {
        for safe_prime in safe_primes {
            print!("-> ");
            println!("{}", safe_prime);
        }
    }
    println!("safe_primes end ===================");

    /* NOTE: EEL_EEL計測 */
    let mut safe_primes: Vec<Integer> = Vec::new();

    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::eel_eel_bench(test.clone()) {
                (false, false) => {
                    safe_primes.push(test.clone());
                    test_miss.2 += 1
                }
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end: time::Duration = now.elapsed();
        println!(
            "\n ========= \nEEL_EEL\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }

    println!("safe_primes ===================");
    if !safe_primes.is_empty() {
        for safe_prime in safe_primes {
            print!("-> ");
            println!("{}", safe_prime);
        }
    }
    println!("safe_primes end ===================");
}

fn return_rands(bits: u64, piece: usize, primes: [i32; 130]) -> Vec<Integer> {
    let zero = Integer::from(0);
    let mut tmp = vec![zero; piece.try_into().unwrap()];
    let mut cnt = 0;
    'outer: while cnt < piece {
        let r = test::random_num(bits);

        for i in primes {
            if &r % Integer::from(i) == Integer::from(0) {
                continue 'outer;
            }
        }

        tmp[cnt] = r;
        cnt += 1;
    }
    tmp
}
