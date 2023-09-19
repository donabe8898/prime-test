// TODO: テストベンチ用関数の作成

// pub mod ll;
// mod mr;

// use crate::ll::gen_prime_num;
use super::ll::is_prime_lucal_lehmer;
use super::mr::is_prime_miller_rabin;
// use ll::is_prime_lucal_lehmer;
// use mr::is_prime_miller_rabin;
use rand::Rng;
use rug::rand::RandState;
use rug::{Assign, Integer};

use crate::test;
// use yaml::load_yaml;
// use yaml_rust::{YamlEmitter, YamlLoader};

pub fn mr_bench(keta: u64) {
    let mut idx = 0;
    let mut rand = RandState::new();
    while idx < 100 {
        // loop {
        let i = Integer::from(Integer::random_bits(keta.try_into().unwrap(), &mut rand));

        if &i % Integer::from(2) == Integer::from(0) {
            // print!("[Even]");
            continue;
        }
        // print!("{}", is_prime_miller_rabin(i.clone(), Integer::from(4)));

        // }
        idx += 1;
    }
}

pub fn llr_bench(keta: u64) {
    let mut idx = 0;
    let mut rand = RandState::new();
    while idx < 100 {
        // loop {
        let i = Integer::from(Integer::random_bits(keta.try_into().unwrap(), &mut rand));

        if &i % Integer::from(2) == Integer::from(0) {
            // print!("[Even]");
            continue;
        }
        // print!("{}", is_prime_lucal_lehmer(i.clone()));

        // }
        idx += 1;
    }
}
