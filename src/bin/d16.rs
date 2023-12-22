use std::{fmt::Display, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum TileType {
    Pipe,
    Dash,
    Slash,
    BackSlash,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    lit: bool,
    lit_from: Vec<Direction>,
}

impl Tile {
    fn is_illuminated_from(&self, d: &Direction) -> bool {
        self.lit && self.lit_from.contains(d)
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut lines = value.lines().peekable();
        let width = lines.peek().unwrap().len();
        let mut tiles = Vec::with_capacity(value.len());

        for line in lines {
            for c in line.chars() {
                let tile_type = match c {
                    '.' => TileType::Empty,
                    '|' => TileType::Pipe,
                    '-' => TileType::Dash,
                    '/' => TileType::Slash,
                    '\\' => TileType::BackSlash,
                    _ => unreachable!(),
                };
                tiles.push(Tile {
                    tile_type,
                    lit: false,
                    lit_from: Vec::new(),
                })
            }
        }

        let height = tiles.len() / width;
        Self {
            tiles,
            width,
            height,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y * self.height + x];
                match tile.lit {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn get(&mut self, c: Coord) -> &Tile {
        let x = c.x;
        let y = c.y;

        &self.tiles[y * self.width + x]
    }

    fn illuminate_tile(&mut self, c: &Coord, direction: &Direction) {
        let tile = &mut self.tiles[c.y * self.width + c.x];
        tile.lit = true;
        tile.lit_from.push(direction.clone());
    }

    fn num_energised(&self) -> usize {
        self.tiles.iter().filter(|x| x.lit).count()
    }

    fn deilluminate(&mut self) {
        for tile in &mut self.tiles {
            tile.lit = false;
            tile.lit_from.clear();
        }
    }

    fn illuminate(&mut self, start: Coord, d: Direction) {
        let mut queue = Vec::new();

        queue.push((start, d));

        while let Some((c, d)) = queue.pop() {
            let t = self.get(c);

            if !t.is_illuminated_from(&d) {
                match (t.tile_type, &d) {
                    (TileType::Empty | TileType::Dash, Direction::Left)
                    | (TileType::Slash, Direction::Bottom)
                    | (TileType::BackSlash, Direction::Top) => {
                        self.illuminate_tile(&c, &d);
                        if c.x < self.width - 1 {
                            queue.push((Coord { x: c.x + 1, y: c.y }, Direction::Left))
                        };
                    }

                    (TileType::Empty | TileType::Dash, Direction::Right)
                    | (TileType::Slash, Direction::Top)
                    | (TileType::BackSlash, Direction::Bottom) => {
                        self.illuminate_tile(&c, &d);
                        if c.x > 0 {
                            queue.push((Coord { x: c.x - 1, y: c.y }, Direction::Right))
                        };
                    }

                    (TileType::Empty | TileType::Pipe, Direction::Top)
                    | (TileType::Slash, Direction::Right)
                    | (TileType::BackSlash, Direction::Left) => {
                        self.illuminate_tile(&c, &d);
                        if c.y < self.width - 1 {
                            queue.push((Coord { x: c.x, y: c.y + 1 }, Direction::Top))
                        };
                    }
                    (TileType::Empty | TileType::Pipe, Direction::Bottom)
                    | (TileType::Slash, Direction::Left)
                    | (TileType::BackSlash, Direction::Right) => {
                        self.illuminate_tile(&c, &d);
                        if c.y > 0 {
                            queue.push((Coord { x: c.x, y: c.y - 1 }, Direction::Bottom))
                        };
                    }

                    (TileType::Pipe, Direction::Left | Direction::Right) => {
                        self.illuminate_tile(&c, &d);
                        if c.y > 0 {
                            queue.push((Coord { x: c.x, y: c.y - 1 }, Direction::Bottom));
                        }
                        if c.y < self.width - 1 {
                            queue.push((Coord { x: c.x, y: c.y + 1 }, Direction::Top));
                        }
                    }

                    (TileType::Dash, Direction::Top | Direction::Bottom) => {
                        self.illuminate_tile(&c, &d);
                        if c.x < self.width - 1 {
                            queue.push((Coord { x: c.x + 1, y: c.y }, Direction::Left));
                        }
                        if c.x > 0 {
                            queue.push((Coord { x: c.x - 1, y: c.y }, Direction::Right))
                        };
                    }
                }
            }
        }
    }
}

fn solution_1() {
    let input = read_to_string("data/d16.txt").unwrap();
    let mut grid = Grid::from(input.as_str());

    grid.illuminate(Coord { x: 0, y: 0 }, Direction::Left);

    println!("{}", grid.num_energised());
    //println!("{}", grid);
}

fn solution_2() {
    let input = read_to_string("data/d16.txt").unwrap();
    let mut grid = Grid::from(input.as_str());

    let start_coords: Vec<_> = (0..grid.width)
        .flat_map(|x| {
            vec![
                (Coord { x, y: 0 }, Direction::Top),
                (
                    Coord {
                        x,
                        y: grid.height - 1,
                    },
                    Direction::Bottom,
                ),
            ]
        })
        .chain((0..grid.height).flat_map(|y| {
            vec![
                ((Coord { x: 0, y }), Direction::Left),
                (
                    (Coord {
                        x: grid.width - 1,
                        y,
                    }),
                    Direction::Right,
                ),
            ]
        }))
        .collect();

    let mut max_count = 0;

    for (c, d) in start_coords {
        grid.illuminate(c, d);
        let num_lit = grid.num_energised();
        max_count = std::cmp::max(num_lit, max_count);
        grid.deilluminate();
    }

    println!("{}", max_count);
}

fn main() {
    solution_1();
    solution_2();
}
