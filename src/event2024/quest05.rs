use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use std::array::from_fn;

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

        let count = seen
            .entry(quick_shout(&dance))
            .and_modify(|x| *x += 1)
            .or_insert(1);

        if *count == 2024 {
            break round * shout(&dance);
        }
    }
}

pub fn part3(notes: &str) -> usize {
    let mut dance = dance_grid(notes);
    let mut history = HashSet::new();
    let mut round = 0;

    while history.insert(dance.clone()) {
        dance_dance(&mut dance, round);
        round += 1;
    }

    let biggest = history
        .iter()
        .max_by_key(|dance_instance| quick_shout(dance_instance))
        .unwrap();
    shout(biggest)
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
    format!(
        "{}{}{}{}",
        dance[0][0], dance[1][0], dance[2][0], dance[3][0]
    )
    .parse()
    .unwrap()
}

#[inline]
fn quick_shout(dance: &Dance) -> usize {
    (dance[0][0] << 48) | (dance[1][0] << 32) | (dance[2][0] << 16) | (dance[3][0])
}

fn dance_dance(dance: &mut Dance, round: usize) {
    let clappist = dance[round % 4].remove(0);
    let destination = (round + 1) % 4;
    let partners = 2 * dance[destination].len();
    let remainder = (clappist - 1) % partners;
    let offset = remainder.min(partners - remainder);

    dance[destination].insert(offset, clappist);
}
