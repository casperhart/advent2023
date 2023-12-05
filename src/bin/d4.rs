use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    my_numbers: Vec<usize>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let split = value.split(": ").filter(|&s| s != "").collect::<Vec<_>>();
        let id_str = split[0].split_whitespace().collect::<Vec<_>>()[1];
        let num_split = split[1].split(" | ").collect::<Vec<_>>();
        let winning_numbers = num_split[0]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<HashSet<_>>();
        let my_numbers = num_split[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            id: id_str.parse().unwrap(),
            winning_numbers,
            my_numbers,
        }
    }
}

fn solution_1() {
    let input = read_to_string("data/d4.txt").unwrap();
    let cards = input.lines().map(|x| Card::from(x)).collect::<Vec<_>>();
    let mut total = 0;
    cards.iter().for_each(|card| {
        let winners: Vec<_> = card
            .my_numbers
            .iter()
            .filter(|c| card.winning_numbers.contains(c))
            .collect();
        if winners.len() > 0 {
            let base: usize = 2;
            total += base.pow((winners.len() - 1).try_into().unwrap());
        }
    });
    println!("Card value: {}", total);
}

fn solution_2() {
    let input = read_to_string("data/d4.txt").unwrap();
    let cards = input.lines().map(|x| Card::from(x)).collect::<Vec<_>>();
    let mut card_counts = vec![1; cards.len()];
    let mut winners: Vec<_>;

    for card in cards {
        winners = card
            .my_numbers
            .iter()
            .filter(|c| card.winning_numbers.contains(c))
            .collect();

        for i in 0..winners.len() {
            card_counts[card.id + i] += card_counts[card.id - 1];
        }
    }
    println!("Total cards: {}", card_counts.iter().sum::<usize>());
}
fn main() {
    solution_1();
    solution_2()
}
