use everybody_codes::event2024::quest08::*;

const EXAMPLE1: &str = "13";
const EXAMPLE2: &str = "3";
const EXAMPLE3: &str = "2";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 21);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 27);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 2);
}