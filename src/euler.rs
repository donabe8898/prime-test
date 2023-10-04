// use rug::rand::RandState;
use rug::Integer;

use super::math::pw;

// TODO: ここでオーバーフローか何かが起きていて、動作がとまる。
pub fn is_prime_euler_lagrange(q: Integer) -> bool {
    let p: Integer = (&q - Integer::from(1)) / Integer::from(2);
    if &p % Integer::from(4) == Integer::from(1) {
        ((pw(&Integer::from(2), &p) + Integer::from(1)) % &q) == Integer::from(0)
    } else {
        ((pw(&Integer::from(2), &p) - Integer::from(1)) % &q) == Integer::from(0)
    }
}
