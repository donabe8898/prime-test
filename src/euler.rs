use rug::Integer;

pub fn is_prime_euler_lagrange(q: Integer) -> bool {
    let p: Integer = (&q - Integer::from(1)) / Integer::from(2);
    let nm: Integer = Integer::from(2).pow_mod(&p, &q).unwrap();
    if &p % Integer::from(4) == Integer::from(1) {
        ((nm + Integer::from(1)) % q) == Integer::from(0)
    } else {
        ((nm - Integer::from(1)) % q) == Integer::from(0)
    }
}
