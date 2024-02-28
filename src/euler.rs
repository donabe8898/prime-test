//! オイラーラグランジュの定理を使ったやつ

use rug::Integer;
use crate::test;

/// オイラーラグランジュの定理を使った素数判定
///
/// # 引数
///
/// * `q` -  素数かどうかを判定したい数. 多倍長整数型
///
/// # 例
///
/// ```
/// let r = is_prime_euler_lagrange(Integer::from(13));
/// assert_eq!(r, true);
/// ```
///
/// # 使用箇所
///
/// - [test::mr_eel_bench](test::mr_eel_bench)
/// - [test::eel_mr_bench](test::eel_mr_bench)
/// - [test::eel_eel_bench](test::eel_eel_bench)
pub fn is_prime_euler_lagrange(q: Integer) -> bool {
    let p: Integer = (&q - Integer::from(1)) / Integer::from(2);
    let nm: Integer = Integer::from(2).pow_mod(&p, &q).unwrap();
    if &p % Integer::from(4) == Integer::from(1) {
        ((nm + Integer::from(1)) % q) == Integer::from(0)
    } else {
        ((nm - Integer::from(1)) % q) == Integer::from(0)
    }
}
