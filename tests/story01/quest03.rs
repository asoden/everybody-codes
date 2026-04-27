use everybody_codes::story01::quest03::*;

const EXAMPLE1: &str = "\
x=1 y=2
x=2 y=3
x=3 y=4
x=4 y=4";
const EXAMPLE2: &str = "\
x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3";
const EXAMPLE3: &str = "\
ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 1310);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 14);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 0);
}