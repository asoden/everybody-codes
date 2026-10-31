const ORTHAGONAL: &[(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

const DIAGONAL: &[(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

pub fn part1(notes: &str) -> usize {
    let mut map = make_grid(notes);
    spread_land(&mut map);
    count(&map, b'#')
}

pub fn part2(notes: &str) -> usize {
    let mut map = make_grid(notes);
    spread_land(&mut map);
    spread_sand(&mut map);
    count(&map, b'=')
}

pub fn part3(notes: &str) -> usize {
    let mut map = make_grid(notes);
    spread_land(&mut map);
    spread_sand(&mut map);

    // first tree placements
    spread_tree(&mut map, |map, (x, y)| {
        let height = map.len() as i32;
        let width = map[0].len() as i32;
        ORTHAGONAL.iter().all(|(dx, dy)| {
            get_adjacent(map, width, height, x + dx, y + dy, &[b'#', b'@']).count() == 8
        })
    });

    //two step grow
    for _ in 0..2 {
        spread_tree(&mut map, |map, (x, y)| {
            let height = map.len() as i32;
            let width = map[0].len() as i32;
            get_adjacent(map, width, height, x, y, &[b'@']).count() >= 4
                && get_adjacent(map, width, height, x, y, &[b'=']).count() == 0
        });
    }

    count(&map, b'@')
}

fn make_grid(notes: &str) -> Vec<Vec<u8>> {
    notes
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect())
        .collect()
}

fn spread_land(map: &mut [Vec<u8>]) {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let queue = vec![(width / 2, height / 2)];

    spread(map, queue, &[b'#', b'1'], b'#');
}

fn spread_sand(map: &mut [Vec<u8>]) {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut beaches = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'#'
                && get_adjacent(map, width, height, x as i32, y as i32, &[b'#']).count() < 8
            {
                beaches.push((x as i32, y as i32));
            }
        }
    }

    spread(map, beaches, &[b'='], b'=');
}

fn spread_tree(map: &mut [Vec<u8>], grow_check: fn(&[Vec<u8>], (i32, i32)) -> bool) {
    let mut to_grow = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'#' && grow_check(map, (x as i32, y as i32)) {
                to_grow.push((x, y));
            }
        }
    }

    for (x, y) in to_grow {
        map[y][x] = b'@';
    }
}

fn spread(map: &mut [Vec<u8>], mut queue: Vec<(i32, i32)>, spreaders: &[u8], tile: u8) {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    queue
        .iter()
        .for_each(|(x, y)| map[*y as usize][*x as usize] = tile);

    while let Some((x, y)) = queue.pop() {
        for (modx, mody) in DIAGONAL.iter().map(|(dx, dy)| (x + dx, y + dy)) {
            if contains(modx, mody, height, width)
                && map[mody as usize][modx as usize] != tile
                && get_adjacent(&map, width, height, modx, mody, spreaders).count() >= 5
            {
                map[mody as usize][modx as usize] = tile;
                queue.push((modx, mody));
            }
        }
    }
}

fn count(map: &[Vec<u8>], symbol: u8) -> usize {
    map.iter()
        .map(|row| row.iter().filter(|&&c| c == symbol).count())
        .sum()
}

fn contains(x: i32, y: i32, height: i32, width: i32) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

fn get_adjacent<'a>(
    grid: &'a [Vec<u8>],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    symbols: &'a [u8],
) -> impl Iterator<Item = u8> + 'a {
    DIAGONAL
        .iter()
        .filter(move |(delta_x, delta_y)| {
            x + delta_x >= 0 && x + delta_x < width && y + delta_y >= 0 && y + delta_y < height
        })
        .map(move |(delta_x, delta_y)| {
            let new_x = x + delta_x;
            let new_y = y + delta_y;
            grid[new_y as usize][new_x as usize]
        })
        .filter(move |c| symbols.contains(c))
}
