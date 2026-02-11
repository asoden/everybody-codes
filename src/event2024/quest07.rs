use ahash::HashMap;

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
    let mut scores = Vec::new();

    fn permutate(scores: &mut Vec<i64>, plan: &mut String, plus: u8, minus: u8, equal: u8) {
        if plus + minus + equal == 0 {
            scores.push(score(TRACK3, 11, plan));
            return;
        }

        if plus > 0 {
            plan.push('+');
            permutate(scores, plan, plus - 1, minus, equal);
            plan.pop();
        }
        if minus > 0 {
            plan.push('-');
            permutate(scores, plan, plus, minus - 1, equal);
            plan.pop();
        }
        if equal > 0 {
            plan.push('=');
            permutate(scores, plan, plus, minus, equal - 1);
            plan.pop();
        }
    }

    permutate(&mut scores, &mut String::with_capacity(11), 5, 3, 3);
    scores
}
