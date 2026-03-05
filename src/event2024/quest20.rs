use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Airspace {
    Cold,
    Hot,
    Empty,
    Beacon(u8),
    Impassible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Glider {
    facing: Direction,
    position: Point3d,
    time: usize,
    beacon_segment: usize,
}

pub fn part1(notes: &str) -> i64 {
    let air_map = parse_map(notes);

    fly_the_sky(air_map, 100)
}

pub fn part2(notes: &str) -> usize {
    let air_map = parse_map(notes);

    fly_the_course(air_map, 10000)
}

pub fn part3(notes: &str) -> i32 {
    let altitude = 384400;
    let air_map = parse_map(notes);

    let width = air_map[0].len();
    let height = air_map.len();

    let mut column_costs: Vec<(usize, i32)> = Vec::new();

    let start = air_map[0]
        .iter()
        .position(|v| *v == Airspace::Beacon(b'S'))
        .unwrap();

    'columns: for x in 0..width {
        let mut flight_cost: i32 = 0;
        for y in 0..height {
            match air_map[y][x] {
                Airspace::Cold => flight_cost -= 2,
                Airspace::Hot => flight_cost += 1,
                Airspace::Empty | Airspace::Beacon(_) => flight_cost -= 1,
                Airspace::Impassible => continue 'columns,
            }
        }
        column_costs.push((x, flight_cost));
    }

    let &(return_col, cost) = column_costs
        .iter()
        .max_by(|a, b| {
            (a.1 - (a.0.abs_diff(start) as i32)).cmp(&(b.1 - (b.0.abs_diff(start) as i32)))
        })
        .unwrap();

    let distance_to_col = start.abs_diff(return_col) as i32;

    let adjusted_altitude = altitude - distance_to_col;
    let times_looped = adjusted_altitude / cost.abs();
    let remainder = adjusted_altitude % cost.abs();

    (times_looped * height as i32) + remainder + distance_to_col
}

