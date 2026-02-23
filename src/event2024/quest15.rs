use pathfinding::prelude::dijkstra;

pub fn part1(notes: &str) -> u32 {
    let (map, herbs) = parse(notes);

    find_route(map, herbs)
}

pub fn part2(notes: &str) -> u32 {
    let (map, herbs) = parse(notes);

    find_route(map, herbs)
}

pub fn part3(notes: &str) -> u32 {
    let (map, herbs) = parse(notes);

    find_route(map, herbs)
}

fn parse(notes: &str) -> (Vec<Vec<u8>>, u32) {
    let mut map = Vec::with_capacity(notes.len() / notes.find('\n').unwrap());
    let mut herbs = 0;

    for line in notes.lines() {
        let mut row = Vec::with_capacity(line.len());
        for val in line.bytes() {
            if val.is_ascii_alphabetic() {
                herbs |= mask(val);
            }
            row.push(val);
        }
        map.push(row);
    }

    (map, herbs)
}

fn find_route(map: Vec<Vec<u8>>, herbs_possible: u32) -> u32 {
    let start = find_entry(&map);

    dijkstra(
        &(start, 0),
        |state| find_successors(state, &map),
        |(pos, herbs_picked)| *pos == start && *herbs_picked == herbs_possible,
    )
    .unwrap()
    .1
}

fn find_successors(state: &((i32, i32), u32), map: &[Vec<u8>]) -> Vec<(((i32, i32), u32), u32)> {
    let mut potential = vec![];

    let ((x, y), herbs) = state;

    for (adj_x, adj_y) in get_adjacent(map, *x, *y) {
        let cell = map[adj_y as usize][adj_x as usize];
        if cell != b'#' && cell != b'~' {
            let new_herbs = herbs | mask(cell);
            potential.push((((adj_x, adj_y), new_herbs), 1));
        }
    }
    potential
}

fn get_adjacent(grid: &[Vec<u8>], x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> + '_ {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter(move |(delta_x, delta_y)| {
            x + delta_x >= 0 && x + delta_x < width && y + delta_y >= 0 && y + delta_y < height
        })
        .map(move |(delta_x, delta_y)| {
            let new_x = x + delta_x;
            let new_y = y + delta_y;
            (new_x, new_y)
        })
}

fn find_entry(map: &[Vec<u8>]) -> (i32, i32) {
    for (x, val) in map[0].iter().enumerate() {
        if *val == b'.' {
            return (x as i32, 0);
        }
    }

    unreachable!()
}

#[inline]
fn mask(character: u8) -> u32 {
    if character.is_ascii_alphabetic() {
        1 << (character - b'A')
    } else {
        0
    }
}
