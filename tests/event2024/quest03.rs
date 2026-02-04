use everybody_codes::event2024::quest03::*;

const EXAMPLE1: &str = "..........\n..###.##..\n...####...\n..######..\n..######..\n...####...\n..........";
const EXAMPLE3: &str = "..........\n..###.##..\n...####...\n..######..\n..######..\n...####...\n..........";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 35);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE1), 35);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 29);
}