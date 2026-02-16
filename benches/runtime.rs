fn main() {
    divan::main();
}

macro_rules! benchmark {
    ($event:tt $($quest:tt),*) => {
        #[divan::bench_group(
            name = stringify!($event)
        )]
        mod $event {
            $(
            #[divan::bench_group()]
            mod $quest {
                use std::fs::read_to_string;
                use everybody_codes::util::ansi::*;
                use everybody_codes::$event::$quest;
                use std::sync::LazyLock;

                static DATA1: LazyLock<String> = LazyLock::new(|| {
                    let event = everybody_codes::util::tools::remove_non_digits_from_start(stringify!($event)).parse::<u32>().unwrap();
                    let quest = everybody_codes::util::tools::remove_non_digits_from_start(stringify!($quest)).parse::<u32>().unwrap();
                    let path = format!("input/{event:02}/everybody_codes_e{event}_q{quest:02}_p1.txt");
                    read_to_string(&path).unwrap_or_else(|_| {
                        panic!("Missing input file {BOLD}{WHITE}{path}{RESET}");
                    })
                });

                static DATA2: LazyLock<String> = LazyLock::new(||{
                    let event = everybody_codes::util::tools::remove_non_digits_from_start(stringify!($event)).parse::<u32>().unwrap();
                    let quest = everybody_codes::util::tools::remove_non_digits_from_start(stringify!($quest)).parse::<u32>().unwrap();
                    let path = format!("input/{event:02}/everybody_codes_e{event}_q{quest:02}_p2.txt");

                    read_to_string(&path).unwrap_or_else(|_| {
                        panic!("Missing input file {BOLD}{WHITE}{path}{RESET}");
                    })
                });

                static DATA3: LazyLock<String> = LazyLock::new(|| {

                    let event = everybody_codes::util::tools::remove_non_digits_from_start(stringify!($event)).parse::<u32>().unwrap();
                    let quest = everybody_codes::util::tools::remove_non_digits_from_start(stringify!($quest)).parse::<u32>().unwrap();
                    let path = format!("input/{event:02}/everybody_codes_e{event}_q{quest:02}_p3.txt");

                    read_to_string(&path).unwrap_or_else(|_| {
                        panic!("Missing input file {BOLD}{WHITE}{path}{RESET}");
                    })
                });

                #[divan::bench()]
                fn part1() {
                    $quest::part1(divan::black_box(&DATA1));
                }

                #[divan::bench()]
                fn part2() {
                    $quest::part2(divan::black_box(&DATA2));
                }

                #[divan::bench()]
                fn part3() {
                    $quest::part3(divan::black_box(&DATA3));
                }
            })*
        }
    };
}

benchmark!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10
);