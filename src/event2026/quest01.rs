pub fn part1(notes: &str) -> u32 {
    parse(notes)
        .map(|pinecones| {
            let square = (pinecones - 1).isqrt() + 1;
            square * square - pinecones
        })
        .sum()
}

pub fn part2(notes: &str) -> u32 {
    parse(notes)
        .map(|pinecones| {
            let square = pinecones.div_ceil(4).max(2);
            square * 4 - pinecones
        })
        .sum()
}

pub fn part3(notes: &str) -> u32 {
    parse(notes)
        .map(|pinecones| {
            let square = ((pinecones).isqrt() + 1) | 1; // MAKE IT ALWAYS ODD BABYYYY
            square * square - pinecones - 1
        })
        .sum()
}

fn parse(notes: &str) -> impl Iterator<Item = u32> {
    notes
        .trim_end()
        .split(',')
        .flat_map(|pinecones| pinecones.parse())
}
