pub fn part1(notes: &str) -> u64 {
    let blocks = notes.parse::<u64>().unwrap();
    let height = blocks.isqrt() + 1;
    let width = 2 * height - 1;
    let missing = height * height - blocks;
    width * missing
}

pub fn part2(notes: &str) -> u64 {
    const SUPPLY: u64 = 20240000;
    const ACOLYTES: u64 = 1111;

    let priests = notes.parse::<u64>().unwrap();

    let mut width = 1;
    let mut layer = 1;
    let mut blocks = 1;

    while blocks < SUPPLY {
        width += 2;
        layer = (layer * priests) % ACOLYTES;
        blocks += width * layer;
    }

    width * (blocks - SUPPLY)
}

pub fn part3(notes: &str) -> u64 {
    const SUPPLY: u64 = 202400000;
    const ACOLYTES: u64 = 10;

    let priests = notes.parse::<u64>().unwrap();

    let mut width = 1;
    let mut layer = 1;
    let mut blocks = 1;
    let mut height = 1;
    let mut layers = Vec::new();

    while blocks < SUPPLY {
        layers.push(layer);
        width += 2;
        layer = (layer * priests) % ACOLYTES + 10;
        blocks += width * layer;
        height += layer;
    }

    blocks += (priests * width * height) % ACOLYTES;

    for layer in layers {
        blocks -= 2 * ((priests * width * height) % ACOLYTES);
        height -= layer;
    }

    blocks - SUPPLY
}
