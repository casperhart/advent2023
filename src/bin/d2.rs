use std::cmp::max;
use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let game_str = value.split(": ").collect::<Vec<_>>();
        let id = game_str[0].split(" ").collect::<Vec<_>>()[1]
            .parse()
            .unwrap();

        let mut game = Game {
            id: id,
            red: 0,
            green: 0,
            blue: 0,
        };

        game_str[1]
            .split("; ")
            .collect::<Vec<_>>()
            .iter()
            .for_each(|x| {
                let vals = x.split(", ").collect::<Vec<_>>();
                for val in vals {
                    let pair = val.split(" ").collect::<Vec<_>>();
                    match pair[1] {
                        "red" => game.red = max(game.red, pair[0].parse().unwrap()),
                        "green" => game.green = max(game.green, pair[0].parse().unwrap()),
                        "blue" => game.blue = max(game.blue, pair[0].parse().unwrap()),
                        _ => unreachable!(),
                    }
                }
            });
        game
    }
}

fn solution_1() {
    let test_str = read_to_string("data/d2.txt").unwrap();
    let max_game = Game {
        id: 0,
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut total = 0;

    for line in test_str.lines() {
        let game = Game::from(line);
        if game.red <= max_game.red && game.green <= max_game.green && game.blue <= max_game.blue {
            total += game.id
        }
    }
    println!("Sum of possible game IDs: {}", total)
}

fn solution_2() {
    let test_str = read_to_string("data/d2.txt").unwrap();
    let mut total = 0;

    for line in test_str.lines() {
        let game = Game::from(line);
        total += game.power();
    }
    println!("Total power: {}", total)
}

fn main() {
    solution_1();
    solution_2();
}
