const INPUT: &str = include_str!("../input");

fn main() {
    let ans: usize = INPUT
        .lines()
        .map(Game::from)
        .filter(Game::is_possible)
        .map(|g| g.id)
        .sum();

    println!("answer: {ans}");
}

struct Game {
    id: usize,
    turns: Vec<Turn>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, turns) = value.split_once(':').unwrap();

        let id = game
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let turns = turns.split(';').map(Turn::from).collect();

        Game { id, turns }
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.turns.iter().all(Turn::is_possible)
    }
}

struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for ball in value.split(',').map(str::trim) {
            println!("{ball}");
            if let Some(n) = ball.strip_suffix(" red") {
                red = n.parse().unwrap();
            }
            if let Some(n) = ball.strip_suffix(" green") {
                green = n.parse().unwrap();
            }
            if let Some(n) = ball.strip_suffix(" blue") {
                blue = n.parse().unwrap();
            }
        }

        Turn { red, green, blue }
    }
}

impl Turn {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
