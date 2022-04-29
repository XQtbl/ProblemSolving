use std::io::{stdin, prelude::*};
use std::str::FromStr;
use std::fmt::Debug;

pub mod p2588;
pub mod p10926;
pub mod p1929;
pub mod p2447;
pub mod brute_force {
    pub mod p2798;
    pub mod p2231;
}
pub mod sorting {
    pub mod p2750;
}

#[allow(dead_code)]
fn get_inputs<T: FromStr>() -> Vec<T>
    where <T as FromStr>::Err: Debug {
    let cin = stdin();
    let mut inputs = String::new();
    cin.lock().read_to_string(&mut inputs).unwrap();
    inputs.trim().split_whitespace().map(|n| {
        n.parse::<T>().expect("Failed to parse")
    }).collect()
}

