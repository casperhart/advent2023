use std::fs::read_to_string;

fn solution(rev: bool) {
    let input = read_to_string("data/d9.txt").unwrap();
    let input_lines = input.lines();
    let mut next_vals = Vec::new();

    for line in input_lines {
        let mut seq: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if rev {
            seq.reverse()
        }

        let mut seqs = Vec::new();
        seqs.push(seq);

        loop {
            let next_seq: Vec<_> = seqs
                .last()
                .unwrap()
                .windows(2)
                .map(|chunk| chunk[1] - chunk[0])
                .collect();
            if next_seq.iter().all(|&x| x == next_seq[0]) {
                let mut next_val = next_seq[0];
                for seq in seqs.iter().rev() {
                    next_val += seq.last().unwrap()
                }
                next_vals.push(next_val);
                break;
            }
            seqs.push(next_seq);
        }
    }
    println!("{:?}", next_vals.iter().sum::<isize>());
}

fn solution_1() {
    solution(false)
}
fn solution_2() {
    solution(true)
}

fn main() {
    solution_1();
    solution_2();
}
