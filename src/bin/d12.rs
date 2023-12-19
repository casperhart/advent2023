use std::{collections::HashMap, fs::read_to_string, hash::Hash};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct State {
    char_ind: usize,
    char_count: usize,
    in_broken_seq: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum Spring {
    Unknown,
    Broken,
    Operational,
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    b_counts: Vec<usize>,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let split: Vec<_> = value.split_whitespace().collect();

        let springs = split[0]
            .chars()
            .map(|c| match c {
                '?' => Spring::Unknown,
                '#' => Spring::Broken,
                '.' => Spring::Operational,
                _ => unreachable!(),
            })
            .collect();

        let b_inds: Vec<_> = split[1].split(',').map(|x| x.parse().unwrap()).collect();

        Self {
            springs,
            b_counts: b_inds,
        }
    }
}

fn solution(n: usize) {
    let input = read_to_string("data/d12.txt").unwrap();
    let rows: Vec<_> = input
        .lines()
        .map(|x| {
            let s: Vec<_> = x.split_whitespace().collect();
            let left = vec![s[0]; n].join("?");
            let right = vec![s[1]; n].join(",");

            let s = [left, right].join(" ");

            Row::from(s.as_str())
        })
        .collect();

    let cnts: Vec<_> = rows
        .iter()
        .map(|row| {
            let mut to_states = HashMap::<(usize, usize, usize), usize>::new();
            let mut from_states = HashMap::<(usize, usize, usize), usize>::new();
            from_states.insert((0, 0, 0), 1);

            for s in &row.springs {
                for ((mut cnt_idx, mut crnt_cnt, mut expct_op), num) in &from_states {
                    match s {
                        Spring::Broken | Spring::Unknown
                            if cnt_idx < row.b_counts.len() && expct_op == 0 =>
                        {
                            if s == &Spring::Unknown && crnt_cnt == 0 {
                                to_states
                                    .entry((cnt_idx, crnt_cnt, expct_op))
                                    .and_modify(|x| *x += num)
                                    .or_insert(*num);
                            }
                            crnt_cnt += 1;
                            if crnt_cnt == row.b_counts[cnt_idx] {
                                (cnt_idx, crnt_cnt, expct_op) = (cnt_idx + 1, 0, 1)
                            }
                            to_states
                                .entry((cnt_idx, crnt_cnt, expct_op))
                                .and_modify(|x| *x += num)
                                .or_insert(*num);
                        }
                        Spring::Operational | Spring::Unknown if crnt_cnt == 0 => {
                            expct_op = 0;
                            to_states
                                .entry((cnt_idx, crnt_cnt, expct_op))
                                .and_modify(|x| *x += num)
                                .or_insert(*num);
                        }
                        _ => (),
                    }
                }
                (from_states, to_states) = (to_states, from_states);
                to_states.clear()
            }

            from_states
                .iter()
                .map(|((i, _, _), &n)| if i == &row.b_counts.len() { n } else { 0 })
                .sum::<usize>()
        })
        .collect();

    println!("{:?}", cnts.iter().sum::<usize>());
}

fn main() {
    solution(1);
    solution(5);
}
