use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    // TEMPLATE: Add fields here as needed
}

impl Puzzle {
    // TEMPLATE: Add any additional methods or helper functions here as needed
}

impl Solution for Puzzle {
    // TEMPLATE: You would rarely need to know if this is an example or not,
    // TEMPLATE: but if you do, you can use the const generic parameter `E`
    // TEMPLATE: to differentiate between the two.
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        // TEMPLATE: Replace the following line with actual parsing logic
        let _ = input.trim();
        Ok(Self {})
    }

    fn part1(&self) -> String {
        // TEMPLATE: Replace the following line with actual logic for part 1
        String::default()
    }

    fn part2(&self) -> String {
        // TEMPLATE: Replace the following line with actual logic for part 2
        String::default()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
