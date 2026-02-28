use std::collections::VecDeque;

use ahash::{HashSet, HashSetExt};

pub fn part1(notes: &str) -> u64 {
    let (map, tree_count) = parse(notes);
    let starts = [(0, 1)];

    bfs(map, &starts, tree_count)
}

pub fn part2(notes: &str) -> u64 {
    let (map, tree_count) = parse(notes);
    let starts = [(0, 1), ((map[0].len() - 1) as i32, (map.len() - 2) as i32)];

    bfs(map, &starts, tree_count)
}

pub fn part3(notes: &str) -> u32 {
    let (map, _) = parse(notes);

    let mut distance_map = vec![vec![0; map[0].len()]; map.len()];
    let trees = tree_list(&map);

    for tree in trees {
        flood_fill(&map, &mut distance_map, tree);
    }

    let mut best = u32::MAX;
    let mut start = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'.' && distance_map[y][x] < best {
                best = distance_map[y][x];
                start = (x as i32, y as i32);
            }
        }
    }

    flood_fill(&map, &mut distance_map, start)
}

fn parse(notes: &str) -> (Vec<Vec<u8>>, u32) {
    let mut trees = 0;
    let mut map = Vec::new();
    for line in notes.lines() {
        let mut temp = Vec::with_capacity(line.len());
        for c in line.bytes() {
            if c == b'P' {
                trees += 1;
            }
            temp.push(c);
        }
        map.push(temp);
    }

    (map, trees)
}

fn tree_list(map: &[Vec<u8>]) -> Vec<(i32, i32)> {
    let mut tree_list = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'P' {
                tree_list.push((x as i32, y as i32));
            }
        }
    }

    tree_list
}

fn flood_fill(map: &[Vec<u8>], distance_map: &mut [Vec<u32>], start: (i32, i32)) -> u32 {
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();
    let mut total = 0;

    todo.push_back((start, 0));
    seen.insert(start);

    while let Some((point, cost)) = todo.pop_front() {
        let (x, y) = point;

        distance_map[y as usize][x as usize] += cost;

        if map[y as usize][x as usize] == b'P' {
            total += cost;
        }

        for adjacent in get_adjacent(map, x, y) {
            if map[adjacent.1 as usize][adjacent.0 as usize] != b'#' && seen.insert(adjacent) {
                todo.push_back((adjacent, cost + 1));
            }
        }
    }

    total
}

fn bfs(map: Vec<Vec<u8>>, starts: &[(i32, i32)], mut tree_count: u32) -> u64 {
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();

    for &start in starts {
        todo.push_back((start, 0));
        seen.insert(start);
    }

    while let Some((point, cost)) = todo.pop_front() {
        let (x, y) = point;

        if map[y as usize][x as usize] == b'P' {
            tree_count -= 1;
            if tree_count == 0 {
                return cost;
            }
        }

        for adjacent in get_adjacent(&map, x, y) {
            if map[adjacent.1 as usize][adjacent.0 as usize] != b'#' && seen.insert(adjacent) {
                todo.push_back((adjacent, cost + 1));
            }
        }
    }

    unreachable!()
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