fn parse_map(notes: &str) -> Vec<Vec<Airspace>> {
    notes
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'.' => Airspace::Empty,
                    b'-' => Airspace::Cold,
                    b'+' => Airspace::Hot,
                    b'#' => Airspace::Impassible,
                    c if c.is_ascii_uppercase() => Airspace::Beacon(c),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn in_bounds<T>(grid: &[Vec<T>], point: &Point3d) -> bool {
    point.y >= 0
        && point.x >= 0
        && point.y < (grid.len() as i64)
        && point.x < (grid[0].len() as i64)
}

fn fly_the_sky(sky_map: Vec<Vec<Airspace>>, time: usize) -> i64 {
    let mut todo = VecDeque::new();
    let start = sky_map[0]
        .iter()
        .position(|v| *v == Airspace::Beacon(b'S'))
        .map(|point| Point3d {
            x: point as i64,
            y: 0,
            z: 1001,
        })
        .unwrap();
    let glider = Glider {
        facing: Direction::South,
        position: start,
        time: 0,
        beacon_segment: 0,
    };
    let mut best_location = HashMap::new();

    todo.push_back(glider);

    while let Some(mut glider) = todo.pop_front() {
        glider.position.z += match sky_map[glider.position.y as usize][glider.position.x as usize] {
            Airspace::Cold => -2,
            Airspace::Hot => 1,
            Airspace::Empty | Airspace::Beacon(_) => -1,
            Airspace::Impassible => continue,
        };

        if glider.position.z
            <= *best_location
                .get(&(
                    Point {
                        x: glider.position.x,
                        y: glider.position.y,
                    },
                    glider.facing,
                ))
                .unwrap_or(&0)
        {
            continue;
        }

        best_location.insert(
            (
                Point {
                    x: glider.position.x,
                    y: glider.position.y,
                },
                glider.facing,
            ),
            glider.position.z,
        );
        if glider.time == time {
            continue;
        }

        let next_facing = match glider.facing {
            Direction::North => [Direction::East, Direction::North, Direction::West],
            Direction::East => [Direction::South, Direction::East, Direction::North],
            Direction::South => [Direction::West, Direction::South, Direction::East],
            Direction::West => [Direction::North, Direction::West, Direction::South],
        };

        for new_facing in next_facing {
            let next = match new_facing {
                Direction::North => Point3d {
                    x: glider.position.x,
                    y: glider.position.y - 1,
                    z: glider.position.z,
                },
                Direction::East => Point3d {
                    x: glider.position.x + 1,
                    y: glider.position.y,
                    z: glider.position.z,
                },
                Direction::South => Point3d {
                    x: glider.position.x,
                    y: glider.position.y + 1,
                    z: glider.position.z,
                },
                Direction::West => Point3d {
                    x: glider.position.x - 1,
                    y: glider.position.y,
                    z: glider.position.z,
                },
            };

            if in_bounds(&sky_map, &next)
                && sky_map[next.y as usize][next.x as usize] != Airspace::Impassible
            {
                todo.push_back(Glider {
                    facing: new_facing,
                    position: next,
                    time: glider.time + 1,
                    beacon_segment: 0,
                });
            }
        }
    }
    *best_location.values().max().unwrap()
}

fn fly_the_course(sky_map: Vec<Vec<Airspace>>, target_altitude: i64) -> usize {
    let mut todo = VecDeque::new();
    let start = sky_map[0]
        .iter()
        .position(|v| *v == Airspace::Beacon(b'S'))
        .map(|point| Point3d {
            x: point as i64,
            y: 0,
            z: target_altitude + 1,
        })
        .unwrap();
    let glider = Glider {
        facing: Direction::South,
        position: start,
        time: 0,
        beacon_segment: 0,
    };
    let mut best_location = HashMap::new();
    let beacons = [b'A', b'B', b'C', b'S'];

    todo.push_back(glider);

    while let Some(mut glider) = todo.pop_front() {
        glider.position.z += match sky_map[glider.position.y as usize][glider.position.x as usize] {
            Airspace::Cold => -2,
            Airspace::Hot => 1,
            Airspace::Empty | Airspace::Beacon(_) => -1,
            Airspace::Impassible => continue,
        };

        if glider.position.z
            <= *best_location
                .get(&(
                    Point {
                        x: glider.position.x,
                        y: glider.position.y,
                    },
                    glider.facing,
                    glider.beacon_segment,
                ))
                .unwrap_or(&0)
        {
            continue;
        }

        if sky_map[glider.position.y as usize][glider.position.x as usize]
            == Airspace::Beacon(beacons[glider.beacon_segment])
        {
            if beacons[glider.beacon_segment] == b'S' {
                if glider.position.z >= target_altitude {
                    return glider.time;
                }
            } else {
                glider.beacon_segment += 1;
            }
        }

        best_location.insert(
            (
                Point {
                    x: glider.position.x,
                    y: glider.position.y,
                },
                glider.facing,
                glider.beacon_segment,
            ),
            glider.position.z,
        );

        let next_facing = match glider.facing {
            Direction::North => [Direction::East, Direction::North, Direction::West],
            Direction::East => [Direction::South, Direction::East, Direction::North],
            Direction::South => [Direction::West, Direction::South, Direction::East],
            Direction::West => [Direction::North, Direction::West, Direction::South],
        };

        for new_facing in next_facing {
            let next = match new_facing {
                Direction::North => Point3d {
                    x: glider.position.x,
                    y: glider.position.y - 1,
                    z: glider.position.z,
                },
                Direction::East => Point3d {
                    x: glider.position.x + 1,
                    y: glider.position.y,
                    z: glider.position.z,
                },
                Direction::South => Point3d {
                    x: glider.position.x,
                    y: glider.position.y + 1,
                    z: glider.position.z,
                },
                Direction::West => Point3d {
                    x: glider.position.x - 1,
                    y: glider.position.y,
                    z: glider.position.z,
                },
            };
            if in_bounds(&sky_map, &next)
                && sky_map[next.y as usize][next.x as usize] != Airspace::Impassible
            {
                todo.push_back(Glider {
                    facing: new_facing,
                    position: next,
                    time: glider.time + 1,
                    beacon_segment: glider.beacon_segment,
                });
            }
        }
    }
    unreachable!()
}
