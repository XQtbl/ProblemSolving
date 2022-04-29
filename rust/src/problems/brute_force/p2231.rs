// N = 생성자 M + M%10 + M%10^2 + ...
// 1 <= N <= 100_0000 --> u32
// input: N, out: min(M1, M2, ...)
// M-1 이상의 생성자 N은 존재 불가능: 가장 작은 M은 N+1이 되기 때문.
// 계산해 볼만한 N의 범위 = [1, M-1] --> 자리당 최대 9이므로 좌범위를 조정 가능할 듯함.

pub use legacy::*;

mod legacy {
    use std::fmt::Debug;
    use std::io::{stdin, stdout, prelude::*};
    use std::str::FromStr;

    fn get_n<T>() -> T
        where T: FromStr, <T as FromStr>::Err: Debug
    {
        let cin = stdin();
        let mut line = String::new();
        stdin().lock().read_line(&mut line).unwrap();
        line.trim().parse().expect("Failed to parse")
    }

    fn get_min_m(n: u32) -> u32 {
        let min_m = (1..n-1)
            .map(|cndt| {
                (cndt, cndt.to_string().as_bytes().iter().fold(cndt, |acc, ch| acc + (ch - b'0') as u32))
            })
            .find(|(_, sum)| *sum == n);
        match min_m {
            Some((m, _)) => m,
            None => 0,
        }
    }

    pub fn main() {
        let n = get_n();

        let cout = stdout();
        let mut cout = cout.lock();
        writeln!(&mut cout, "{}", get_min_m(n)).unwrap();
    }
}

mod improving {
    use std::fmt::{Debug, Display, Formatter};
    use std::io::{stdin, stdout, prelude::*};
    use std::str::FromStr;

    #[derive(Clone)]
    struct DecimalUInt<const N: usize = 0> {
        data: [u8; N]
    }

    impl<const N: usize> DecimalUInt<N> {
        fn new() -> Self {
            Self { data: [0; N] }
        }
    }

    impl<T, const N: usize> From<T> for DecimalUInt<N>
        where T: Into<u64>
    {
        fn from(target: T) -> Self {
            let target_uint = target.into();

            if target_uint == 0 {
                return Self::new();
            }

            let target_f32 = target_uint as f32;
            let digit_cnt = (target_f32.log10().floor() + 1.0) as u32;

            let mut data = [0u8; N];
            for (idx, digit) in (1..=digit_cnt).map(|exp| {
                let divisor = 10u64.pow(exp);
                target_uint % divisor / (divisor / 10)
            }).enumerate() {
                data[idx] = digit as u8;
            }
            Self { data }
        }
    }

    impl<const N: usize> ToString for DecimalUInt<N> {
        fn to_string(&self) -> String {
            self.data.iter()
                .map(|&ch| char::from(ch))
                .collect()
        }
    }

    // impl<const N: usize> TryInto<u64> for DecimalUInt<N> {
    //     type Error = ();
    //     fn try_into(self) -> Result<u64, Self::Error> {
    //         u64::from_str(self.to_string().as_str())
    //     }
    // }

    fn get_n<T>() -> T
        where T: FromStr, <T as FromStr>::Err: Debug
    {
        let cin = stdin();
        let mut line = String::new();
        cin.lock().read_line(&mut line).unwrap();
        line.trim().parse().expect("Failed to parse")
    }

    fn get_min_m(n: u32) -> u32 {
        let min_m = (1..n - 1)
            .map(|cndt| {
                (cndt, cndt.to_string().as_bytes().iter().fold(cndt, |acc, ch| acc + (ch - b'0') as u32))
            })
            .find(|(_, sum)| *sum == n);
        match min_m {
            Some((m, _)) => m,
            None => 0,
        }
    }

    pub fn main() {
        println!("{}", DecimalUInt::<2>::from(32u32).to_string());

        let n = get_n();

        let cout = stdout();
        let mut cout = cout.lock();
        writeln!(&mut cout, "{}", get_min_m(n)).unwrap();
    }
}