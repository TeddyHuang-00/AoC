use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u8, u16},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};
use util::{Solution, chore, parser};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Clone, Copy)]
enum Activity {
    BeginShift(u16),
    FallAsleep,
    WakeUp,
}

struct Puzzle {
    sleep_records: BTreeMap<u16, [u8; 60]>,
}

impl Puzzle {
    fn parse_datetime(input: &str) -> IResult<&str, DateTime> {
        map(
            delimited(
                char('['),
                (
                    u16,
                    char('-'),
                    u8,
                    char('-'),
                    u8,
                    char(' '),
                    u8,
                    char(':'),
                    u8,
                ),
                char(']'),
            ),
            |(year, _, month, _, day, _, hour, _, minute)| DateTime {
                year,
                month,
                day,
                hour,
                minute,
            },
        )
        .parse_complete(input)
    }

    fn parse_log(input: &str) -> IResult<&str, Activity> {
        alt((
            map(
                delimited(tag("Guard #"), u16, tag(" begins shift")),
                Activity::BeginShift,
            ),
            value(Activity::FallAsleep, tag("falls asleep")),
            value(Activity::WakeUp, tag("wakes up")),
        ))
        .parse_complete(input)
    }

    fn parse_line(input: &str) -> IResult<&str, (DateTime, Activity)> {
        separated_pair(Self::parse_datetime, char(' '), Self::parse_log).parse_complete(input)
    }
}

impl Solution for Puzzle {
    fn parse<const E: bool>(input: &str) -> Result<Self> {
        let mut log =
            parser::parse_input_str(input, separated_list1(line_ending, Self::parse_line))?;
        log.sort_unstable_by_key(|&(dt, _)| dt);
        let mut sleep_records = BTreeMap::new();
        let mut ptr = 0;
        while ptr < log.len() {
            let (_, act) = log[ptr];
            let Activity::BeginShift(guard) = act else {
                panic!("Expect a guard to begin shift, found something else");
            };
            let entry = sleep_records.entry(guard).or_insert([0; 60]);
            ptr += 1;
            while ptr + 2 <= log.len()
                && let [(dt1, Activity::FallAsleep), (dt2, Activity::WakeUp)] = (log[ptr..ptr + 2])
            {
                for minute in dt1.minute..dt2.minute {
                    entry[usize::from(minute)] += 1;
                }
                ptr += 2;
            }
        }
        Ok(Self { sleep_records })
    }

    fn part1(&self) -> String {
        self.sleep_records
            .iter()
            .max_by_key(|&(_, record)| record.iter().copied().map(u16::from).sum::<u16>())
            .map(|(guard, record)| {
                (
                    *guard,
                    record
                        .iter()
                        .enumerate()
                        .max_by_key(|&(_, cnt)| cnt)
                        .unwrap_or_else(|| unreachable!())
                        .0,
                )
            })
            .map_or_else(
                || panic!("Shoud be at least one guard in log"),
                |(guard, minute)| usize::from(guard) * minute,
            )
            .to_string()
    }

    fn part2(&self) -> String {
        self.sleep_records
            .iter()
            .map(|(guard, record)| {
                let (minute, count) = record
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, cnt)| cnt)
                    .unwrap_or_else(|| unreachable!());
                (*guard, minute, *count)
            })
            .max_by_key(|&(_, _, cnt)| cnt)
            .map_or_else(
                || panic!("Shoud be at least one guard in log"),
                |(guard, minute, _)| usize::from(guard) * minute,
            )
            .to_string()
    }
}

// WARNING: DO NOT CHANGE THE LINE BELOW.
chore!(env!("CARGO_PKG_NAME"));
