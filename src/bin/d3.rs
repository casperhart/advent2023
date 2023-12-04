use std::cmp::min;
use std::collections::HashSet;
use std::fs::read_to_string;

use std::fmt::Display;

struct Grid {
    values: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get_value(&self, x: usize, y: usize) -> char {
        if x >= self.width || y >= self.height {
            unreachable!()
        }
        self.values[y * self.width + x]
    }
    fn from_str(s: String) -> Self {
        let mut values: Vec<char> = Vec::with_capacity(s.len());
        let mut height = 0;
        for line in s.lines() {
            height += 1;
            line.chars().for_each(|c| values.push(c));
        }
        let width = values.len() / height;
        Self {
            values,
            width,
            height,
        }
    }

    fn get_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        let mut s = String::with_capacity(5);
        let mut start_coord = Coord { x: 0, y: 0 };

        let mut current_char: char;

        for y in 0..self.height {
            let mut has_number = false;
            for x in 0..self.width {
                current_char = self.get_value(x, y);

                // start of number
                if !has_number && current_char.is_ascii_digit() {
                    has_number = true;
                    start_coord = Coord { x, y };
                }

                // middle of number
                if has_number && current_char.is_ascii_digit() {
                    s.push(current_char)
                }

                // end of number
                if has_number && !current_char.is_ascii_digit() {
                    numbers.push(Number {
                        start: start_coord.clone(),
                        end: Coord { x: x - 1, y },
                        value: s.parse().unwrap(),
                    });
                    s.clear();
                    has_number = false
                }

                // end of number is at end of line
                if has_number && current_char.is_ascii_digit() && x == self.width - 1 {
                    numbers.push(Number {
                        start: start_coord.clone(),
                        end: Coord { x, y },
                        value: s.parse().unwrap(),
                    });
                    s.clear();
                    has_number = false;
                    continue;
                }
            }
        }
        numbers
    }

    fn get_surrounding_coords(&self, number: &Number) -> Vec<Coord> {
        let min_x = number.start.x.checked_sub(1).unwrap_or(0);
        let max_x = min(self.width - 1, number.end.x + 1);
        let min_y = number.start.y.checked_sub(1).unwrap_or(0);
        let max_y = min(number.start.y + 1, self.height - 1);

        let mut surrounding_coords = Vec::new();

        let mut top_coords: Vec<Coord> = (min_x..=max_x).map(|x| Coord { x, y: min_y }).collect();
        let mut bot_coords: Vec<Coord> = (min_x..=max_x).map(|x| Coord { x, y: max_y }).collect();
        surrounding_coords.append(&mut top_coords);
        surrounding_coords.append(&mut bot_coords);
        surrounding_coords.push(Coord {
            x: min_x,
            y: number.start.y,
        });
        surrounding_coords.push(Coord {
            x: max_x,
            y: number.start.y,
        });
        return surrounding_coords;
    }

    fn number_is_near_symbol(&self, number: Number) -> bool {
        let surrounding_coords = self.get_surrounding_coords(&number);

        surrounding_coords.iter().any(|coord| {
            let val = self.get_value(coord.x, coord.y);
            val.is_ascii_punctuation() && val != '.'
        })
    }

    fn get_possible_gears(&self, number: &Number) -> Vec<Coord> {
        let surrounding_coords = self.get_surrounding_coords(number);
        surrounding_coords
            .into_iter()
            .filter(|c| self.get_value(c.x, c.y) == '*')
            .collect()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get_value(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone, Copy)]
struct Number {
    start: Coord,
    end: Coord,
    value: usize,
}

fn solution1() {
    let input = read_to_string("data/d3.txt").unwrap();
    let grid = Grid::from_str(input);
    let all_numbers = grid.get_numbers();

    let part_numbers: Vec<_> = all_numbers
        .iter()
        .filter(|&&n| grid.number_is_near_symbol(n))
        .collect();

    let sum_of_parts: usize = part_numbers.iter().map(|n| n.value).sum();

    println!("{}", &grid);
    println!("Sum of part numbers (solution 1): {}", sum_of_parts);
}

fn solution_2() {
    let input = read_to_string("data/d3.txt").unwrap();
    let grid = Grid::from_str(input);
    let all_numbers = grid.get_numbers();

    let part_numbers: Vec<_> = all_numbers
        .iter()
        .filter(|&&n| grid.number_is_near_symbol(n))
        .collect();

    // vec of adjacent '*' for each part number in part_numbers
    let candidate_gears: Vec<_> = part_numbers
        .iter()
        .map(|n| grid.get_possible_gears(n))
        .collect();

    let mut gear_coords = Vec::with_capacity(part_numbers.len());
    let mut set = HashSet::new();

    // put any duplicate coordinates for '*' in to separate vec
    for v in &candidate_gears {
        for gear in v {
            if !set.insert(gear) {
                gear_coords.push(gear)
            }
        }
    }

    let mut total = 0;
    // iterate part numbers / adjacent '*' coordinates. If the '*' coordinate is in the gears vec, add total
    for coord in gear_coords {
        let adjacent_numbers: Vec<_> = candidate_gears
            .iter()
            .zip(part_numbers.iter())
            .filter(|(gears, _)| gears.contains(&coord))
            .collect();
        if adjacent_numbers.len() == 2 {
            total += adjacent_numbers[0].1.value * adjacent_numbers[1].1.value
        }
    }
    println!("Total (solution 2): {}", total)
}

fn main() {
    solution1();
    solution_2()
}
