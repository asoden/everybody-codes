use everybody_codes::util::ansi::*;
use std::{fs::read_to_string, time::Instant};

fn main() {
    let solutions = [event2024()];

    // Filter solutions then pretty print output.
    solutions.into_iter().flatten().for_each(
        |Solution {
             event,
             quest,
             part1,
             part2,
             part3,
         }| {
            println!("{YELLOW}Event {event} Quest {quest}{RESET}");
            solve(event, quest, 1, part1);
            solve(event, quest, 2, part2);
            solve(event, quest, 3, part3);
        },
    );
}

fn solve(event: u32, quest: u32, part: u32, wrapper: fn(&str) -> String) {
    let path = format!("input/{event:02}/everybody_codes_e{event}_q{quest:02}_p{part}.txt");

    if let Ok(notes) = read_to_string(&path) {
        let now = Instant::now();
        println!(
            "    Part {part}: {BOLD}{WHITE}{} {:.2?}{RESET}",
            wrapper(&notes),
            now.elapsed()
        );
    } else {
        eprintln!("    Part {part}: {BOLD}{WHITE}{path}{RESET} missing");
    }
}

struct Solution {
    event: u32,
    quest: u32,
    part1: fn(&str) -> String,
    part2: fn(&str) -> String,
    part3: fn(&str) -> String,
}

macro_rules! run {
    ($event:tt $($quest:tt),*) => {
        fn $event() -> Vec<Solution> {
            vec![$({
                use everybody_codes::$event::$quest::*;
                Solution {
                    event: everybody_codes::util::tools::remove_non_digits_from_start(stringify!($event)).parse::<u32>().unwrap(),
                    quest: everybody_codes::util::tools::remove_non_digits_from_start(stringify!($quest)).parse::<u32>().unwrap(),
                    part1: |notes: &str| part1(notes).to_string(),
                    part2: |notes: &str| part2(notes).to_string(),
                    part3: |notes: &str| part3(notes).to_string(),
                }
            },)*]
        }
    }
}

run!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12, quest13, quest14, quest15
);
