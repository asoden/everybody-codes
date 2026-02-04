macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

test!(event2024
    quest01, quest02, quest03
);
