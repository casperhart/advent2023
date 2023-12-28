use std::cell::Cell;
use std::{collections::BinaryHeap, fs::read_to_string};
#[derive(Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    d: Direction,
}

#[derive(Debug, Clone, Eq)]
struct Tile {
    coord: Coord,
    loss: usize,
    total_loss: Cell<usize>,
}

struct Step<'a> {
    tile: &'a Tile,
    loss: usize,
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_loss.cmp(&self.total_loss)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.total_loss.eq(&other.total_loss)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Grid {
    h_tiles: Vec<Tile>,
    v_tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut lines = value.lines().peekable();
        let width = lines.peek().unwrap().len();
        let height = (value.len() + 1) / (width + 1);

        let mut horizontal_tiles = Vec::with_capacity(value.len());
        let mut vertical_tiles = Vec::with_capacity(value.len());

        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                horizontal_tiles.push(Tile {
                    coord: Coord {
                        x,
                        y,
                        d: Direction::Horizontal,
                    },
                    loss: c.to_digit(10).unwrap() as usize,
                    total_loss: Cell::new(usize::MAX),
                });
                vertical_tiles.push(Tile {
                    coord: Coord {
                        x,
                        y,
                        d: Direction::Vertical,
                    },
                    loss: c.to_digit(10).unwrap() as usize,
                    total_loss: Cell::new(usize::MAX),
                });
            }
        }

        vertical_tiles[0].total_loss.set(0);
        horizontal_tiles[0].total_loss.set(0);
        Self {
            h_tiles: horizontal_tiles,
            v_tiles: vertical_tiles,
            width,
            height,
        }
    }
}

impl Grid {
    fn get_horizontal(&self, x: usize, y: usize, rng: &[i32]) -> Vec<Step> {
        let mut tiles = Vec::new();
        for sign in [-1, 1] {
            let mut loss = 0;

            for n in 1..=*rng.last().unwrap() {
                let x0 = x as i32 + n * sign;
                if x0 >= 0 && x0 < self.width as i32 {
                    let i = self.width as i32 * y as i32 + x0;
                    let tile = &self.h_tiles[i as usize];
                    loss += tile.loss;
                    if n >= *rng.first().unwrap() {
                        tiles.push(Step { tile, loss })
                    }
                }
            }
        }

        tiles
    }

    fn get_vertical(&self, x: usize, y: usize, rng: &[i32]) -> Vec<Step> {
        let mut tiles = Vec::new();
        for sign in [-1, 1] {
            let mut loss = 0;
            for n in 1..=*rng.last().unwrap() {
                let y0 = y as i32 + n * sign;
                if y0 >= 0 && y0 < self.height as i32 {
                    let i = self.width as i32 * y0 + x as i32;
                    let tile = &self.v_tiles[i as usize];
                    loss += tile.loss;
                    if n >= *rng.first().unwrap() {
                        tiles.push(Step { tile, loss })
                    }
                }
            }
        }

        tiles
    }

    fn update<'a>(&self, src: &'a Tile, next: Step<'a>, heap: &mut BinaryHeap<&'a Tile>) {
        let new_loss = src.total_loss.get() + next.loss;

        if new_loss < next.tile.total_loss.get() {
            next.tile.total_loss.set(new_loss);
            heap.push(next.tile);
        }
    }
}

fn solution(rng: &[i32]) {
    let input = read_to_string("data/d17.txt").unwrap();
    let grid = Grid::from(input.as_str());

    let mut heap: BinaryHeap<&Tile> = BinaryHeap::new();

    heap.push(&grid.h_tiles[0]);
    heap.push(&grid.v_tiles[0]);

    while let Some(t) = heap.pop() {
        let c = &t.coord;

        let next_tiles = match c.d {
            Direction::Horizontal => grid.get_vertical(c.x, c.y, rng),
            Direction::Vertical => grid.get_horizontal(c.x, c.y, rng),
        };

        for step in next_tiles {
            grid.update(t, step, &mut heap)
        }
    }

    let h = grid.h_tiles.last().unwrap().total_loss.get();
    let v = grid.v_tiles.last().unwrap().total_loss.get();
    println!("{:?}", std::cmp::min(h, v));
}

fn main() {
    let solution_1_range: Vec<_> = (1..=3).collect();
    solution(&solution_1_range);
    let solution_2_range: Vec<_> = (4..=10).collect();
    solution(&solution_2_range);
}
