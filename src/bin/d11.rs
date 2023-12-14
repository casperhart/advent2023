use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

struct Grid {
    galaxies: Vec<Coord>,
    width: usize,
    height: usize,
}

impl Grid {
    fn expand(&mut self, expansion: usize) {
        let x_gaps: Vec<_> = (0..self.width)
            .filter(|x| {
                let xs: Vec<_> = self.galaxies.iter().map(|c| c.x).collect();
                !xs.contains(x)
            })
            .collect();
        let y_gaps: Vec<_> = (0..self.height)
            .filter(|y| {
                let ys: Vec<_> = self.galaxies.iter().map(|c| c.y).collect();
                !ys.contains(y)
            })
            .collect();

        self.galaxies.iter_mut().for_each(|g| {
            g.x += x_gaps.iter().filter(|&&gap| g.x > gap).count() * (expansion - 1);
            g.y += y_gaps.iter().filter(|&&gap| g.y > gap).count() * (expansion - 1);
        });

        self.width += x_gaps.len();
        self.height += y_gaps.len();
    }
}

impl From<String> for Grid {
    fn from(value: String) -> Self {
        let mut galaxies = Vec::new();
        let mut lines = value.lines().peekable();
        let width = lines.peek().unwrap().len();
        let mut height = 0;

        for (y, line) in lines.enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => {
                        galaxies.push(Coord { x, y });
                    }
                    '.' => (),
                    _ => unreachable!(),
                }
            }
            height += 1;
        }

        Grid {
            galaxies,
            width,
            height,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                if self.galaxies.contains(&Coord { x: i, y: j }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "Width: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        Ok(())
    }
}

fn solution_1() {
    let input = read_to_string("data/d11.txt").unwrap();
    let mut grid = Grid::from(input);
    grid.expand(2);
    let mut total_dist = 0;

    for i in 0..grid.galaxies.len() {
        for j in i..grid.galaxies.len() {
            let g0 = grid.galaxies[i];
            let g1 = grid.galaxies[j];
            total_dist += g0.x.abs_diff(g1.x) + g0.y.abs_diff(g1.y);
        }
    }

    println!("{}", total_dist);
}

fn solution_2() {
    let input = read_to_string("data/d11.txt").unwrap();
    let mut grid = Grid::from(input);
    grid.expand(1_000_000);
    let mut total_dist = 0;

    for i in 0..grid.galaxies.len() {
        for j in i..grid.galaxies.len() {
            let g0 = grid.galaxies[i];
            let g1 = grid.galaxies[j];
            total_dist += g0.x.abs_diff(g1.x) + g0.y.abs_diff(g1.y);
        }
    }

    println!("{}", total_dist);
}
fn main() {
    solution_1();
    solution_2();
}
