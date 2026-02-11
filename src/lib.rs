macro_rules! library {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

library!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08
);

library!(util
    ansi, tools
);
