pub fn part1(notes: &str) -> u64 {
    parse(notes)
        .iter()
        .map(|(x, y)| {
            let tier = x + y - 1;
            let travled_x = (x - 1 + 100) % tier;
            (travled_x + 1) + 100 * (tier - travled_x)
        })
        .sum()
}

pub fn part2(notes: &str) -> u64 {
    find_golden_line(notes)
}

pub fn part3(notes: &str) -> u64 {
    find_golden_line(notes)
}

fn parse(notes: &str) -> Vec<(u64, u64)> {
    notes
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(" ").unwrap();
            let x = x.split_once("=").unwrap().1.parse().unwrap();
            let y = y.split_once("=").unwrap().1.parse().unwrap();
            (x, y)
        })
        .collect()
}

fn find_golden_line(notes: &str) -> u64 {
    let mut time = 0;
    let mut factor = 1;

    for (x, y) in parse(notes) {
        let tier = x + y - 1;

        while (x + time) % tier != 0 {
            time += factor;
        }

        factor *= tier;
    }

    time
}
