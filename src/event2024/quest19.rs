pub fn part1(notes: &str) -> String {
    decode(notes, 1)
}

pub fn part2(notes: &str) -> String {
    decode(notes, 100)
}

pub fn part3(notes: &str) -> String {
    decode(notes, 1048576000)
}

fn decode(notes: &str, rounds: i32) -> String {
    let (left, right) = notes.split_once("\n\n").unwrap();
    let width = right.find("\n").unwrap();
    let height = right.len() / width;
    let mut operations = left.bytes().cycle();

    let mut grid: Vec<Vec<u8>> = right.lines().map(|line| line.bytes().collect()).collect();

    let mut lookup: Vec<Vec<(i32, i32)>> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut temp = Vec::new();
        for (x, _) in row.iter().enumerate() {
            temp.push((x as i32, y as i32));
        }
        lookup.push(temp);
    }

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if operations.next().unwrap() == b'R' {
                rotate_right(&mut lookup, x, y);
            } else {
                rotate_left(&mut lookup, x, y);
            }
        }
    }

    let mut exponent = 1;

    while exponent <= rounds {
        if exponent & rounds != 0 {
            grid = unscramble(&grid, &lookup);
        }
        lookup = unscramble(&lookup, &lookup);
        exponent *= 2;
    }

    let left = find(&grid, b'>');
    let right = find(&grid, b'<');

    grid[left.1][left.0 + 1..right.0]
        .iter()
        .map(|c| *c as char)
        .collect()
}

fn rotate_right<T: Copy>(grid: &mut [Vec<T>], x: usize, y: usize) {
    let temp = grid[y - 1][x - 1];
    grid[y - 1][x - 1] = grid[y][x - 1];
    grid[y][x - 1] = grid[y + 1][x - 1];
    grid[y + 1][x - 1] = grid[y + 1][x];
    grid[y + 1][x] = grid[y + 1][x + 1];
    grid[y + 1][x + 1] = grid[y][x + 1];
    grid[y][x + 1] = grid[y - 1][x + 1];
    grid[y - 1][x + 1] = grid[y - 1][x];
    grid[y - 1][x] = temp;
}

fn rotate_left<T: Copy>(grid: &mut [Vec<T>], x: usize, y: usize) {
    let temp = grid[y - 1][x - 1];
    grid[y - 1][x - 1] = grid[y - 1][x];
    grid[y - 1][x] = grid[y - 1][x + 1];
    grid[y - 1][x + 1] = grid[y][x + 1];
    grid[y][x + 1] = grid[y + 1][x + 1];
    grid[y + 1][x + 1] = grid[y + 1][x];
    grid[y + 1][x] = grid[y + 1][x - 1];
    grid[y + 1][x - 1] = grid[y][x - 1];
    grid[y][x - 1] = temp;
}

fn find(grid: &[Vec<u8>], goal: u8) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == goal {
                return (x, y);
            }
        }
    }

    unreachable!()
}

fn unscramble<T: Copy>(grid: &Vec<Vec<T>>, lookup: &[Vec<(i32, i32)>]) -> Vec<Vec<T>> {
    let mut next = grid.clone();

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let dest = lookup[y][x];
            next[y][x] = grid[dest.1 as usize][dest.0 as usize];
        }
    }
    next
}
