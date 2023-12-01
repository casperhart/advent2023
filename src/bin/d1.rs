use std::fs::read_to_string;
fn solution_1() {
    let test_str = read_to_string("data/d1_1.txt").unwrap();
    let mut result: usize = 0;
    for line in test_str.lines() {
        let first_digit = line.chars().find(|x| char::is_ascii_digit(x)).unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(|x| char::is_ascii_digit(x))
            .unwrap();
        result += format!("{}{}", first_digit, last_digit)
            .parse::<usize>()
            .unwrap();
    }
    println!("{}", result)
}

fn solution_2() {
    let test_str = read_to_string("data/d1_2.txt").unwrap();
    let valid_numbers = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut total = 0;

    for line in test_str.lines() {
        let l_indices: Vec<_> = valid_numbers
            .iter()
            .filter_map(|(k, v)| match line.find(k) {
                Some(i) => Some((i, v)),
                None => None,
            })
            .collect();
        let r_indices: Vec<_> = valid_numbers
            .iter()
            .filter_map(|(k, v)| match line.rfind(k) {
                Some(i) => Some((i, v)),
                None => None,
            })
            .collect();
        let first_val = l_indices.iter().min_by_key(|(i, _)| i).unwrap();
        let last_val = r_indices.iter().max_by_key(|(i, _)| i).unwrap();
        total += first_val.1 * 10;
        total += last_val.1;
    }
    println!("{:?}", total);
}
fn main() {
    solution_1();
    solution_2();
}
