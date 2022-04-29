#![allow(dead_code)]

pub use short::*;

mod long {
    use std::io::stdin;

    fn get_id() -> String {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }

    fn make_surprised(mut x: String) -> String {
        x.push_str("??!");
        x
    }

    pub fn main() {
        println!("{}", make_surprised(get_id()));
    }
}

mod short {
    pub fn main() {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("{}??!", buf.trim());
    }
}