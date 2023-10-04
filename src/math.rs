use rug::Integer;

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
