const INPUT: &str = include_str!("../input");

fn main() {
    let games: Vec<Game> = INPUT.lines().map(Game::from).collect();

    let ans1: usize = games.iter().filter(|g| g.is_possible()).map(|g| g.id).sum();

    println!("answer: {ans1}");

    let ans2: usize = games
        .iter()
        .map(Game::minimum_set)
        .map(BallSet::power)
        .sum();
    println!("answer: {ans2}");
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

    fn minimum_set(&self) -> BallSet {
        self.turns.iter().fold(BallSet::default(), BallSet::include)
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

struct BallSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl BallSet {
    fn power(self) -> usize {
        self.red * self.green * self.blue
    }

    fn include(mut self, turn: &Turn) -> Self {
        self.red = self.red.max(turn.red);
        self.green = self.green.max(turn.green);
        self.blue = self.blue.max(turn.blue);
        self
    }
}

impl Default for BallSet {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}
