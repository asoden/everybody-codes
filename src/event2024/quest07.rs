use ahash::HashMap;
use itertools::Itertools;

const TRACK1: &str = "=";

#[cfg(test)]
const TRACK2: &str = "\
+===++\
-=+=-=";
#[cfg(not(test))]
const TRACK2: &str = "\
-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=---=++==--\
+++==++=+=--==++==+++=++=+++=--=+=-=+=-+=-+=-+-=+=-=+=-+++=+==++++==---=+=+=-=";

const TRACK3: &str = "\
+=+++===-+++++=-==+--+=+===-++=====+--===++=-==+=++====-==-===+=+=--==++=+========-==\
=====++--+++=-++=-+=+==-=++=--+=-====++--+=-==++======+=++=-+==+=-==++=-=-=---++=-=++\
==++===--==+===++===---+++==++=+=-=====+==++===--==-==+++==+++=++=+===--==++--===+===\
==-=++====-+=-+--=+++=-+-===++====+++--=++====+=-=+===+=====-+++=+==++++==----=+=+=-=";

pub fn part1(notes: &str) -> String {
    let plans = parse(notes);
    let mut results = Vec::new();

    for (chariot, plan) in plans {
        results.push((score(TRACK1, 10, &plan), chariot));
    }

    results.sort_unstable_by_key(|(score, _)| *score);

    results
        .into_iter()
        .rev()
        .map(|(_, chariot)| chariot)
        .collect()
}

pub fn part2(notes: &str) -> String {
    let plans = parse(notes);
    let mut results = Vec::new();

    for (chariot, plan) in plans {
        results.push((score(TRACK2, 10, &plan), chariot));
    }

    results.sort_unstable_by_key(|(score, _)| *score);

    results
        .into_iter()
        .rev()
        .map(|(_, chariot)| chariot)
        .collect()
}

pub fn part3(notes: &str) -> usize {
    let rival_notes = parse(notes).into_values().next().unwrap();
    let rival_score = score(TRACK3, 11, &rival_notes);
    scoring_permutations()
        .into_iter()
        .filter(|score| *score > rival_score)
        .count()
}

fn parse(notes: &str) -> HashMap<String, String> {
    notes
        .lines()
        .map(|line| {
            let (chariot, actions) = line.split_once(":").unwrap();
            (chariot.to_string(), actions.replace(",", ""))
        })
        .collect()
}

fn score(track: &str, laps: usize, plan: &str) -> i64 {
    let track_iter = track.bytes().cycle();
    let plan_iter = plan.bytes().cycle();

    let size = track.len() * laps;
    let combined_iter = track_iter.zip(plan_iter).take(size);

    let mut power = 10;
    let mut essense = 0;

    for (terrain, segment) in combined_iter {
        let action = if terrain == b'=' { segment } else { terrain };
        match action {
            b'+' => power += 1,
            b'-' => power -= 1,
            _ => (),
        }
        essense += power;
    }

    essense
}

fn scoring_permutations() -> Vec<i64> {
    let original = "+++++---===";
    let mut scores = Vec::new();

    for plan in original.bytes().permutations(original.len()).unique() {
        let s: &str;
        unsafe {
            s = str::from_utf8_unchecked(&plan);
        }
        scores.push(score(TRACK3, 11, s));
    }

    scores
}
