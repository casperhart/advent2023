use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use std::{fmt::Debug, fs::read_to_string};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Empty,
    Start,
}

struct Grid {
    coords: Vec<Coord>,
    width: usize,
}

impl Grid {
    fn height(&self) -> usize {
        self.coords.len() / self.width
    }

    fn get(&self, x: usize, y: usize) -> &Coord {
        &self.coords[y * self.width + x]
    }

    fn neighbours(&self, coord: &Coord) -> Vec<&Coord> {
        let x = coord.x;
        let y = coord.y;

        let mut surrounding_values = Vec::new();

        if x > 0 {
            let left = self.get(x - 1, y);
            match (coord.tile, left.tile) {
                (
                    Tile::Start | Tile::Horizontal | Tile::NorthWest | Tile::SouthWest,
                    Tile::Horizontal | Tile::NorthEast | Tile::SouthEast,
                ) => surrounding_values.push(left),
                _ => (),
            }
        }

        if x < self.width - 1 {
            let right = self.get(x + 1, y);
            match (coord.tile, right.tile) {
                (
                    Tile::Start | Tile::Horizontal | Tile::NorthEast | Tile::SouthEast,
                    Tile::Horizontal | Tile::NorthWest | Tile::SouthWest,
                ) => surrounding_values.push(right),
                _ => (),
            }
        }

        if y > 0 {
            let top = self.get(x, y - 1);
            match (coord.tile, top.tile) {
                (
                    Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest,
                    Tile::Vertical | Tile::SouthEast | Tile::SouthWest,
                ) => surrounding_values.push(top),
                _ => (),
            }
        }

        if y < self.height() - 1 {
            let bottom: &Coord = self.get(x, y + 1);
            match (coord.tile, bottom.tile) {
                (
                    Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest,
                    Tile::Vertical | Tile::NorthEast | Tile::NorthWest,
                ) => surrounding_values.push(bottom),
                _ => (),
            }
        }

        surrounding_values
    }

    fn start_pos(&self) -> &Coord {
        let idx = self
            .coords
            .iter()
            .position(|x| x.tile == Tile::Start)
            .unwrap();

        &self.coords[idx]
    }

    fn populate_distances(&self) {
        let mut queue = BinaryHeap::new();

        let start = self.start_pos();
        start.dist.set(Some(0));

        queue.push(start);

        while let Some(coord) = queue.pop() {
            let mut neighbours = self.neighbours(coord);
            for c in neighbours.iter_mut() {
                match c.dist.get() {
                    Some(val) => {
                        if val > coord.dist.get().unwrap() {
                            c.dist.set(Some(coord.dist.get().unwrap() + 1));
                            queue.push(c);
                        }
                    }
                    None => {
                        c.dist.set(Some(coord.dist.get().unwrap() + 1));
                        queue.push(c);
                    }
                }
            }
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width {
                let char = match self.get(col, row).tile {
                    Tile::Empty => '.',
                    Tile::Horizontal => '-',
                    Tile::Vertical => '|',
                    Tile::NorthEast => 'L',
                    Tile::SouthEast => 'F',
                    Tile::SouthWest => '7',
                    Tile::NorthWest => 'J',
                    Tile::Start => 'S',
                };
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<String> for Grid {
    fn from(value: String) -> Self {
        let value = value.lines();
        let mut lines = value.peekable();
        let first_line = lines.peek().unwrap();

        let mut grid = Grid {
            coords: Vec::new(),
            width: first_line.len(),
        };

        lines.enumerate().for_each(|(y, line)| {
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Empty,
                    '|' => Tile::Vertical,
                    '-' => Tile::Horizontal,
                    'F' => Tile::SouthEast,
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWest,
                    '7' => Tile::SouthWest,
                    'S' => Tile::Start,
                    _ => unreachable!(),
                };
                grid.coords.push(Coord {
                    x,
                    y,
                    tile,
                    dist: Cell::new(None),
                })
            }
        });
        grid
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord {
    x: usize,
    y: usize,
    tile: Tile,
    dist: Cell<Option<usize>>,
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.dist.get(), other.dist.get()) {
            (Some(d1), Some(d2)) => d1.cmp(&d2),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn solution_1() {
    let input = read_to_string("data/d10.txt").unwrap();
    let grid = Grid::from(input);

    grid.populate_distances();
    println!(
        "Farthest distance in loop: {:?}",
        grid.coords
            .iter()
            .max_by_key(|c| c.dist.get())
            .unwrap()
            .dist
            .get()
            .unwrap()
    );
}

fn solution_2() {
    let input = read_to_string("data/d10.txt").unwrap();
    let grid = Grid::from(input);
    grid.populate_distances();

    let mut num_enclosed = 0;

    for row_idx in 0..grid.height() {
        let mut row = grid.coords[(row_idx * grid.width)..((row_idx + 1) * grid.width)].iter();
        let mut in_loop = false;

        while let Some(coord) = row.next() {
            // count tile enclosed by loop
            if coord.dist.get().is_none() && in_loop {
                num_enclosed += 1;
            }

            // tile is outside the loop
            if coord.dist.get().is_none() {
                continue;
            }

            // Transitions from inside to outside are sequences
            // |, F--J, L--7
            // i.e. crossing the pipe
            match coord.tile {
                // check for |
                Tile::Vertical => in_loop = !in_loop,
                // check for L--7
                Tile::NorthEast => {
                    while let Some(coord) = row.next() {
                        match coord.tile {
                            Tile::Horizontal | Tile::Start => continue,
                            Tile::SouthWest => {
                                in_loop = !in_loop;
                                break;
                            }
                            _ => break,
                        }
                    }
                }
                // check for F--J
                Tile::SouthEast => {
                    while let Some(coord) = row.next() {
                        match coord.tile {
                            Tile::Horizontal | Tile::Start => continue,
                            Tile::NorthWest => {
                                in_loop = !in_loop;
                                break;
                            }
                            _ => break,
                        }
                    }
                }
                _ => (),
            }
        }
    }
    println!("Tiles enclosed by loop: {:?}", num_enclosed);
}

fn main() {
    solution_1();
    solution_2();
}
