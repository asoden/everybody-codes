use pathfinding::prelude::dijkstra;

pub fn part1(notes: &str) -> u64 {
    let map = parse(notes);

    find_route(map, b'S', b'E')
}

pub fn part2(notes: &str) -> u64 {
    let map = parse(notes);

    find_route(map, b'S', b'E')
}

pub fn part3(notes: &str) -> u64 {
    let map = parse(notes);

    find_route(map, b'E', b'S')
}

fn parse(notes: &str) -> Vec<Vec<u8>> {
    notes.lines().map(|line| line.bytes().collect()).collect()
}

fn find_route(map: Vec<Vec<u8>>, start: u8, end: u8) -> u64 {
    let start = find(&map, start);

    dijkstra(
        &start,
        |state| find_successors(state, &map),
        |(x, y)| map[*y as usize][*x as usize] == end,
    )
    .unwrap()
    .1
}

fn find_successors(state: &(i32, i32), map: &Vec<Vec<u8>>) -> Vec<((i32, i32), u64)> {
    let mut potential = vec![];

    let (x, y) = state;

    for (adj_x, adj_y) in get_adjacent(map, *x, *y) {
        if map[adj_y as usize][adj_x as usize] != b'#' {
            let next = to_value(map[adj_y as usize][adj_x as usize]);
            let current = to_value(map[*y as usize][*x as usize]);
            let diff = next.abs_diff(current);
            let looped_diff = diff.min(10 - diff);
            potential.push(((adj_x, adj_y), (looped_diff + 1)));
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

fn find(map: &Vec<Vec<u8>>, symbol: u8) -> (i32, i32) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == symbol {
                return (x as i32, y as i32);
            }
        }
    }

    unreachable!()
}

fn to_value(val: u8) -> u64 {
    if val.is_ascii_digit() {
        (val - b'0') as u64
    } else {
        0
    }
}
