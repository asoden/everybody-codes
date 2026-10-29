pub fn part1(notes: &str) -> u64 {
    wtf_is_this_numbering(notes)
}

pub fn part2(notes: &str) -> u64 {
    wtf_is_this_numbering(notes)
}

pub fn part3(notes: &str) -> u64 {
    notes
        .lines()
        .map(|line| {
            line[5..]
                .split_once(' ')
                .into_iter()
                .filter_map(|(left, right)| {
                    let left = left.as_bytes();
                    let right = right.as_bytes();

                    let vote = right
                        .iter()
                        .fold(0_u64, |acc, &x| 10 * acc + (x - b'0') as u64);

                    let number: u64 = left
                        .into_iter()
                        .rev()
                        .cycle()
                        .zip(right.into_iter().rev())
                        .enumerate()
                        .map(|(i, (power, vote))| {
                            u64::from(vote - b'0') * u64::from(power - b'0').pow(i as u32)
                        })
                        .sum();

                    (0..vote)
                        .all(|y| {
                            let potential: u64 = left
                                .into_iter()
                                .rev()
                                .cycle()
                                .zip(y.to_string().as_bytes().into_iter().rev())
                                .enumerate()
                                .map(|(i, (power, vote))| {
                                    u64::from(vote - b'0') * u64::from(power - b'0').pow(i as u32)
                                })
                                .sum();
                            potential != number
                        })
                        .then_some(number)
                })
                .sum::<u64>()
        })
        .sum()
}

fn wtf_is_this_numbering(notes: &str) -> u64 {
    notes
        .lines()
        .map(|line| line[4..].split_once(' ').into_iter().map(calc).sum::<u64>())
        .sum()
}

fn calc((left, right): (&str, &str)) -> u64 {
    let left = left.as_bytes();
    let right = right.as_bytes();
    // Some(
    left.into_iter()
        .rev()
        .cycle()
        .zip(right.into_iter().rev())
        .enumerate()
        .map(|(i, (power, vote))| u64::from(vote - b'0') * u64::from(power - b'0').pow(i as u32))
        .sum::<u64>()
}
