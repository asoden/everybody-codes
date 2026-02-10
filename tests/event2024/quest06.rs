use everybody_codes::event2024::quest06::*;

const EXAMPLE1: &str = "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@";
const EXAMPLE2: &str = "RR:ABAB
ABAB:CDCD
CDCD:EFEF
EFEF:ROLO
ROLO:@";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), "RRB@");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), "RACER@");
}