use anyhow::Result;
use util::{Solution, chore};

struct Puzzle {
    // Add fields here as needed
}

impl Puzzle {
    // Add any additional methods or helper functions here as needed
}

impl Solution for Puzzle {
    // You would rarely need to know if this is an example or not,
    // but if you do, you can use the const generic parameter `E`
    // to differentiate between the two.
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        // Replace the following line with actual parsing logic
        let _ = input.trim();
        Ok(Self {})
    }

    fn part1(&self) -> String {
        // Replace the following line with actual logic for part 1
        String::default()
    }

    fn part2(&self) -> String {
        // Replace the following line with actual logic for part 2
        String::default()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
