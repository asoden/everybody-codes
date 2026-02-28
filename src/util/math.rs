// Euclid was a genius so let's use that
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

// get least common multiple by relationship of greatest common denominator
pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
