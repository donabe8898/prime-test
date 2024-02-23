//! Miller-Rabin素数判定法に使うメソッドモジュール

use rug::rand::RandState;
use rug::Integer;

/// Miller-Rabin素数判定法を使った素数判定メソッド
///
/// # 引数
///
/// * `p` - 素数かどうかを判定したい数.
/// * `k` - 素数判定のテスト回数. 多いほど正確に判定される.
///
/// # 例
/// ```
/// let r = is_prime_miller_rabin(Integer::from(13),5);
/// assert_eq!(r, true);
/// ```

#[allow(dead_code)]
pub fn is_prime_miller_rabin(p: Integer, k: u64) -> bool {
    let mut rand = RandState::new();
    let is_prime = |p: &Integer, k: Integer| -> bool {
        if p <= &Integer::from(2) {
            return p == &Integer::from(2);
        }
        if p == &Integer::from(5) {
            return true;
        }
        if p % Integer::from(2) == Integer::from(0) {
            return false;
        }

        let q: Integer = p - Integer::from(1);

        let mut i = Integer::from(0);
        let mut qx = q.clone();
        let s = loop {
            if &qx % Integer::from(2) != Integer::from(0) {
                break i;
            }
            qx >>= 1;
            i += Integer::from(1);
        };

        let t = qx.clone();

        let mut x = k.pow_mod(&t, &p).unwrap();

        if &x == &Integer::from(1) || &x == &q {
            return true;
        }

        let mut r = Integer::from(0);
        let res = loop {
            if r > &s - Integer::from(1) {
                break false;
            }

            x = (x.clone() * x) % p;

            if x == q {
                break true;
            }
            r += Integer::from(1);
        };
        res
    };

    let mut count = Integer::from(0);
    // let mut set = HashSet::new();

    let res = loop {
        if count > k {
            break true;
        }
        let a = random_num(&mut rand, &p);
        // let a = random_num(p.clone());

        if !is_prime(&p, a) {
            break false;
        }
        count += Integer::from(1);
    };
    res
}

/// Integer型の乱数を生成するメソッド
///
/// # 引数
///
/// * `rand` - シード
/// * `p` - 乱数の最大値

pub fn random_num(rand: &mut RandState, p: &Integer) -> Integer {
    let below = p.clone().random_below(rand);
    below
}
