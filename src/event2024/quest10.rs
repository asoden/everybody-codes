pub fn part1(notes: &str) -> String {
    let mut grid = parse(notes);
    let mut word = String::with_capacity(16);

    find_runes(&mut grid, (0, 0));

    for row in grid.iter().take(6).skip(2) {
        for &rune in row.iter().take(6).skip(2) {
            word.push(rune as char);
        }
    }

    word
}

pub fn part2(notes: &str) -> i32 {
    notes
        .split("\n\n")
        .map(|panel_row| {
            let mut panel_grid = parse(panel_row);
            (0..panel_grid[0].len())
                .step_by(9)
                .filter_map(|x| {
                    find_runes(&mut panel_grid, (x, 0));
                    find_power(&mut panel_grid, (x, 0))
                })
                .sum::<i32>()
        })
        .sum()
}

pub fn part3(notes: &str) -> i32 {
    let mut grid = parse(notes);
    let mut anchor_points = Vec::new();
    let mut previous = -1;
    let mut total = 0;

    for y in (0..grid.len() - 2).step_by(6) {
        for x in (0..grid[0].len() - 2).step_by(6) {
            anchor_points.push((x, y));
        }
    }

    while previous < total {
        previous = total;

        anchor_points.retain(|&anchor| {
            find_runes(&mut grid, anchor);
            if let Some(power) = find_power(&mut grid, anchor) {
                total += power;
                false
            } else {
                true
            }
        });
    }
    total
}

fn parse(notes: &str) -> Vec<Vec<u8>> {
    notes.lines().map(|line| line.bytes().collect()).collect()
}

fn find_runes(grid: &mut [Vec<u8>], anchor_point: (usize, usize)) {
    let (anchor_point_x, anchor_point_y) = anchor_point;
    for y in 2..6 {
        for x in 2..6 {
            let mut row = 0;
            let mut column = 0;

            for i in 0..8 {
                row |= mask(grid[anchor_point_y + y][anchor_point_x + i]);
                column |= mask(grid[anchor_point_y + i][anchor_point_x + x]);
            }

            let unique = row & column;
            if unique.count_ones() == 1 {
                grid[anchor_point_y + y][anchor_point_x + x] = unmask_ascii(unique);
            }
        }
    }
}

fn find_power(grid: &mut [Vec<u8>], anchor_point: (usize, usize)) -> Option<i32> {
    let (anchor_point_x, anchor_point_y) = anchor_point;
    let mut position = 1;
    let mut total = 0;

    let mut solved = true;

    for y in 2..6 {
        for x in 2..6 {
            let rune = grid[anchor_point_y + y][anchor_point_x + x];
            if rune.is_ascii_uppercase() {
                total += position * (rune - b'A' + 1) as i32;
                position += 1;
                continue;
            }

            let mut unique = 0;
            let mut unknown = (0, 0);

            let mut update = |(x, y): (usize, usize)| match grid[y][x] {
                b'?' => unknown = (x, y),
                other => unique ^= mask(other),
            };

            for i in 0..8 {
                update((anchor_point_x + i, anchor_point_y + y));
                update((anchor_point_x + x, anchor_point_y + i));
            }

            if unique.count_ones() == 1 {
                let rune = unmask_ascii(unique);
                grid[anchor_point_y + y][anchor_point_x + x] = rune;
                grid[unknown.1][unknown.0] = rune;
                total += position * (rune - b'A' + 1) as i32;
                position += 1;
            } else {
                solved = false;
            }
        }
    }

    solved.then_some(total)
}

#[inline]
fn mask(character: u8) -> u32 {
    if character.is_ascii_uppercase() {
        1 << (character - b'A')
    } else {
        0
    }
}

#[inline]
fn unmask_ascii(mask: u32) -> u8 {
    b'A' + mask.trailing_zeros() as u8
}
