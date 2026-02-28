macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

test!(event2024
    quest01, quest02, quest03, quest04, quest05, quest06, quest07, quest08, quest09, quest10,
    quest11, quest12, quest13, quest14, quest15, quest16
);
