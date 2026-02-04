use everybody_codes::event2024::quest02::*;

const EXAMPLE1: &str = "WORDS:THE,OWE,MES,ROD,HER\n\nAWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
const EXAMPLE2: &str = "WORDS:THE,OWE,MES,ROD,HER,QAQ\n\nAWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE\nTHE FLAME SHIELDED THE HEART OF THE KINGS\nPOWE PO WER P OWE R\nTHERE IS THE END\nQAQAQ";
const EXAMPLE3: &str = "WORDS:THE,OWE,MES,ROD,RODEO\n\nHELWORLT\nENIGWDXL\nTRODEOAL";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), 4);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 42);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 10);
}