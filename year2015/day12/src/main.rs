mod json;

use anyhow::Result;
use json::{Json, json_parser};
use util::{Solution, chore, parser};

struct Puzzle {
    json: Json,
}

impl Puzzle {
    fn sum_all_numbers(&self, ignore_red: bool) -> i32 {
        let mut frontiers = vec![&self.json];
        let mut sum = 0;
        while let Some(json) = frontiers.pop() {
            match json {
                &Json::Num(val) => sum += i32::from(val),
                Json::Arr(arr) => frontiers.extend_from_slice(&arr[..].iter().collect::<Vec<_>>()),
                Json::Obj(obj)
                    if (!ignore_red
                        || !obj
                            .values()
                            .any(|v| matches!(v, Json::Str(x) if x.as_str() == "red"))) =>
                {
                    frontiers.extend_from_slice(&obj.values().collect::<Vec<_>>());
                }
                _ => {}
            }
        }
        sum
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let json = parser::parse_input_str(input, json_parser)?;
        Ok(Self { json })
    }

    fn part1(&self) -> String {
        self.sum_all_numbers(false).to_string()
    }

    fn part2(&self) -> String {
        self.sum_all_numbers(true).to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
