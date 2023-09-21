mod ll;
mod mr;
mod yaml;
extern crate yaml_rust;

use crate::ll::gen_lucas;
use ll::is_prime_lucal_lehmer;
use mr::is_prime_miller_rabin;
use rand::Rng;
use rug::rand::RandState;
use rug::{Assign, Integer};
use yaml::load_yaml;
use yaml_rust::{YamlEmitter, YamlLoader};

/*
    - 素数を100個、素数じゃない奇数を100個用意して、それぞれのアルゴリズムで判定。
    - 判定にかかった時間と正解率のコスパが良いものはどれか
    - 1024〜4096 bit
*/

fn main() /*-> Result<(), Box<dyn std::error::Error>> */
{
    let mut int = Integer::new();
    let decimal = "524287";
    int.assign(Integer::parse(decimal).unwrap());

    let p = Integer::from(int);
    let k = Integer::from(4);

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
    match is_prime_miller_rabin(p.clone(), k.clone()) {
        true => println!("=====\n{} is Prime\n=====", &p),
        false => println!("=====\n{} is Not Prime\n=====", &p),
    };
    // 17 -> 19
    let lucas = gen_lucas(19);
    match is_prime_lucal_lehmer(p.clone(), lucas) {
        true => println!("=====\n{} is Prime\n=====", &p),
        false => println!("=====\n{} is Not Prime\n=====", &p),
    }

    // Ok(())
}
