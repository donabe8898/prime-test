mod euler;
mod ll;
mod mr;
mod test;

use rug::Integer;
use std::time;

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
    let piece = 12000;
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541,
    ];

    /*
       計測用整数生成
       zeroとtmpはシャドーイング
    */

    // HACK: 512bit
    // let test512 = return_rands(512, piece, primes);
    // println!("512 OK!");

    // 1024 bit
    let test1024 = return_rands(1024, piece, primes);
    println!("1024 OK!");

    // 2048 bit
    let test2048 = return_rands(2048, piece, primes);
    println!("2048 OK!");

    // 4096 bit
    let test4096 = return_rands(4096, piece, primes);
    println!("4096 OK!");

    /* NOTE: MR_MR計測 */
    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::mr_mr_bench(test.clone()) {
                (false, false) => test_miss.2 += 1,
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!(
            "\n ========= \nMR_MR\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }
    /* NOTE:  MR_EEL計測 */
    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::mr_eel_bench(test.clone()) {
                (false, false) => test_miss.2 += 1,
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!(
            "\n ========= \nMR_EEL\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }
    /* NOTE: EEL_MR計測 */
    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::eel_mr_bench(test.clone()) {
                (false, false) => test_miss.2 += 1,
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!(
            "\n ========= \nEEL_MR\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }
    /* NOTE: EEL_EEL計測 */

    for tests in [&test1024, &test2048, &test4096] {
        let now: time::Instant = time::Instant::now();
        let mut test_miss = (0, 0, 0); // (1回目でmiss, 2回目でmiss, missなし)
        for test in tests {
            match test::eel_eel_bench(test.clone()) {
                (false, false) => test_miss.2 += 1,
                (true, false) => test_miss.0 += 1,
                (false, true) => test_miss.1 += 1,
                (_, _) => {}
            }
        }
        let end = now.elapsed();
        println!(
            "\n ========= \nEEL_EEL\n time: {:?}, miss: {:?}\n========= ",
            end, test_miss
        );
    }
}

fn return_rands(bits: u64, piece: usize, primes: [i32; 100]) -> Vec<Integer> {
    let zero = Integer::from(0);
    let mut tmp = vec![zero; piece.try_into().unwrap()];
    let mut cnt = 0;
    'outer: while cnt < piece {
        let r = test::random_num(bits);

        for i in primes {
            if &r % Integer::from(i) == Integer::from(0) {
                continue 'outer;
            }
        }

        tmp[cnt] = r;
        cnt += 1;
    }
    tmp
}
