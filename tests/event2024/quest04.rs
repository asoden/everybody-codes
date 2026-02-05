use everybody_codes::event2024::quest04::*;

const EXAMPLE1: &str = "3\n4\n7\n8";
const EXAMPLE3: &str = "2\n4\n5\n6\n8";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 10);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 8);
}
