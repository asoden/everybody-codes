pub fn remove_non_digits_from_start(s: &str) -> &str {
    let first_digit_index = s
        .chars()
        .position(|c| c.is_ascii_digit())
        .unwrap_or(s.len());

    &s[first_digit_index..]
}
