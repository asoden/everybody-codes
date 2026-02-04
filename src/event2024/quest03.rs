use std::collections::VecDeque;

const ORTHAGONAL: &[(i32, i32)] =     &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1)
    ];

const DIAGONAL: &[(i32, i32)] =     &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

pub fn part1(notes: &str) -> i32 {
    diggy_diggy_hole(notes, false)
}

pub fn part2(notes: &str) -> i32 {
    diggy_diggy_hole(notes, false)
}

pub fn part3(notes: &str) -> i32 {
    diggy_diggy_hole(notes, true)
}

fn get_adjacent(grid: &[i32], width: i32, height: i32, x: i32, y: i32, is_diag: bool) -> impl Iterator<Item = i32> + '_ {
    if is_diag {DIAGONAL} else {ORTHAGONAL}
    .iter()
    .filter(move |(delta_x, delta_y)| {
        x + delta_x >= 0 && x + delta_x < width && y + delta_y >= 0 && y + delta_y < height
    })
    .map(move |(delta_x, delta_y)| {
        let new_x = x + delta_x;
        let new_y = y + delta_y;
        grid[(new_y * width + new_x) as usize]
    })
}

fn diggy_diggy_hole(notes: &str, is_diag: bool) -> i32 {
    let digsite: Vec<Vec<u8>> = notes.split_ascii_whitespace().map(|line| line.bytes().collect()).collect();
    let width = digsite[0].len();
    let height = digsite.len();

    let mut depths = vec![0; width * height];
    let mut dig_spots = VecDeque::new();
    digsite.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().filter(|(_, x)| **x == b'#').for_each(|(x, _)|{
            dig_spots.push_back((x, y));
        });
    });

    while let Some(spot) = dig_spots.pop_front() {
        let (x, y) = spot;
        if get_adjacent(&depths, width as i32, height as i32, x as i32, y as i32, is_diag).all(|n| {
             depths[y * width + x] <= n
            }) {
            depths[y * width + x] += 1;
            dig_spots.push_back(spot);
        }
    }

    // for j in 0..height {
    //     for i in 0..width {
    //         print!("{}", depths[j* width + i]);
    //     }
    //     println!();
    // }
    depths.iter().sum()
}