pub fn part1(notes: &str) -> usize {
    let stamps = [1, 3, 5, 10];
    beetle_count(notes, &stamps, |beetles, dots| beetles[dots])
}

pub fn part2(notes: &str) -> usize {
    let stamps = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
    beetle_count(notes, &stamps, |beetles, dots| beetles[dots])
}

pub fn part3(notes: &str) -> usize {
    let stamps = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    beetle_count(notes, &stamps, |beetles, dots| {
        let mid_split = dots / 2;
        (mid_split..=mid_split + 50)
            .map(|i| beetles[i] + beetles[dots - i])
            .min()
            .unwrap()
    })
}

fn beetle_count(notes: &str, stamps: &[usize], count: fn(&[usize], usize) -> usize) -> usize {
    let sparkleballs: Vec<usize> = notes.lines().map(|line| line.parse().unwrap()).collect();
    let max = 1 + sparkleballs.iter().max().unwrap();

    let mut beetle_cache = vec![max; max];
    beetle_cache[0] = 0;

    for &stamp in stamps {
        for i in stamp..max {
            beetle_cache[i] = beetle_cache[i].min(beetle_cache[i - stamp] + 1)
        }
    }

    sparkleballs
        .iter()
        .map(|&dots| count(&beetle_cache, dots))
        .sum()
}
