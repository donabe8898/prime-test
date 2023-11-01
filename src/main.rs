// use rand::Rng;
// use rug::{rand::RandState, Integer};
mod euler;
mod ll;
mod math;
mod mr;
mod test;
mod yaml;
// extern crate yaml_rust;

use euler::is_prime_euler_lagrange;
use euler::is_prime_euler_lagrange_old;
// use euler::is_prime_euler_lagrange2;
use mr::is_prime_miller_rabin;
// use mylib::math::pw;
use rand::Rng;
use rug::rand::RandState;
use rug::Assign;
use rug::Integer;
use std::time;
use std::time::Instant;
use test::mr_eel_bench;
use test::mr_mr_bench;
// use yaml::load_yaml;
// use yaml_rust::{YamlEmitter, YamlLoader};

/*
    - 素数を100個、素数じゃない奇数を100個用意して、それぞれのアルゴリズムで判定。
    - 判定にかかった時間と正解率のコスパが良いものはどれか
    - 1024〜4096 bit
*/

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

fn main() /*-> Result<(), Box<dyn std::error::Error>> */
{
    /*　生成個数 */
    let piece = 1000;
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
    ];

    /* 計測用整数生成
    zeroとtmpはシャドーイング
     */
    // 1024 bit
    let zero = Integer::from(0);
    let mut tmp = vec![zero; piece];
    let test1024 = {
        let mut cnt = 0;
        'outer: while cnt < piece {
            let r = test::random_num(1024);

            for i in primes {
                if &r % Integer::from(i) == Integer::from(0) {
                    continue 'outer;
                }
            }

            tmp[cnt] = r;
            cnt += 1;
        }
        tmp
    };
    println!("Ok"); // HACK: debug

    // 2048 bit
    let zero = Integer::from(0);
    let mut tmp = vec![zero; piece];

    let test2048 = {
        let mut cnt = 0;
        'outer: while cnt < piece {
            let r = test::random_num(2048);

            for i in primes {
                if &r % Integer::from(i) == Integer::from(0) {
                    continue 'outer;
                }
            }

            tmp[cnt] = r;
            cnt += 1;
        }
        tmp
    };

    println!("Ok"); // HACK: debug

    // 4096 bit
    let zero = Integer::from(0);
    let mut tmp = vec![zero; piece];
    let test4096 = {
        let mut cnt = 0;
        'outer: while cnt < piece {
            let r = test::random_num(4096);

            for i in primes {
                if &r % Integer::from(i) == Integer::from(0) {
                    continue 'outer;
                }
            }
            tmp[cnt] = r;
            cnt += 1;
        }
        tmp
    };

    println!("Ok"); // HACK: debug

    /* TODO: MR_MR計測 */
    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut mr_mr_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::mr_mr_bench(test.clone()) {
                (false, false) => mr_mr_miss.2 += 1,
                (true, false) => mr_mr_miss.0 += 1,
                (false, true) => mr_mr_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!("MRMR {:?}\n{:?}", end, mr_mr_miss);
    }
    /* TODO: MR_EEL計測 */
    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut mr_eel_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::mr_eel_bench(test.clone()) {
                (false, false) => mr_eel_miss.2 += 1,
                (true, false) => mr_eel_miss.0 += 1,
                (false, true) => mr_eel_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!("MREEL {:?}\n{:?}", end, mr_eel_miss);
    }
    /* TODO: EEL_MR計測 */
    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut mr_eel_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::eel_mr_bench(test.clone()) {
                (false, false) => mr_eel_miss.2 += 1,
                (true, false) => mr_eel_miss.0 += 1,
                (false, true) => mr_eel_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!("EEL+MR {:?}\n{:?}", end, mr_eel_miss);
    }
    /* TODO: EEL_EEL計測 */
    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut mr_eel_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::eel_eel_bench(test.clone()) {
                (false, false) => mr_eel_miss.2 += 1,
                (true, false) => mr_eel_miss.0 += 1,
                (false, true) => mr_eel_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!("EEL+EEL {:?}\n{:?}", end, mr_eel_miss);
    }
    // let m = 2;
    // let mut rand = RandState::new();
    // for _ in 0..100 {
    //     println!("{}", mr::random_num(&mut rand, Integer::from(16)));
    // }
    // println!("{:?}", ll::gen_lucas(5));

    // let mut int = Integer::new();
    // let decimal = "13331";
    // int.assign(Integer::parse(decimal).unwrap());

    // let primes: [u32; 20] = [
    //     3259, 3271, 3299, 3301, 3307, 3313, 3319, 3323, 3329, 3331, 3343, 3347, 3359, 3361, 3371,
    //     3373, 3389, 3391, 3407, 3413,
    // ];

    // let p = Integer::from(int);
    // println!("{}", euler::is_prime_euler_lagrange(p));
    // let pp = (&p - Integer::from(1)) / Integer::from(2);

    // println!(
    //     "{},{}",
    //     is_prime_miller_rabin(p.clone(), 12),
    //     is_prime_euler_lagrange(p.clone())
    // );

    // println!("{}", Integer::from(2).pow_mod(&pp, &p).unwrap());

    // // TODO: 本番
    // let path = "./test_num.yaml";
    // let docs = load_yaml(&path);
    // let doc = &docs[0];

    // // TODO: miller-rabinテストの実行時間計測
    // // TODO: リュカ-レーマ-テストの実行時間計測

    // // HACK: yamlのテストコード
    // let path = "./test_num.yaml";
    // let docs = load_yaml(&path);
    // let doc = &docs[0];
    // // numberを取り出す
    // let number = &doc[0]["number"];
    // // is_primeを取り出す
    // let is_prime = &doc[0]["is_prime"];

    // for data in doc.clone() {
    //     println!(
    //         "{}, {}",
    //         data["number"].as_str().unwrap(),
    //         data["is_prime"].as_bool().unwrap()
    //     );
    // }

    // HACK: 乱数生成用テストコード
    // let mut idx = 0;
    // let mut rand = RandState::new();
    // while (idx < 100) {
    //     let mut i = Integer::from(Integer::random_bits(4096, &mut rand));

    //     // if &i % Integer::from(2) != 0 {
    //     //     idx += 1;
    //     // }
    //     idx += 1;
    //     println!("{}", idx);
    // }
    // HACK: 判定テスト用

    // for i in (2..55500).step_by(2) {
    //     let p = &p + Integer::from(i);

    //     if is_prime_euler_lagrange(p.clone()) != is_prime_euler_lagrange2(p.clone()) {
    //         println!("{}", p);
    //     }

    //     match is_prime_miller_rabin(Integer::from(p), 6) {
    //         true => println!("=====\n{} is Prime (MR)\n=====", &p),
    //         false => println!("=====\n{} is Not Prime (MR)\n=====", &p),
    //     };
    //     match is_prime_euler_lagrange(Integer::from(p.clone())) {
    //         true => println!("=====\n{} is Prime\n=====", &p),
    //         false => println!("=====\n{} is Not Prime\n=====", &p),
    //     }
    //     match is_prime_euler_lagrange2(Integer::from(p.clone())) {
    //         true => println!("=====\n{} is Prime (2)\n=====", &p),
    //         false => println!("=====\n{} is Not Prime (2)\n=====", &p),
    //     }
    // }
    // // 17 -> 19
    // let lucas = gen_lucas(20);
    // for i in 0..100 {
    //     match is_prime_lucal_lehmer(p.clone(), lucas.clone) {
    //         true => println!("=====\n{} is Prime\n=====", &p),
    //         false => println!("=====\n{} is Not Prime\n=====", &p),
    //     }
    // }

    // Ok(())
}
