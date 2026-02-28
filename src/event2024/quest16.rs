use ahash::{HashMap, HashMapExt};

use crate::util::math::lcm;

pub fn part1(notes: &str) -> String {
    let (spins, cat_faces) = parse(notes);
    let mut result = Vec::new();

    for (number, faces) in spins.iter().zip(cat_faces.iter()) {
        let index = (number * 100) % faces.len();
        result.push(faces[index]);
    }

    result.join(" ")
}

pub fn part2(notes: &str) -> usize {
    let (spins, cat_faces) = parse(notes);

    let lcm = cat_faces
        .iter()
        .fold(1, |acc, face| lcm(acc, face.len() as u64)) as usize;

    let quotient = 202420242024 / lcm;
    let remainder = 202420242024 % lcm;

    let coins = |loops| score(&spins, &cat_faces, loops + 1, 0, 0);

    let num: usize = (0..remainder).map(coins).sum();
    let remaining: usize = (remainder..lcm).map(coins).sum();

    num + quotient * (num + remaining)
}

pub fn part3(notes: &str) -> String {
    let (spins, cat_faces) = parse(notes);

    let mut current_max = [0; 515];
    let mut next_max = [0; 515];
    let mut current_min = [usize::MAX; 515];
    let mut next_min = [usize::MAX; 515];

    let middle = 257;
    current_min[middle] = 0;

    for spin_val in 1..=256 {
        for i in (middle - spin_val)..=(middle + spin_val) {
            let left = middle.saturating_sub(i);
            let right = i.saturating_sub(middle);
            let score = score(&spins, &cat_faces, spin_val, left, right);

            next_max[i] = score + current_max[i - 1..=i + 1].iter().max().unwrap();
            next_min[i] = score + current_min[i - 1..=i + 1].iter().min().unwrap();
        }
        (current_max, next_max) = (next_max, current_max);
        (current_min, next_min) = (next_min, current_min);
    }

    format!(
        "{} {}",
        current_max.iter().max().unwrap(),
        current_min.iter().min().unwrap()
    )
}

fn parse(notes: &str) -> (Vec<usize>, Vec<Vec<&str>>) {
    let (spins, faces) = notes.split_once("\n\n").unwrap();
    let spins: Vec<_> = spins.split(',').map(|x| x.parse().unwrap()).collect();

    let mut cat_faces = vec![Vec::new(); spins.len()];

    for line in faces.lines() {
        for i in (0..line.len()).step_by(4) {
            let whimsy_face = &line[i..i + 3];
            if whimsy_face != "   " {
                cat_faces[i / 4].push(whimsy_face);
            }
        }
    }

    (spins, cat_faces)
}

fn score(
    spins: &[usize],
    cat_faces: &Vec<Vec<&str>>,
    spin_val: usize,
    left: usize,
    right: usize,
) -> usize {
    let mut symbol_count = HashMap::new();

    for (number, faces) in spins.iter().zip(cat_faces.iter()) {
        let size = faces.len();
        let index = (number * spin_val + right + (size - left % size)) % size;

        let face = faces[index].as_bytes();

        symbol_count
            .entry(face[0])
            .and_modify(|val| *val += 1)
            .or_insert(1);
        symbol_count
            .entry(face[2])
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    symbol_count
        .values()
        .map(|val: &usize| val.saturating_sub(2))
        .sum()
}
