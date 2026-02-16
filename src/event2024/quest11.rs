use ahash::{HashMap, HashMapExt};

pub fn part1(notes: &str) -> u64 {
    let generation_map = parse(notes);
    tng(&generation_map, "A", 4)
}

pub fn part2(notes: &str) -> u64 {
    let generation_map = parse(notes);
    tng(&generation_map, "Z", 10)
}

pub fn part3(notes: &str) -> u64 {
    let generation_map = parse(notes);
    let populations: Vec<_> = generation_map
        .keys()
        .map(|&initial| tng(&generation_map, initial, 20))
        .collect();
    populations.iter().max().unwrap() - populations.iter().min().unwrap()
}

fn parse(notes: &str) -> HashMap<&str, Vec<&str>> {
    notes
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let next_gen = right.split(",").collect();
            (left, next_gen)
        })
        .collect()
}

fn tng(generation_map: &HashMap<&str, Vec<&str>>, initial: &str, cycles: u32) -> u64 {
    let mut current_generation = HashMap::new();
    let mut next_generation = HashMap::new();

    current_generation.insert(initial, 1);

    for _ in 0..cycles {
        for (parent, population) in current_generation.drain() {
            for &child in &generation_map[parent] {
                *next_generation.entry(child).or_insert(0) += population;
            }
        }

        (current_generation, next_generation) = (next_generation, current_generation);
    }

    current_generation.values().sum()
}
