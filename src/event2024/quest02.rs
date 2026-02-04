pub fn part1(notes: &str) -> i32 {
    let (words, runic_phrase) = parse(notes, false);

    words.iter().fold(0, |acc, word| {
        acc + runic_phrase.match_indices(word).count() as i32
    })
}

pub fn part2(notes: &str) -> i32 {
    let (words, runic_phrase) = parse(notes, true);
    let mut runes = vec![0; runic_phrase.len()];

    for word in &words {
        for i in 0..runic_phrase.len() {
            if runic_phrase[i..].starts_with(word) {
                for n in 0..word.len() {
                    runes[i + n] = 1;
                }
            }
        }
    }

    runes.iter().sum()
}

pub fn part3(notes: &str) -> i32 {
    let (words, runic_phrase) = parse(notes, true);
    let grid: Vec<Vec<u8>> = runic_phrase.split_ascii_whitespace().map(|line| line.bytes().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut runes = vec![vec![0;width]; height];
    for word in words {
        for i in 0..width {
            for j in 0..height {
                check_right((i, j), &word, &grid, &mut runes, width);
                check_down((i, j), &word, &grid, &mut runes, height);
            }
        }
    }
    runes.iter().map(|row| row.iter().sum::<i32>()).sum()
}

fn check_right(position: (usize, usize), word: &str, grid: &[Vec<u8>], runes: &mut [Vec<i32>], width: usize) {
    let (mut x, y) = position;

    for &b in word.as_bytes() {
        if b != grid[y][x] {
            return;
        }
        x += 1;
        x %= width;
    }

    let (mut x, y) = position;

    for _ in 0..word.len() {
        runes[y][x] = 1;
        x += 1;
        x %= width;
    }
}

fn check_down(position: (usize, usize), word: &str, grid: &[Vec<u8>], runes: &mut [Vec<i32>], height: usize) {
    let (x, mut y) = position;

    for &b in word.as_bytes() {
        if y >= height || b != grid[y][x] {
            return;
        }
        y += 1;
    }

    let (x, mut y) = position;

    for _ in 0..word.len() {
        runes[y][x] = 1;
        y += 1;
    }
}

fn parse(notes: &str, reverse_words: bool) -> (Vec<String>, &str) {
    let (words, phrase) = notes.split_once("\n\n").unwrap();
    let mut runic_words: Vec<String> = Vec::new();

    for word in words[6..].split(',') {
        runic_words.push(String::from(word));
        if reverse_words {
            runic_words.push(word.chars().rev().collect());
        }
}
    (runic_words, phrase)
}
