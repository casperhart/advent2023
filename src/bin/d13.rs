use std::fs::read_to_string;

struct Grid {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let rows: Vec<_> = value.lines().map(|s| s.to_string()).collect();
        let w = rows[0].len();
        let h = rows.len();
        let mut cols = vec![String::with_capacity(h); w];

        rows.iter()
            .for_each(|line| line.chars().enumerate().for_each(|(i, c)| cols[i].push(c)));
        Self { rows, cols }
    }
}

impl Grid {
    fn get_reflect_1(&self, vertical: bool) -> Option<usize> {
        let value = match vertical {
            true => &self.rows,
            false => &self.cols,
        };
        for i in 0..(value.len() - 1) {
            let t = &value[0..(i + 1)];
            let b = &value[(i + 1)..value.len()];

            if t.iter().rev().zip(b.iter()).all(|(t, b)| t == b) {
                return Some(i + 1);
            }
        }
        None
    }

    fn get_reflect_2(&self, vertical: bool) -> Option<usize> {
        let value = match vertical {
            true => &self.rows,
            false => &self.cols,
        };
        for i in 0..(value.len() - 1) {
            let t = &value[0..(i + 1)];
            let b = &value[(i + 1)..value.len()];

            let mut diff_num = 0;

            for (a, b) in t.iter().rev().zip(b.iter()) {
                for (c1, c2) in a.chars().zip(b.chars()) {
                    if c1 != c2 {
                        diff_num += 1
                    }
                }
                if diff_num > 1 {
                    break;
                }
            }
            if diff_num == 1 {
                return Some(i + 1);
            }
        }
        None
    }
}

fn solution_1() {
    let input = read_to_string("data/d13.txt").unwrap();
    let grids: Vec<_> = input.split("\n\n").map(Grid::from).collect();

    let h_reflect = grids
        .iter()
        .filter_map(|x| x.get_reflect_1(true))
        .collect::<Vec<_>>();

    let v_reflect = grids
        .iter()
        .filter_map(|x| x.get_reflect_1(false))
        .collect::<Vec<_>>();
    println!(
        "{:?}",
        h_reflect.iter().sum::<usize>() * 100 + v_reflect.iter().sum::<usize>()
    );
}

fn solution_2() {
    let input = read_to_string("data/d13.txt").unwrap();
    let grids: Vec<_> = input.split("\n\n").map(Grid::from).collect();

    let h_reflect = grids
        .iter()
        .filter_map(|x| x.get_reflect_2(true))
        .collect::<Vec<_>>();

    let v_reflect = grids
        .iter()
        .filter_map(|x| x.get_reflect_2(false))
        .collect::<Vec<_>>();

    println!(
        "{:?}",
        h_reflect.iter().sum::<usize>() * 100 + v_reflect.iter().sum::<usize>()
    );
}

fn main() {
    solution_1();
    solution_2();
}
