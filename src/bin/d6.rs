use std::fs::read_to_string;

fn get_winning_times(t: usize, d: usize) -> usize {
    let t = t as f64;
    let d = d as f64;
    let min_button_time = (-(t.powi(2) / 4. - d).sqrt() + t / 2. + 1.).floor();
    let max_button_time = ((t.powi(2) / 4. - d).sqrt() + t / 2. - 1.).ceil();

    (max_button_time - min_button_time + 1.) as usize
}

fn solution_1() {
    let input = read_to_string("data/d6.txt").unwrap();
    let mut input_lines = input.lines();

    let times: Vec<_> = input_lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<_> = input_lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let button_times: usize = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| get_winning_times(*t, *d))
        .product();

    println!("{:?}", button_times)
}

fn solution_2() {
    let input = read_to_string("data/d6.txt").unwrap();
    let mut input_lines = input.lines();
    let t: usize = input_lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(&mut "".to_owned(), |a, b| {
            a.push_str(b);
            a
        })
        .parse()
        .unwrap();
    let d: usize = input_lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(&mut "".to_owned(), |a, b| {
            a.push_str(b);
            a
        })
        .parse()
        .unwrap();

    let w = get_winning_times(t, d);

    println!("{:?}", w);
}
fn main() {
    solution_1();
    solution_2();
}
