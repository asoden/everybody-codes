use ahash::{HashMap, HashMapExt};
use rayon::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Parameters {
    a: usize,
    b: usize,
    c: usize,
    x: usize,
    y: usize,
    z: usize,
    m: usize,
}

pub fn part1(notes: &str) -> usize {
    let parameters = parse(notes);
    parameters
        .iter()
        .map(|parameter| {
            eni(1, parameter.a, parameter.x, parameter.m)
                + eni(1, parameter.b, parameter.y, parameter.m)
                + eni(1, parameter.c, parameter.z, parameter.m)
        })
        .max()
        .unwrap()
}

pub fn part2(notes: &str) -> usize {
    let parameters = parse(notes);
    parameters
        .iter()
        .map(|parameter| {
            eni(
                mod_pow(parameter.a, parameter.x - 5, parameter.m),
                parameter.a,
                5,
                parameter.m,
            ) + eni(
                mod_pow(parameter.b, parameter.y - 5, parameter.m),
                parameter.b,
                5,
                parameter.m,
            ) + eni(
                mod_pow(parameter.c, parameter.z - 5, parameter.m),
                parameter.c,
                5,
                parameter.m,
            )
        })
        .max()
        .unwrap()
}

pub fn part3(notes: &str) -> usize {
    let parameters = parse(notes);
    parameters
        .par_iter()
        .map(|parameter| {
            cycle_eni(parameter.a, parameter.x, parameter.m)
                + cycle_eni(parameter.b, parameter.y, parameter.m)
                + cycle_eni(parameter.c, parameter.z, parameter.m)
        })
        .max()
        .unwrap()
}

fn eni(mut score: usize, n: usize, exp: usize, m: usize) -> usize {
    let mut power = 0;

    let mut result = 0;

    for _ in 0..exp {
        score = (score * n) % m;
        result += score * 10_usize.pow(power);
        power += if score < 10 { 1 } else { score.ilog10() + 1 };
    }

    result
}

fn parse(notes: &str) -> Vec<Parameters> {
    notes
        .lines()
        .map(|line| {
            let vals = line
                .split_ascii_whitespace()
                .map(|x| x.split_once('=').unwrap().1)
                .flat_map(|x| x.parse())
                .collect::<Vec<usize>>();
            Parameters {
                a: vals[0],
                b: vals[1],
                c: vals[2],
                x: vals[3],
                y: vals[4],
                z: vals[5],
                m: vals[6],
            }
        })
        .collect()
}

fn mod_pow(mut base: usize, mut exponent: usize, m: usize) -> usize {
    let mut result = 1;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base) % m;
        }

        base = (base * base) % m;

        exponent >>= 1;
    }

    result
}

fn cycle_eni(n: usize, exp: usize, m: usize) -> usize {
    let mut score = 1;
    let mut total = 0;
    let mut index = 0;

    let mut sums = Vec::with_capacity(m);
    let mut history = HashMap::with_capacity(m);

    sums.push(0);

    loop {
        score = (score * n) % m;
        total += score;
        index += 1;

        if let Some(previous) = history.insert(score, index) {
            let cycle = index - previous;
            let cycle_total = total - sums[previous];

            let remaining = exp - index + 1;
            let quotient = remaining / cycle;
            let remainder = remaining % cycle;

            return (total - score)
                + (quotient * cycle_total)
                + (sums[previous + remainder - 1] - sums[previous - 1]);
        }

        sums.push(total);
    }
}
