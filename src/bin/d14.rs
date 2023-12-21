use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    fs::read_to_string,
    hash::{Hash, Hasher},
};

struct Coord {
    x: usize,
    y: usize,
}
struct Grid {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let mut chars = Vec::with_capacity(value.len());
        for line in value.lines() {
            chars.append(&mut line.chars().collect::<Vec<_>>())
        }
        let height = chars.len() / width;
        Self {
            chars,
            width,
            height,
        }
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> char {
        self.chars[y * self.width + x]
    }

    fn swap(&mut self, a: Coord, b: Coord) {
        self.chars
            .swap(a.y * self.width + a.x, b.y * self.width + b.x)
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.chars.hash(&mut hasher);
        hasher.finish()
    }

    fn tilt_up(&mut self) {
        for x in 0..self.width {
            for y in (0..self.height).rev() {
                if self.get(x, y) == 'O' {
                    for j in (0..y).rev() {
                        match self.get(x, j) {
                            '.' => self.swap(Coord { x, y }, Coord { x, y: j }),
                            '#' => break,
                            'O' => continue,
                            _ => unreachable!(),
                        }
                    }
                };
            }
        }
    }
    fn tilt_down(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) == 'O' {
                    for j in y..self.height {
                        match self.get(x, j) {
                            '.' => self.swap(Coord { x, y }, Coord { x, y: j }),
                            '#' => break,
                            'O' => continue,
                            _ => unreachable!(),
                        }
                    }
                };
            }
        }
    }
    fn tilt_left(&mut self) {
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                if self.get(x, y) == 'O' {
                    for j in (0..x).rev() {
                        match self.get(j, y) {
                            '.' => self.swap(Coord { x, y }, Coord { x: j, y }),
                            '#' => break,
                            'O' => continue,
                            _ => unreachable!(),
                        }
                    }
                };
            }
        }
    }
    fn tilt_right(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == 'O' {
                    for j in x..self.width {
                        match self.get(j, y) {
                            '.' => self.swap(Coord { x, y }, Coord { x: j, y }),
                            '#' => break,
                            'O' => continue,
                            _ => unreachable!(),
                        }
                    }
                };
            }
        }
    }

    fn get_load(&self) -> usize {
        let mut total_load = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) == 'O' {
                    total_load += self.height - y;
                }
            }
        }

        total_load
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solution_1() {
    let input = read_to_string("data/d14.txt").unwrap();
    let mut grid = Grid::from(input.as_str());

    grid.tilt_up();

    println!("{:?}", grid.get_load());
}

fn solution_2() {
    let input = read_to_string("data/d14.txt").unwrap();
    let mut grid = Grid::from(input.as_str());

    let mut hashes = Vec::new();

    for i in 0..1000000 {
        grid.tilt_up();
        grid.tilt_left();
        grid.tilt_down();
        grid.tilt_right();

        let hash = grid.get_hash();
        if hashes.contains(&hash) {
            let first = hashes.iter().position(|&x| x == hash).unwrap();

            if (1000000000 - first - 1) % (i - first) == 0 {
                break;
            }
        }
        hashes.push(grid.get_hash());
    }
    println!("{:?}", grid.get_load());
}

fn main() {
    solution_1();
    solution_2();
}
