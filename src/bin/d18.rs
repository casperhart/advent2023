use std::{collections::HashSet, fs::read_to_string, str::Lines};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(i64, i64);

impl Coord {
    fn is_within(&self, grid: &Grid) -> bool {
        let is_edge = grid.perim_segs.iter().any(|p| {
            //horizontal
            let horizontal_match =
                (p.2 == p.3 && p.2 == self.1) && (self.0 >= p.0 && self.0 <= p.1);

            let vertical_match = (p.0 == p.1 && p.0 == self.0) && self.1 >= p.2 && self.1 <= p.3;
            vertical_match || horizontal_match
        });

        let intersects = grid
            .perim_segs
            .iter()
            .filter(|&s| s.1 < self.0 && s.2 <= self.1 && s.3 > self.1)
            .count();

        is_edge || (intersects > 0 && intersects % 2 == 1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Seg(i64, i64, i64, i64);

impl Seg {
    fn new(x0: i64, x1: i64, y0: i64, y1: i64) -> Self {
        let min_x = std::cmp::min(x0, x1);
        let max_x = std::cmp::max(x0, x1);
        let min_y = std::cmp::min(y0, y1);
        let max_y = std::cmp::max(y0, y1);

        Seg(min_x, max_x, min_y, max_y)
    }

    fn from_coords(c0: Coord, c1: Coord) -> Self {
        Self::new(c0.0, c1.0, c0.1, c1.1)
    }
}

impl Seg {
    fn len(&self) -> i64 {
        if self.0 == self.1 {
            (self.3 - self.2).abs() + 1
        } else {
            (self.1 - self.0).abs() + 1
        }
    }

    fn is_within(&self, grid: &Grid) -> bool {
        let centre_x = (self.0 - self.1) as f32 / 2. + self.1 as f32;
        let centre_y = (self.2 - self.3) as f32 / 2. + self.3 as f32;

        let is_edge = grid.perim_segs.iter().any(|p| {
            //horizontal
            let horizontal_match =
                (p.2 == p.3 && p.2 == self.2 && p.2 == self.3) && (self.0 >= p.0 && self.1 <= p.1);

            let vertical_match =
                (p.0 == p.1 && p.0 == self.0 && p.1 == self.1) && self.2 >= p.2 && self.3 <= p.3;
            vertical_match || horizontal_match
        });

        let intersects = grid
            .perim_segs
            .iter()
            .filter(|&s| {
                (s.1 as f32) < centre_x && (s.2 as f32) <= centre_y && (s.3 as f32) > centre_y
            })
            .count();

        is_edge || (intersects > 0 && intersects % 2 == 1)
    }
}

//x0, x1, y0, y1
#[derive(Debug)]
struct Rect(i64, i64, i64, i64);

impl Rect {
    fn is_within(&self, grid: &Grid) -> bool {
        let centre_x = (self.0 - self.1) as f32 / 2. + self.1 as f32;
        let centre_y = (self.2 - self.3) as f32 / 2. + self.3 as f32;

        if self.0 == self.1 || self.2 == self.3 {
            print!("")
        }

        let intersects = grid
            .perim_segs
            .iter()
            .filter(|&s| {
                (s.0 as f32) <= centre_x
                    && s.0 == s.1
                    && ((s.2 as f32) < centre_y && s.3 as f32 >= centre_y)
            })
            .count();

        intersects > 0 && intersects % 2 == 1
    }
}

#[derive(Debug)]
struct Grid {
    bounds: Rect,
    perim_segs: Vec<Seg>,
    vertices: Vec<Coord>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (self.bounds.2..=self.bounds.3).rev() {
            for x in self.bounds.0..=self.bounds.1 {
                if self.vertices.contains(&Coord(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn get_vertices_1(lines: Lines) -> Vec<Coord> {
        let mut vertices = Vec::new();
        let mut current_coord = Coord(0, 0);

        for line in lines {
            let s: Vec<_> = line.split_whitespace().collect();
            let val = s[1].parse::<i64>().unwrap();
            current_coord = match s[0] {
                "R" => Coord(current_coord.0 + val, current_coord.1),
                "L" => Coord(current_coord.0 - val, current_coord.1),
                "D" => Coord(current_coord.0, current_coord.1 - val),
                "U" => Coord(current_coord.0, current_coord.1 + val),
                _ => unreachable!(),
            };
            vertices.push(current_coord)
        }
        vertices
    }
    fn get_vertices_2(lines: Lines) -> Vec<Coord> {
        let mut vertices = Vec::new();
        let mut current_coord = Coord(0, 0);

        for line in lines {
            let s = line.split_whitespace().collect::<Vec<_>>()[2];
            let val = i64::from_str_radix(&s[2..7], 16).unwrap();
            current_coord = match &s[7..8] {
                "0" => Coord(current_coord.0 + val, current_coord.1),
                "1" => Coord(current_coord.0, current_coord.1 - val),
                "2" => Coord(current_coord.0 - val, current_coord.1),
                "3" => Coord(current_coord.0, current_coord.1 + val),
                _ => unreachable!(),
            };
            vertices.push(current_coord)
        }
        vertices
    }

    fn from_vertices(vertices: Vec<Coord>) -> Self {
        // get vertical line segments for determining if a rect / seg is within the shape
        let n = vertices.len();
        let mut perim_segs = Vec::with_capacity(n - 1);
        for i in 0..(n - 1) {
            perim_segs.push(Seg::from_coords(vertices[i], vertices[i + 1]));
        }
        perim_segs.push(Seg::from_coords(vertices[0], *vertices.last().unwrap()));
        perim_segs = perim_segs
            .into_iter()
            //.filter(|s| s.0 == s.1)
            .collect::<Vec<_>>();

        let min_x = vertices.iter().map(|c| c.0).min().unwrap();
        let min_y = vertices.iter().map(|c| c.1).min().unwrap();
        let max_x = vertices.iter().map(|c| c.0).max().unwrap();
        let max_y = vertices.iter().map(|c| c.1).max().unwrap();

        Self {
            bounds: Rect(min_x, max_x, min_y, max_y),
            perim_segs,
            vertices,
        }
    }
}

impl Grid {}

fn solution(grid: Grid) -> i64 {
    // get x and y values for all vertices
    let mut xs = grid.vertices.iter().map(|c| c.0).collect::<Vec<_>>();
    xs.sort();
    xs.dedup();

    let mut ys = grid.vertices.iter().map(|c| c.1).collect::<Vec<_>>();
    ys.sort();
    ys.dedup();

    // divide grid in to squares, by drawing a vertical and horizontal line through each vertex
    // to get the internal volume of the rectangles
    let mut rects = Vec::with_capacity((xs.len() - 1) * (ys.len() - 1));
    for x in xs.windows(2) {
        for y in ys.windows(2) {
            rects.push(Rect(x[0], x[1], y[0], y[1]));
        }
    }
    rects.retain(|r| r.is_within(&grid));
    let rect_volume: i64 = rects
        .iter()
        .map(|r| ((r.0 - r.1).abs() - 1) * ((r.2 - r.3).abs() - 1))
        .sum();

    // get all the segments within the grid (including edges) and sum up the volume
    let mut segs = HashSet::with_capacity(xs.len() * ys.len());
    for x in xs.windows(2) {
        for y in ys.windows(2) {
            segs.insert(Seg::new(x[0], x[1], y[0], y[0]));
            segs.insert(Seg::new(x[0], x[1], y[1], y[1]));
            segs.insert(Seg::new(x[0], x[0], y[0], y[1]));
            segs.insert(Seg::new(x[1], x[1], y[0], y[1]));
        }
    }

    segs.retain(|s| s.is_within(&grid));
    let seg_volume = segs.iter().map(|x| x.len() - 2).sum::<i64>();

    // get all the vertices within the grid
    let mut points = Vec::with_capacity(xs.len() * ys.len());
    for x in &xs {
        for y in &ys {
            points.push(Coord(*x, *y));
        }
    }
    let point_volume = points.iter().filter(|p| p.is_within(&grid)).count();

    rect_volume + seg_volume + point_volume as i64
}

fn solution_1() {
    let input = read_to_string("data/d18.txt").unwrap();
    let lines = input.lines();
    let vertices = Grid::get_vertices_1(lines);
    let grid = Grid::from_vertices(vertices);

    let res = solution(grid);
    println!("{}", res)
}

fn solution_2() {
    let input = read_to_string("data/d18.txt").unwrap();
    let lines = input.lines();
    let vertices = Grid::get_vertices_2(lines);
    let grid = Grid::from_vertices(vertices);

    let res = solution(grid);
    println!("{}", res)
}

fn main() {
    solution_1();
    solution_2();
}
