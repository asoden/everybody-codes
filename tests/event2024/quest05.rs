use everybody_codes::event2024::quest05::*;

const EXAMPLE1: &str = "2 3 4 5\n3 4 5 2\n4 5 2 3\n5 2 3 4";
const EXAMPLE2: &str = "2 3 4 5\n6 7 8 9";
const EXAMPLE3: &str = "xBxAAABCDxCC";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 2323);
}

#[test] 
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 50877075);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 30);
}