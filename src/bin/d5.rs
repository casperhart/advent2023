use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct MapRange {
    dest_start: usize,
    source_start: usize,
    len: usize,
}

impl MapRange {
    fn get_mapping(&self, num: usize) -> Option<usize> {
        if num >= self.source_start && num < self.source_start + self.len {
            return Some(self.dest_start + num - self.source_start);
        }
        None
    }
}

impl From<Vec<&str>> for MapRange {
    fn from(value: Vec<&str>) -> Self {
        Self {
            dest_start: value[0].parse().unwrap(),
            source_start: value[1].parse().unwrap(),
            len: value[2].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: MapRange) {
        self.ranges.push(range)
    }

    fn get_mapping(&self, num: usize) -> usize {
        for range in &self.ranges {
            if let Some(dest) = range.get_mapping(num) {
                return dest;
            }
        }
        num
    }
}

fn solution_1() {
    let input = read_to_string("data/d5.txt").unwrap();
    let mut input_lines = input.lines();

    let seeds_line = input_lines.next().unwrap();
    let seeds_strs = seeds_line.split_whitespace().collect::<Vec<_>>();
    let seeds = seeds_strs[1..]
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut current_map_name = "";

    let mut maps: HashMap<&str, Map> = HashMap::new();

    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        let split_line: Vec<_> = line.split_whitespace().collect();

        if split_line.len() == 2 {
            current_map_name = split_line[0];
            maps.insert(current_map_name, Map::new());
        }

        if split_line.len() == 3 {
            maps.get_mut(current_map_name)
                .unwrap()
                .add_range(MapRange::from(split_line));
        }
    }

    let mut location = usize::MAX;
    let map_names = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut current_idx: usize;

    for seed in seeds {
        current_idx = seed;
        for name in &map_names {
            current_idx = maps.get(name).unwrap().get_mapping(current_idx);
        }
        if current_idx < location {
            location = current_idx
        }
    }
    println!("{}", location)
}

#[derive(Debug, Clone)]
struct Mapping {
    source_start: usize,
    dest_start: usize,
    len: usize,
}

impl Mapping {
    fn source_end(&self) -> usize {
        self.source_start + self.len - 1
    }

    fn contains_source(&self, num: usize) -> bool {
        num >= self.source_start && num < self.source_start + self.len
    }

    fn map_forward(&self, num: usize) -> usize {
        self.dest_start + num - self.source_start
    }

    fn fully_contains(&self, seed: &Seed) -> bool {
        seed.start >= self.source_start && seed.end() <= self.source_end()
    }

    fn overlaps(&self, seed: &Seed) -> bool {
        !(seed.start > self.source_end() || seed.end() < self.source_start)
    }
}

#[derive(Debug, Clone)]
struct Map2 {
    mappings: Vec<Mapping>,
}

impl Map2 {
    fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    fn apply_mapping(&self, num: usize) -> usize {
        let mut res = num;
        for m in &self.mappings {
            if m.contains_source(num) {
                res = m.map_forward(num);
                break;
            }
        }
        res
    }

    fn append_mappings(&mut self, value: Vec<&str>) {
        let dest = value[0].parse().unwrap();
        let src = value[1].parse().unwrap();
        let len: usize = value[2].parse().unwrap();

        self.mappings.push(Mapping {
            source_start: src,
            dest_start: dest,
            len,
        });
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Seed {
    start: usize,
    len: usize,
}

impl Seed {
    fn split(&self, m: &Mapping) -> Vec<Self> {
        let mut new_seeds = Vec::new();
        if m.fully_contains(self) || !m.overlaps(self) {
            new_seeds.push(self.clone())
        } else if self.start < m.source_start && self.end() > m.source_end() {
            new_seeds.push(Seed {
                start: self.start,
                len: m.source_start - self.start,
            });
            new_seeds.push(Seed {
                start: m.source_start,
                len: m.len,
            });
            new_seeds.push(Seed {
                start: m.source_end() + 1,
                len: self.end() - m.source_end(),
            })
        } else if self.start > m.source_start {
            new_seeds.push(Seed {
                start: self.start,
                len: m.source_end() - self.start + 1,
            });
            new_seeds.push(Seed {
                start: m.source_end() + 1,
                len: self.len + self.start - m.source_end() - 1,
            })
        } else if self.end() < m.source_end() {
            new_seeds.push(Seed {
                start: self.start,
                len: m.source_start - self.start,
            });
            new_seeds.push(Seed {
                start: m.source_start,
                len: self.len + self.start - m.source_start,
            })
        }
        new_seeds
    }

    fn end(&self) -> usize {
        self.start + self.len - 1
    }
}

fn solution_2() {
    let input = read_to_string("data/d5.txt").unwrap();
    let mut input_lines = input.lines();

    let seeds_line = input_lines.next().unwrap();
    let seeds_strs = seeds_line.split_whitespace().collect::<Vec<_>>();
    let seeds = seeds_strs[1..]
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut seeds: Vec<_> = seeds
        .chunks(2)
        .map(|chunk| Seed {
            start: chunk[0],
            len: chunk[1],
        })
        .collect();

    let mut maps: Vec<Map2> = Vec::new();

    for line in input_lines {
        if line.is_empty() {
            continue;
        }
        let split_line: Vec<_> = line.split_whitespace().collect();

        if split_line.len() == 2 {
            maps.push(Map2::new());
        }

        if split_line.len() == 3 {
            maps.last_mut().unwrap().append_mappings(split_line);
        }
    }

    for map in maps {
        for m in &map.mappings {
            let mut new_seeds = Vec::new();
            for seed in seeds {
                new_seeds.append(&mut seed.split(m));
            }
            seeds = new_seeds
        }
        seeds = seeds
            .iter()
            .map(|s| Seed {
                start: map.apply_mapping(s.start),
                len: s.len,
            })
            .collect();
    }

    println!("{:?}", seeds.iter().min_by_key(|s| s.start));
}

fn main() {
    solution_1();
    solution_2();
}

#[cfg(test)]
mod test {
    use crate::{Mapping, Seed};
    #[test]
    fn test_seed_split() {
        let seed = Seed { start: 10, len: 5 };
        let mapping = Mapping {
            dest_start: 30,
            source_start: 7,
            len: 6,
        };

        let split = seed.split(&mapping);
        assert!(split.contains(&Seed { start: 10, len: 3 }));
        assert!(split.contains(&Seed { start: 13, len: 2 }));

        let seed = Seed { start: 4, len: 5 };
        let mapping = Mapping {
            dest_start: 30,
            source_start: 7,
            len: 6,
        };

        let split = seed.split(&mapping);
        assert!(split.contains(&Seed { start: 4, len: 3 }));
        assert!(split.contains(&Seed { start: 7, len: 2 }));

        let seed = Seed { start: 4, len: 7 };
        let mapping = Mapping {
            dest_start: 30,
            source_start: 7,
            len: 2,
        };

        let split = seed.split(&mapping);
        assert!(split.contains(&Seed { start: 4, len: 3 }));
        assert!(split.contains(&Seed { start: 7, len: 2 }));
        assert!(split.contains(&Seed { start: 9, len: 2 }));
    }
}
