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

/* アノテーションこめんど
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
    /*　100個生成テスト用 */
    let mut mr_mr_miss: [(u64, u64); 100] = [(0, 0); 100];
    let mut mr_eel_miss: [(u64, u64); 100] = [(0, 0); 100];
    // mr_mr 計測開始
    let now: time::Instant = time::Instant::now();
    for i in 0..100 {
        mr_mr_miss[i] = mr_mr_bench(64);
    }
    let end = now.elapsed();
    println!("time: {:?}\n{:?}", end, mr_mr_miss);

    // mr_eel 計測開始
    let now = time::Instant::now();
    for i in 0..100 {
        mr_eel_miss[i] = mr_eel_bench(64);
    }
    let end = now.elapsed();
    println!("time: {:?}\n{:?}", end, mr_eel_miss);
    // for _ in 0..20 {
    //     println!("{:?}", test::mr_eel_bench(64));
    // }

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
