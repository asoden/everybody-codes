use ahash::HashMap;
use ahash::HashSet;
use ahash::HashSetExt;

pub fn part1(notes: &str) -> usize {
    parse(notes, "NULL", false).len() - 1
}

pub fn part2(notes: &str) -> usize {
    parse(notes, "NULL", true).len() - 1
}

pub fn part3(notes: &str) -> usize {
    let [v1, v2, v3] = ["NULLv1", "NULLv2", "NULLv3"].map(|start| parse(notes, start, true));

    v1.union(&v2)
        .filter(|&item| v3.contains(item) || (v1.contains(item) && v2.contains(item)))
        .count()
}

fn parse<'a>(notes: &'a str, start: &'a str, invert: bool) -> HashSet<&'a str> {
    let mut ingredients = HashMap::default();

    for line in notes.lines() {
        let (item, rest) = line.split_once(':').unwrap();
        for component in rest.split(',') {
            let (key, value) = if invert {
                (component, item)
            } else {
                (item, component)
            };
            ingredients.entry(key).or_insert_with(Vec::new).push(value);
        }
    }

    let mut queue: Vec<&str> = vec![start];
    let mut seen = HashSet::new();

    while let Some(item) = queue.pop() {
        if seen.insert(item) {
            if let Some(temp) = ingredients.get(item) {
                queue.extend(temp);
            }
        }
    }

    seen
}
