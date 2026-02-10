use ahash::{HashMap, HashMapExt};

pub fn part1(notes: &str) -> String {
    search_tree(notes, false)
}

pub fn part2(notes: &str) -> String {
    search_tree(notes, true)
}

pub fn part3(notes: &str) -> String {
    search_tree(notes, true)
}

fn search_tree(notes: &str, first_letter: bool) -> String {
    let mut apples = Vec::new();
    let mut parents = HashMap::new();

    for line in notes.lines() {
        let (parent, children) = line.split_once(":").unwrap();

        for child in children.split(',') {
            if child == "@" {
                apples.push(parent);
            } else {
                parents.insert(child, parent);
            }
        }
    }

    let mut branches = HashMap::new();

    'apples: for apple in apples {
        let mut current = apple;
        let mut path = vec!["@", apple];

        while let Some(&next) = parents.get(current) {
            if path.contains(&next) {
                continue 'apples;
            }
            current = next;
            path.push(next);
        }

        branches
            .entry(path.len())
            .or_insert_with(Vec::new)
            .push(path);
    }

    let most_powerful = &branches.values().find(|path| path.len() == 1).unwrap()[0];
    most_powerful
        .iter()
        .rev()
        .map(|path| if first_letter { &path[..1] } else { path })
        .collect()
}
