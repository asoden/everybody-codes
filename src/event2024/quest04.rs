pub fn part1(notes: &str) -> i32 {
    nailed_it(notes)
}

pub fn part2(notes: &str) -> i32 {
    nailed_it(notes)
}

pub fn part3(notes: &str) -> u32 {
    nail_median(notes)
}

fn nailed_it(notes: &str) -> i32 {
    let mut nails: Vec<i32> = notes
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();
    nails.sort_unstable();

    let smallest = nails[0];
    let nails = &nails[1..];

    let mut sum = 0;

    for num in nails {
        sum += num - smallest;
    }

    sum
}

fn nail_median(notes: &str) -> u32 {
    let mut nails: Vec<i32> = notes
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();
    nails.sort_unstable();

    // minimize mean absolute error
    let median = nails[nails.len() / 2];
    nails.iter().map(|nail| nail.abs_diff(median)).sum()
}
