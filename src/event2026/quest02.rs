use ahash::AHashSet;

pub fn part1(notes: &str) -> usize {
    parse(notes)
}

pub fn part2(notes: &str) -> usize {
    parse(notes)
}

pub fn part3(notes: &str) -> usize {
    parse(notes)
}

fn parse(notes: &str) -> usize {
    notes
        .lines()
        .map(|line| {
            let mut alloys = AHashSet::new();
            let mut above = [[true; 10]; 10];
            let mut result = 0;

            for val in line.as_bytes().split(|&x| x == b' ') {
                let left = (val[0] - b'0') as usize;
                let right = (val[2] - b'0') as usize;
                let (small, big) = if val[1] == b'<' {
                    (left, right)
                } else {
                    (right, left)
                };

                alloys.insert(big);
                alloys.insert(small);
                above[small][big] = false;
            }

            while let Some(&num) = alloys
                .iter()
                .find(|&&big| alloys.iter().all(|&small| above[big][small]))
            {
                alloys.remove(&num);
                result = result * 10 + num;
            }

            result
        })
        .sum()
}
