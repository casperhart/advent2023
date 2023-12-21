use std::fs::read_to_string;

fn hash(value: &[u8]) -> usize {
    let mut current_val: usize = 0;
    for &c in value {
        current_val = (current_val + c as usize) * 17 % 256;
    }
    current_val
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Clone, Debug)]
struct Boxx {
    lenses: Vec<Option<Lens>>,
}

impl Boxx {
    fn insert(&mut self, label: &str, focal_length: usize) {
        let mut lens_exists = false;
        for lens in self.lenses.iter_mut() {
            if let Some(l) = lens {
                if l.label == label {
                    *lens = Some(Lens {
                        label: label.to_string(),
                        focal_length,
                    });
                    lens_exists = true;
                }
            }
        }
        if !lens_exists {
            self.lenses.push(Some(Lens {
                label: label.to_string(),
                focal_length,
            }))
        }
    }

    fn remove(&mut self, label: &str) {
        for lens in self.lenses.iter_mut() {
            if let Some(l) = lens {
                if l.label == label {
                    *lens = None
                }
            }
        }
    }
}

fn solution_1() {
    let input = read_to_string("data/d15.txt").unwrap();
    let val = input.split(',').map(|x| hash(x.as_bytes())).sum::<usize>();
    println!("{:?}", val);
}

fn solution_2() {
    let input = read_to_string("data/d15.txt").unwrap();
    let operations: Vec<_> = input.split(',').collect();
    let mut boxes = vec![Boxx { lenses: Vec::new() }; 256];

    for operation in operations {
        let pos = operation.find('=').or(operation.find('-')).unwrap();
        let label = &operation[0..pos];
        let bx = hash(label.as_bytes());
        match operation.chars().nth(pos).unwrap() {
            '=' => boxes[bx].insert(label, operation[pos + 1..(pos + 2)].parse().unwrap()),
            '-' => boxes[bx].remove(label),
            _ => unreachable!(),
        }
    }

    let total = boxes
        .iter()
        .enumerate()
        .map(|(b_idx, b)| {
            let filtered: Vec<_> = b.lenses.iter().flatten().collect();
            filtered
                .iter()
                .enumerate()
                .map(|(slot_idx, x)| (b_idx + 1) * (slot_idx + 1) * x.focal_length)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", total);
}

fn main() {
    solution_1();
    solution_2();
}
