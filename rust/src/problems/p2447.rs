#![allow(dead_code)]

use std::io::{stdin, stdout, prelude::*};
use std::fmt::Debug;
use std::str::FromStr;

fn get_input_by_line<T: FromStr>() -> T
    where <T as FromStr>::Err: Debug {
    let cin = stdin();
    let mut line = String::new();
    cin.read_line(&mut line).unwrap();
    line.trim().parse::<T>().expect("Failed to parse")
}

fn make_canvas(n: usize) -> Vec<Vec<char>> {
    vec![vec!['*'; n]; n]
}

fn print_square(canvas: &mut [Vec<char>], pos: (usize, usize), size: usize) {
    // (x, y, size) --> (x + size/3, y + size/3) ~ (x + size * 2/3, y + size * 2/3)
    if size == 1 {
        return;
    }
    for i in (0..size/3).map(|i| i + pos.0 + size/3) {
        for j in (0..size / 3).map(|j| j + pos.1 + size / 3) {
            canvas[i][j] = ' ';
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            print_square(canvas, (pos.0 + size / 3 * i, pos.1 + size / 3 * j), size / 3);
        }
    }
}

fn print_canvas(canvas: &[Vec<char>]) {
    let cout = stdout();
    let mut cout = cout.lock();
    for line in canvas {
        for pixel in line {
            write!(&mut cout, "{}", pixel).unwrap();
        }
        writeln!(&mut cout).unwrap();
    }
}

pub fn main() {
    let n: usize = get_input_by_line();
    let mut canvas = make_canvas(n);
    print_square(canvas.as_mut(), (0, 0), n);
    print_canvas(canvas.as_ref());
}