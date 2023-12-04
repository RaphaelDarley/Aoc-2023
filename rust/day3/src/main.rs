const INPUT: &str = include_str!("../input");

fn main() {
    let mut schematic: Schematic = INPUT.into();

    println!(
        "{}",
        schematic
            .symbols
            .iter()
            .map(|r| r
                .iter()
                .map(|b| if b.is_symbol { '*' } else { '-' })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );

    let serials = Serial::str_to_serials(INPUT);

    println!("{:?}", serials);

    let total: usize = Serial::str_to_serials(INPUT)
        .iter()
        .filter(|s| s.is_adjacent(&schematic))
        .map(|s| s.number)
        .sum();

    println!(
        "unfiltered sum: {}",
        serials.iter().map(|s| s.number).sum::<usize>()
    );

    println!("part1 result: {total}");

    serials.iter().for_each(|s| s.annotate(&mut schematic));
    let ratios: usize = schematic
        .symbols
        .iter()
        .map(|r| r.iter())
        .flatten()
        .filter(|s| s.serials.len() == 2)
        .map(|s| s.serials.iter().map(|ser| ser.number).product::<usize>())
        .sum();

    println!("part2 result: {ratios}");
}

struct Schematic {
    symbols: Vec<Vec<Symbol>>,
}

struct Symbol {
    is_symbol: bool,
    serials: Vec<Serial>,
}

impl From<bool> for Symbol {
    fn from(value: bool) -> Self {
        Symbol {
            is_symbol: value,
            serials: vec![],
        }
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let symbols = value
            .lines()
            .map(str::chars)
            .map(|l| {
                l.map(|c| c != '.' && !c.is_alphanumeric())
                    .map(Symbol::from)
                    .collect()
            })
            .collect();

        Schematic { symbols }
    }
}

#[derive(Debug, Clone, Copy)]
struct Serial {
    number: usize,
    length: usize,
    position: (usize, usize),
}

impl Serial {
    fn is_adjacent(&self, schematic: &Schematic) -> bool {
        let (i, j) = self.position;

        if i > 0 {
            let line = schematic.symbols.get(i - 1).unwrap();

            if line
                .iter()
                .skip(if j > 0 { j - 1 } else { 0 })
                .take(self.length + 2)
                .any(|b| b.is_symbol)
            {
                return true;
            }
        }

        {
            let line = schematic.symbols.get(i).unwrap();
            if line
                .iter()
                .skip(if j > 0 { j - 1 } else { 0 })
                .take(self.length + 2)
                .any(|b| b.is_symbol)
            {
                return true;
            }
        }

        if let Some(line) = schematic.symbols.get(i + 1) {
            if line
                .iter()
                .skip(if j > 0 { j - 1 } else { 0 })
                .take(self.length + 2)
                .any(|b| b.is_symbol)
            {
                return true;
            }
        }

        false
    }

    fn str_to_serials(value: &str) -> Vec<Serial> {
        value
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let mut char_iter = l.char_indices();
                let mut serials = vec![];
                while let Some((j, c)) = char_iter.next() {
                    if !c.is_numeric() {
                        continue;
                    }
                    let mut num_acc = c.to_string();
                    while let Some((_, d)) = char_iter.next() {
                        if !d.is_numeric() {
                            break;
                        };
                        num_acc.push(d)
                    }
                    serials.push(Serial {
                        number: num_acc.parse().unwrap(),
                        length: num_acc.len(),
                        position: (i, j),
                    });
                }
                serials
            })
            .flatten()
            .collect()
    }

    fn annotate(&self, schematic: &mut Schematic) {
        let (i, j) = self.position;

        if i > 0 {
            let line = schematic.symbols.get_mut(i - 1).unwrap();

            for sym in line
                .iter_mut()
                .skip(if j > 0 { j - 1 } else { 0 })
                .take(self.length + 2)
                .filter(|b| b.is_symbol)
            {
                sym.serials.push(*self);
            }
        }

        {
            let line = schematic.symbols.get_mut(i).unwrap();
            for sym in line
                .iter_mut()
                .skip(if j > 0 { j - 1 } else { 0 })
                .take(self.length + 2)
                .filter(|b| b.is_symbol)
            {
                sym.serials.push(*self);
            }
        }

        if let Some(line) = schematic.symbols.get_mut(i + 1) {
            for sym in line
                .iter_mut()
                .skip(if j > 0 { j - 1 } else { 0 })
                .take(self.length + 2)
                .filter(|b| b.is_symbol)
            {
                sym.serials.push(*self);
            }
        }
    }
}
