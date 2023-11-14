// use super::math::pw;
// use rug::integer::IntegerExt64;
// use rug::rand::RandState;
// use rug::rand;
use rug::Integer;
// use super::math::pw;

// TODO: ここでオーバーフローか何かが起きていて、動作がとまる。
// pub fn is_prime_euler_lagrange_old(q: Integer) -> bool {
//     let p: Integer = (&q - Integer::from(1)) / Integer::from(2);
//     let nm = pw(&Integer::from(2), &p);
//     if &p % Integer::from(4) == Integer::from(1) {
//         //        println!("1");
//         ((nm + Integer::from(1)) % q) == Integer::from(0)
//     } else {
//         // println!("2");
//         ((nm - Integer::from(1)) % q) == Integer::from(0)
//     }
// }

// CHANGED: pow modを使う
pub fn is_prime_euler_lagrange(q: Integer) -> bool {
    let p: Integer = (&q - Integer::from(1)) / Integer::from(2);
    // let nm = pw(&Integer::from(2), &p);
    let nm: Integer = Integer::from(2).pow_mod(&p, &q).unwrap();
    if &p % Integer::from(4) == Integer::from(1) {
        //        println!("1");
        ((nm + Integer::from(1)) % q) == Integer::from(0)
    } else {
        // println!("2");
        ((nm - Integer::from(1)) % q) == Integer::from(0)
    }
}
