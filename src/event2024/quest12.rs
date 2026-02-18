pub fn part1(notes: &str) -> i32 {
    tracking(notes)
}

pub fn part2(notes: &str) -> i32 {
    tracking(notes)
}

pub fn part3(notes: &str) -> i32 {
    notes
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(" ").unwrap();
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();
            score(x / 2, y - (x - (x / 2)))
        })
        .sum()
}

fn tracking(notes: &str) -> i32 {
    let width = notes.find("\n").unwrap();
    let height = notes.len() / width;

    notes
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    b'T' => score(x as i32 - 1, (height - 2 - y) as i32),
                    b'H' => 2 * score(x as i32 - 1, (height - 2 - y) as i32),
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum()
}

fn score(x: i32, y: i32) -> i32 {
    for catapult in 0..3 {
        let y = y - catapult;
        let distance = x + y;

        if x < y {
            continue;
        }

        if x <= 2 * y {
            return (catapult + 1) * y;
        }

        if distance % 3 == 0 {
            return (catapult + 1) * (distance / 3);
        }
    }

    unreachable!()
}
