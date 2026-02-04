macro_rules! library {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

library!(event2024
    quest01, quest02
);

library!(util
    ansi, tools
);
