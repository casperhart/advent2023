use std::collections::HashMap;
use std::fs::read_to_string;

use std::usize;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    is_start: bool,
    is_end: bool,
    l: usize,
    r: usize,
}

fn solution_1() {
    let input = read_to_string("data/d8.txt").unwrap();
    let mut input_lines = input.lines();

    let instructions: Vec<Direction> = input_lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();

    let raw_vals: Vec<_> = input_lines
        .skip(1)
        .map(|line| {
            let src = &line[0..3];
            let l = &line[7..10];
            let r = &line[12..15];
            (src, l, r)
        })
        .collect();

    let src: Vec<_> = raw_vals.iter().map(|(x, _, _)| x).collect();
    let src_map: HashMap<&str, usize> = src.iter().enumerate().map(|(i, &&x)| (x, i)).collect();

    let nodes: Vec<_> = raw_vals
        .iter()
        .map(|(s, l, r)| Node {
            is_start: s.contains("AAA"),
            is_end: s.contains("ZZZ"),
            l: *src_map.get(l).unwrap(),
            r: *src_map.get(r).unwrap(),
        })
        .collect();

    let mut current_index = *src_map.get("AAA").unwrap();
    let mut current_node = &nodes[current_index];
    let mut i = 0;

    for direction in instructions.iter().cycle() {
        i += 1;
        current_index = match direction {
            Direction::Left => current_node.l,
            Direction::Right => current_node.r,
        };
        current_node = &nodes[current_index];

        if current_node.is_end {
            break;
        }
    }
    println!("{}", i);
}

fn solution_2() {
    let input = read_to_string("data/d8.txt").unwrap();
    let mut input_lines = input.lines();

    let instructions: Vec<Direction> = input_lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();

    let raw_vals: Vec<_> = input_lines
        .skip(1)
        .map(|line| {
            let src = &line[0..3];
            let l = &line[7..10];
            let r = &line[12..15];
            (src, l, r)
        })
        .collect();

    let src: Vec<_> = raw_vals.iter().map(|(x, _, _)| x).collect();
    let src_map: HashMap<&str, usize> = src.iter().enumerate().map(|(i, &&x)| (x, i)).collect();

    let nodes: Vec<_> = raw_vals
        .iter()
        .map(|(s, l, r)| Node {
            is_start: s.ends_with('A'),
            is_end: s.ends_with('Z'),
            l: *src_map.get(l).unwrap(),
            r: *src_map.get(r).unwrap(),
        })
        .collect();

    let current_nodes: Vec<_> = nodes.iter().filter(|&x| x.is_start).collect();
    let mut num_cycles = Vec::new();

    current_nodes.iter().for_each(|&node| {
        let mut i = 0;
        let mut current_node = node;

        loop {
            for direction in &instructions {
                current_node = match direction {
                    Direction::Left => &nodes[current_node.l],
                    Direction::Right => &nodes[current_node.r],
                };
            }

            i += 1;

            if current_node.is_end {
                num_cycles.push(i);
                break;
            }
        }
    });

    println!(
        "{:?}",
        num_cycles.iter().product::<usize>() * instructions.len()
    );
}

fn main() {
    solution_1();
    solution_2();
}
