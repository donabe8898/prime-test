mod ll;
mod mr;

use crate::ll::gen_prime_num;
use ll::is_prime_lucal_lehmer;
use mr::is_prime_miller_rabin;
use rand::Rng;
use rug::rand::RandState;
use rug::{Assign, Integer};

/*
    - 素数を100個、素数じゃない奇数を100個用意して、それぞれのアルゴリズムで判定。
    - 判定にかかった時間と正解率のコスパが良いものはどれか
    - 1024〜4096 bit
*/

fn main() {
    let mut int = Integer::new();
    let decimal = "1475979915214180235084898622737381736312066145333169775147771216478570297878078949377407337049389289382748507531496480477281264838760259191814463365330269540496961201113430156902396093989090226259326935025281409614983499388222831448598601834318536230923772641390209490231836446899608210795482963763094236630945410832793769905399982457186322944729636418890623372171723742105636440368218459649632948538696905872650486914434637457507280441823676813517852099348660847172579408422316678097670224011990280170474894487426924742108823536808485072502240519452587542875349976558572670229633962575212637477897785501552646522609988869914013540483809865681250419497686697771007";
    int.assign(Integer::parse(decimal).unwrap());

    let p = Integer::from(int);
    let k = Integer::from(4);

    // HACK: 乱数生成用テストコード
    let mut rand = RandState::new();
    // let mut i = Integer::from(Integer::random_bits(1024, &mut rand));
    // println!("{}", i);
    for i in 0..10 {
        let mut i = Integer::from(Integer::random_bits(1024, &mut rand));
        i.assign(Integer::random_bits(1024, &mut rand));
        println!("\n {}", i);
    }
    // HACK: 判定テスト用
    // match is_prime_miller_rabin(p.clone(), k.clone()) {
    //     true => println!("=====\n{} is Prime\n=====", &p),z
    //     false => println!("=====\n{} is Not Prime\n=====", &p),
    // };

    // match is_prime_lucal_lehmer(p.clone()) {
    //     true => println!("=====\n{} is Prime\n=====", &p),
    //     false => println!("=====\n{} is Not Prime\n=====", &p),
    // }
}
