fn main() {
    let input = include_str!("../input");

    let sum: usize = input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_numeric()))
        .map(|s| {
            let numstr: String = s.collect();
            let first = numstr.chars().next().unwrap();
            let last = numstr.chars().last().unwrap();
            let num: usize = [first, last].iter().collect::<String>().parse().unwrap();
            num
        })
        .sum();
    println!("sum: {sum}");
}
