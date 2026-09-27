#[macro_export]
macro_rules! chore {
    ($day:expr) => {
        const YEAR_N_DAY: &str = $day;
        const PUZZLE_INPUT: &str = include_str!("../puzzle.in");
        const EXAMPLE_INPUT: &str = include_str!("../example.in");
        const EXAMPLE_OUTPUT: &str = include_str!("../example.out");

        impl util::InputData for Puzzle {
            const INPUT: &str = PUZZLE_INPUT;
            const EXAMPLE: &str = EXAMPLE_INPUT;
        }

        impl Puzzle {
            fn new() -> Result<Self> {
                use util::InputData;
                Self::parse::<false>(Puzzle::INPUT)
            }

            fn example() -> Result<Self> {
                use util::InputData;
                Self::parse::<true>(Puzzle::EXAMPLE)
            }
        }

        fn main() -> anyhow::Result<()> {
            use util::InputData;

            let puzzle = Puzzle::new()?;
            let year = &YEAR_N_DAY[1..5];
            let day = &YEAR_N_DAY[6..];
            println!("Year {year} Day {day} Part 1: {}", puzzle.part1());
            println!("Year {year} Day {day} Part 2: {}", puzzle.part2());

            Ok(())
        }

        #[cfg(test)]
        mod tests {
            use std::time::Duration;

            use util::{Benchmark, Serializable};

            use super::*;

            fn split_answers(output: &str) -> (&str, &str) {
                let mut lines = output.lines();
                let p1 = lines.next().unwrap_or("").trim();
                let p2 = lines.next().unwrap_or("").trim();
                (p1, p2)
            }

            #[test]
            fn test_part1() -> anyhow::Result<()> {
                let puzzle = Puzzle::example()?;
                assert_eq!(puzzle.part1(), split_answers(EXAMPLE_OUTPUT).0);
                Ok(())
            }

            #[test]
            fn test_part2() -> anyhow::Result<()> {
                let puzzle = Puzzle::example()?;
                assert_eq!(puzzle.part2(), split_answers(EXAMPLE_OUTPUT).1);
                Ok(())
            }

            #[test]
            fn benchmark() -> Result<()> {
                Puzzle::bench_all(Duration::from_secs(1)).to_csv()
            }
        }
    };
}
