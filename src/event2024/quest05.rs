use std::{array::from_fn, collections::HashMap};
// use ahash::{HashMap, HashMapExt};

type Dance = [Vec<usize>; 4];

pub fn part1(notes: &str) -> usize {
    let mut dance = dance_grid(notes);

    for i in 0..10 {
        dance_dance(&mut dance, i);
    }
    shout(&dance)
}

pub fn part2(notes: &str) -> usize {
    let mut dance = dance_grid(notes);
    let mut seen = HashMap::new();
    let mut round = 0;

    loop {
        dance_dance(&mut dance, round);
        round += 1;

        let val = shout(&dance);
        let count = seen.entry(val).and_modify(|x| *x += 1).or_insert(1);

        if *count == 2024 {
            break round * shout(&dance);
        }
    }
}

pub fn part3(notes: &str) -> i32 {
    4
}

fn dance_grid(notes: &str) -> Dance {
    let mut dance = from_fn(|_| Vec::new());

    for (i, num) in notes.split_ascii_whitespace().enumerate() {
        dance[i % 4].push(num.parse().unwrap())
    }
    dance
}

#[inline]
fn shout(dance: &Dance) -> usize {
    dance[0][0] * 1000 + dance[1][0] * 100 + dance[2][0] * 10 + dance[3][0]
}

fn dance_dance(dance: &mut Dance, round: usize) {
    let clappist = dance[round % 4].remove(0);
    let destination = (round + 1) % 4;
    let partners = 2 * dance[destination].len();
    let remainder = (clappist - 1) % partners;
    let offset = remainder.min(partners - remainder);

    dance[destination].insert(offset, clappist);
}

