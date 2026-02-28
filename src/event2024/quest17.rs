use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::HashSet;

use crate::util::math::taxicab_distance;

pub fn part1(notes: &str) -> u64 {
    let stars = parse(notes);

    constelation_size(stars)
}

pub fn part2(notes: &str) -> u64 {
    let stars = parse(notes);

    constelation_size(stars)
}

pub fn part3(notes: &str) -> u64 {
    let stars = parse(notes);
    let mut constelations: Vec<Vec<(usize, usize)>> = Vec::new();

    for star in stars {
        let (near, far): (Vec<_>, Vec<_>) = constelations
            .into_iter()
            .partition(|constelation| constelation.iter().any(|&s| taxicab_distance(s, star) < 6));

        let mut constelation = Vec::from([star]);
        constelation.extend(near.iter().flatten());
        constelations = far;
        constelations.push(constelation);
    }

    let mut constelation_sizes = constelations
        .into_iter()
        .map(constelation_size)
        .collect::<Vec<u64>>();
    constelation_sizes.sort_unstable();
    constelation_sizes.iter().rev().take(3).product()
}

fn parse(notes: &str) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for (y, line) in notes.lines().enumerate() {
        line.bytes()
            .enumerate()
            .filter(|(_, val)| *val == b'*')
            .for_each(|(x, _)| points.push((x, y)))
    }

    points
}

fn constelation_size(stars: Vec<(usize, usize)>) -> u64 {
    let mut min_heap = BinaryHeap::from([Reverse((0, stars[0]))]);
    let mut stars: HashSet<_> = stars.iter().collect();
    let mut total = 0;

    while !stars.is_empty() {
        let (distance, next_star) = min_heap.pop().unwrap().0;

        if stars.remove(&next_star) {
            total += 1 + distance;
            for &&star in &stars {
                min_heap.push(Reverse((taxicab_distance(next_star, star), star)));
            }
        }
    }

    total as u64
}
