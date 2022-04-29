// n: 3~100 --> 카드 개수
// m: 10~300000 --> 목표 합
// 3개 골라 m에 최대한 가깝게
// 최대 100C3 = 100 * 99 * 98 / (3!) < 1000000 --> 모든 경우 계산해도 1초 안에 가능.
// Permutation logic: [2][1][0],

pub use improved::*;

mod legacy {
    use std::cmp::max;
    use std::io::{stdin, stdout, prelude::*};

    fn get_input() -> (usize, Vec<usize>) {
        let cin = stdin();
        let mut cin = cin.lock();
        let mut input_data_string = String::new();
        cin.read_to_string(&mut input_data_string).unwrap();
        let mut input_data_iter = input_data_string.trim().split_whitespace();

        let n = input_data_iter.next().expect("Got no input").parse().expect("Failed to parse n");
        let m = input_data_iter.next().expect("Got only 1 input").parse().expect("Failed to parse m");
        let cards = input_data_iter.take(n).map(|s| s.parse().expect("Failed to parse card")).collect();
        (m, cards)
    }

    #[derive(Debug)]
    struct Combination3<'d, T> {
        data: &'d [T],
        thres: [usize; 3],
        next_idx: [usize; 3],
    }

    impl<'d, T> Combination3<'d, T> {
        fn new(data: &'d [T]) -> Self {
            assert!(data.len() >= 3);
            Self { data, thres: [data.len() - 2, data.len() - 1, data.len()], next_idx: [0, 1, 2] }
        }
    }

    impl<'d, T: Copy> Iterator for Combination3<'d, T> {
        type Item = [T; 3];

        fn next(&mut self) -> Option<Self::Item> {
            if self.next_idx[0] >= self.thres[0] {
                return None;
            }

            let ret = self.next_idx.map(|idx| self.data[idx]);

            self.next_idx[2] += 1;
            if self.next_idx[2] >= self.thres[2] {
                self.next_idx[1] += 1;
                self.next_idx[2] = self.next_idx[1] + 1;

                if self.next_idx[1] >= self.thres[1] {
                    self.next_idx[0] += 1;
                    self.next_idx[1] = self.next_idx[0] + 1;
                    self.next_idx[2] = self.next_idx[1] + 1;
                }
            }

            Some(ret)
        }
    }

    pub fn main() {
        let (m, cards) = get_input();

        let c3_it = Combination3::new(cards.as_slice());

        let mut result = 0;
        for c3 in c3_it {
            let candidate = c3.iter().sum();
            if candidate > m {
                continue;
            }
            result = max(result, candidate);
        }

        let cout = stdout();
        let mut cout = cout.lock();
        writeln!(&mut cout, "{}", result).unwrap();
    }
}

mod improved {
    use std::cmp::max;
    use std::fmt::Debug;
    use std::io::{stdin, stdout, prelude::*};
    use std::str::FromStr;

    fn get_input<T>() -> (T, Vec<T>)
        where T: FromStr, <T as FromStr>::Err: Debug {
        let cin = stdin();
        let mut cin = cin.lock();
        let mut input_data_string = String::new();
        cin.read_to_string(&mut input_data_string).unwrap();
        let mut input_data_iter = input_data_string.trim().split_whitespace();

        let n = input_data_iter.next().expect("Got no input").parse().expect("Failed to parse n");
        let m = input_data_iter.next().expect("Got only 1 input").parse().expect("Failed to parse m");
        let cards = input_data_iter.take(n).map(|s| s.parse().expect("Failed to parse card")).collect();
        (m, cards)
    }

    fn get_result(m: usize, data: &[usize]) -> usize {
        let mut result = 0;
        for (i, &first_choice) in data[0..].iter().enumerate() {
            for (j, &second_choice) in data[i+1..].iter().enumerate() {
                for &third_choice in data[i+1 + j+1..].iter() {
                    let candidate = first_choice + second_choice + third_choice;
                    if m >= candidate {
                        result = max(result, candidate);
                    }
                }
            }
        }
        result
    }

    pub fn main() {
        let (m, cards) = get_input();

        let mut result = get_result(m, cards.as_slice());

        let cout = stdout();
        let mut cout = cout.lock();
        writeln!(&mut cout, "{}", result).unwrap();
    }
}