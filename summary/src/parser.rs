use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, line_ending, usize},
    combinator::{map, value},
    multi::many_m_n,
    number::complete::double,
    sequence::{preceded, separated_pair},
};
use util::float_as_key;

#[derive(Clone, Copy, Debug)]
pub enum Scale {
    NanoSecond = 0,
    MicroSecond = 3,
    MilliSecond = 6,
    Second = 9,
    Minute = 10,
}

impl Scale {
    pub fn to_decimal(self) -> f64 {
        match self {
            Self::Minute => Self::Second.to_decimal() * 60.0,
            _ => 10f64.powi(self as i32),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Timing {
    pub median: f64,
    pub mad: f64,
    pub scale: Scale,
}

float_as_key!(Timing, median);

fn parse_time(input: &str) -> IResult<&str, (f64, Scale)> {
    (
        double,
        alt((
            value(Scale::NanoSecond, tag("ns")),
            value(Scale::MicroSecond, tag("µs")),
            value(Scale::MilliSecond, tag("ms")),
            value(Scale::Second, tag("s")),
            value(Scale::Minute, tag("m")),
        )),
    )
        .parse_complete(input)
}

fn parse_line(input: &str) -> IResult<&str, Timing> {
    preceded(
        (
            take_till(|ch| ch == ','),
            char(','),
            usize,
            char(','),
            many_m_n(5, 5, (parse_time, char(','))),
        ),
        map(
            separated_pair(parse_time, char(','), parse_time),
            |((median, scale), (mad, _))| Timing { median, mad, scale },
        ),
    )
    .parse_complete(input)
}

pub fn parse_csv(input: &str) -> IResult<&str, Vec<Timing>> {
    preceded(
        (take_till(|ch| ch == '\n'), line_ending),
        many_m_n(3, 3, parse_line),
    )
    .parse_complete(input)
}
