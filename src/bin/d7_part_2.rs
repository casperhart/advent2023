use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CardType {
    Joker = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<CardType>,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn get_hand_type(cards: &[CardType]) -> HandType {
        let mut card_counts = HashMap::new();

        cards
            .iter()
            .for_each(|c| *card_counts.entry(c).or_insert(0) += 1);

        // get highest counted card, as the jokers will imitate this card
        if card_counts.contains_key(&CardType::Joker) {
            let mut non_joker_counts: Vec<_> = card_counts
                .iter()
                .filter(|(&k, _)| k != &CardType::Joker)
                .collect();

            // hand may be JJJJJ
            if !non_joker_counts.is_empty() {
                non_joker_counts.sort_by_key(|(_, &v)| v);
                let max_count = non_joker_counts.iter().map(|(k, _)| k).last().unwrap();

                // increase count of most common card by count of jokers
                let jokers = card_counts.get(&CardType::Joker).unwrap().clone();
                card_counts.entry(max_count).and_modify(|e| *e += jokers);
                card_counts.remove(&CardType::Joker);
            }
        }

        let mut counts: Vec<_> = card_counts.values().into_iter().collect();
        counts.sort();
        counts.reverse();

        let hand_type = match counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match counts[1] {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match counts[1] {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            1 => HandType::HighCard,
            _ => unreachable!(),
        };

        return hand_type;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut ordering = self.hand_type.cmp(&other.hand_type);
        ordering = match ordering {
            Ordering::Equal => {
                let mut ord: Ordering = Ordering::Equal;
                for (c, o) in self.cards.iter().zip(other.cards.iter()) {
                    ord = c.cmp(&o);
                    if ord == Ordering::Equal {
                        continue;
                    } else {
                        break;
                    }
                }
                ord
            }
            ord => ord,
        };
        ordering
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .all(|(c, o)| c == o)
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let split: Vec<_> = value.split_whitespace().collect();
        let cards = split[0]
            .chars()
            .map(|c| match c {
                '2' => CardType::Two,
                '3' => CardType::Three,
                '4' => CardType::Four,
                '5' => CardType::Five,
                '6' => CardType::Six,
                '7' => CardType::Seven,
                '8' => CardType::Eight,
                '9' => CardType::Nine,
                'T' => CardType::Ten,
                'J' => CardType::Joker,
                'Q' => CardType::Queen,
                'K' => CardType::King,
                'A' => CardType::Ace,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let bid = split[1].parse().unwrap();
        let hand_type = Hand::get_hand_type(&cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn solution_2() {
    let input = read_to_string("data/d7.txt").unwrap();
    let input_lines = input.lines();

    let mut hands: Vec<_> = input_lines.map(|x| Hand::from(x)).collect();
    hands.sort();

    for hand in &hands {
        println!("{:?}", hand)
    }

    let total_winnings: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    println!("{}", total_winnings)
}

fn main() {
    solution_2();
}
