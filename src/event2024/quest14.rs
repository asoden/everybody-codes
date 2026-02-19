use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

pub fn part1(notes: &str) -> i32 {
    let (plan, _) = grow(notes);
    plan.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1
}

pub fn part2(notes: &str) -> usize {
    grow(notes).0.len()
}

pub fn part3(notes: &str) -> i32 {
    let (plant, leaves) = grow(notes);
    let mut murkiness = HashMap::new();

    for leaf in leaves {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_front((leaf, 0));

        while let Some((segment @ (x, y, z), distance)) = queue.pop_front() {
            let neighbors = [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ];

            if x == 0 && z == 0 {
                *murkiness.entry(segment).or_insert(0) += distance;
            }

            for adjacent in neighbors {
                if plant.contains(&adjacent) && visited.insert(adjacent) {
                    queue.push_back((adjacent, distance + 1));
                }
            }
        }
    }
    *murkiness.values().min().unwrap()
}

fn grow(notes: &str) -> (HashSet<(i32, i32, i32)>, HashSet<(i32, i32, i32)>) {
    let mut plant = HashSet::new();
    let mut leaves = HashSet::new();

    for line in notes.lines() {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        let directions = line.bytes().filter(|b| b.is_ascii_uppercase());
        let values = line.split(',').map(|val| val[1..].parse::<i32>().unwrap());

        for (direction, magnitude) in directions.zip(values) {
            let (dx, dy, dz) = match direction {
                b'U' => (0, 1, 0),
                b'D' => (0, -1, 0),
                b'R' => (1, 0, 0),
                b'L' => (-1, 0, 0),
                b'F' => (0, 0, 1),
                b'B' => (0, 0, -1),
                _ => unreachable!(),
            };

            for _ in 0..magnitude {
                x += dx;
                y += dy;
                z += dz;
                plant.insert((x, y, z));
            }
        }
        leaves.insert((x, y, z));
    }
    (plant, leaves)
}
