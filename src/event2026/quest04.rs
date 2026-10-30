use std::{cmp::Reverse, hash::Hash};

use ahash::HashMap;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Participant {
    id: u32,
    quest: u32,
    part: u64,
    time: u64,
}

pub fn part1(notes: &str) -> u64 {
    parse(notes)
}

pub fn part2(notes: &str) -> u64 {
    parse(notes)
}

pub fn part3(notes: &str) -> u64 {
    let mut time_parts = HashMap::default();
    let mut player_scores = HashMap::default();
    let mut rows: Vec<Participant> = notes
        .lines()
        .map(|line| {
            let columns: Vec<_> = line.split_ascii_whitespace().collect();
            Participant {
                id: columns[0].split_once('=').unwrap().1.parse().unwrap(),
                quest: columns[1].split_once('=').unwrap().1.parse().unwrap(),
                part: columns[2].split_once('=').unwrap().1.parse().unwrap(),
                time: columns[3].split_once('=').unwrap().1.parse().unwrap(),
            }
        })
        .collect();

    // sort by parts
    rows.sort_unstable_by_key(|key| key.part);

    // get the proper total time for every part calculated
    for entry in &mut rows {
        let sum = time_parts.entry((entry.id, entry.quest)).or_insert(0);
        *sum += entry.time;
        entry.time = *sum;
    }

    // order by quest > part > time
    rows.sort_unstable_by(|a, b| {
        a.quest
            .cmp(&b.quest)
            .then_with(|| a.part.cmp(&b.part))
            .then_with(|| a.time.cmp(&b.time))
    });

    let mut rank = 0;
    // score quest groupings
    for quest in
        rows.chunk_by(|first, second| first.quest == second.quest && first.part == second.part)
    {
        for ties in quest.chunk_by(|first, second| first.time == second.time) {
            for &entry in ties {
                *player_scores.entry(entry.id).or_insert(0) +=
                    (50 * entry.part).saturating_sub(rank);
            }
            rank += ties.len() as u64;
        }
        rank = 0;
    }

    let mut score_board = player_scores.values().collect::<Vec<_>>();
    score_board.sort_unstable_by_key(|&score| Reverse(score));

    score_board
        .iter()
        .map(|&score| {
            let rank = score_board.partition_point(|&val| val > score) as u64;
            (rank + 1) * score
        })
        .sum()
}

fn parse(notes: &str) -> u64 {
    let mut map: HashMap<u32, Participant> = HashMap::default();

    notes.lines().for_each(|line| {
        let columns: Vec<_> = line.split_ascii_whitespace().collect();
        let participant = Participant {
            id: columns[0].split_once('=').unwrap().1.parse().unwrap(),
            quest: columns[1].split_once('=').unwrap().1.parse().unwrap(),
            part: 1,
            time: columns[3].split_once('=').unwrap().1.parse().unwrap(),
        };

        map.entry(participant.id)
            .and_modify(|v| {
                v.time += participant.time;
                v.part += 1;
            })
            .or_insert(participant);
    });

    map.values()
        .sorted_by(|a, b| b.part.cmp(&a.part).then_with(|| a.time.cmp(&b.time)))
        .enumerate()
        .map(|(i, participant)| (i as u64 + 1) * participant.time)
        .sum()
}
