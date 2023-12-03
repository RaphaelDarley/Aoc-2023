fn main() {
    let input = include_str!("../input");

    let sum: usize = input
        .lines()
        .map(NumsParse::nums_parse)
        .map(Iterator::collect::<Vec<usize>>)
        .map(|nums| {
            let first = nums.first().unwrap();
            let last = nums.last().unwrap();
            first * 10 + last
        })
        .sum();
    println!("sum: {sum}");
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
struct NumsParser {
    inner: String,
}

impl Iterator for NumsParser {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for (i, d) in DIGITS.iter().enumerate() {
            if let Some(_) = self.inner.strip_prefix(d) {
                self.inner = self.inner.chars().skip(1).collect();
                return Some(i);
            }
        }

        for (i, d) in ('0'..='9').into_iter().enumerate() {
            if let Some(_) = self.inner.strip_prefix(d) {
                self.inner = self.inner.chars().skip(1).collect();
                return Some(i);
            }
        }
        self.inner = self.inner.chars().skip(1).collect();
        if self.inner.is_empty() {
            None
        } else {
            self.next()
        }
    }
}

trait NumsParse {
    fn nums_parse(self) -> NumsParser
    where
        Self: Sized;
}

impl<T: ToString> NumsParse for T {
    fn nums_parse(self) -> NumsParser
    where
        Self: Sized,
    {
        NumsParser {
            inner: self.to_string(),
        }
    }
}

// impl<T: Iterator<Item = char>> NumsParse for T {
//     fn nums_parse(self) -> NumsParser
//     where
//         Self: Sized,
//     {
//         let string = self.collect();
//         println!("making nums parser from:{}", string);
//         NumsParser { inner: string }
//     }
// }
