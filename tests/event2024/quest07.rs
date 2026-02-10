use everybody_codes::event2024::quest07::*;

const EXAMPLE: &str = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), "BDCA");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE), "DCBA");
}