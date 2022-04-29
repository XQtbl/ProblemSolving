#![allow(dead_code)]

use std::io::{stdin, prelude::*};

fn read() -> [u8; 3] {
    let mut asciies = [0; 3];

    let mut cin = stdin();
    cin.read_exact(&mut asciies).unwrap();
    cin.take(1).bytes().last();

    asciies.map(|c| c - b'0')
}

fn digits_into_int(x: &[u8; 3]) -> u16 {
    x.iter().fold(0, |acc, n| 10 * acc + *n as u16)
}

pub fn main() {
    let a = read(); // Max: 9 --> u8
    let b = read();

    let a_as_uint = digits_into_int(&a); // Max: 999 --> u16
    let products = b.map(|x| a_as_uint * x as u16); // Max: 999 * 9 < 10000 --> u16


    for x in products.iter().rev() {
        println!("{}", x);
    }

    let result = products.iter().fold(0u32, |acc, n| 10 * acc + *n as u32); //  Max: 999 * 999 < 1,000,000 --> u32
    println!("{}", result);
}