use std::fmt::Debug;

mod problems;

fn get_iter<T>(iterable: T) -> impl Iterator<Item = usize> where
    T: Iterator,
    T::Item: Into<usize> {
    iterable.map(|e| e.into() * 2usize)
}

fn main() {
    for e in get_iter(0usize..10usize) {
        println!("{}", e)
    }
}