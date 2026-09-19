use std::{collections::HashMap, str::FromStr};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, i16},
    combinator::{map, map_res, value},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
};

#[derive(Clone, Eq, PartialEq)]
pub enum Json {
    Null,
    // In an actual parser you might want f64,
    // but in this case we only need i16
    Num(i16),
    Bool(bool),
    Str(String),
    Arr(Vec<Self>),
    Obj(HashMap<String, Self>),
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(false, tag("false")), value(true, tag("true")))).parse_complete(input)
}

fn string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        // Assume all strings are simple, no escape, no drama!
        map_res(alphanumeric1, String::from_str),
        char('"'),
    )
    .parse_complete(input)
}

fn array(input: &str) -> IResult<&str, Vec<Json>> {
    delimited(
        char('['),
        separated_list0(char(','), json_parser),
        char(']'),
    )
    .parse_complete(input)
}

fn object(input: &str) -> IResult<&str, HashMap<String, Json>> {
    map(
        delimited(
            char('{'),
            separated_list0(char(','), separated_pair(string, char(':'), json_parser)),
            char('}'),
        ),
        |kv_pairs| kv_pairs.into_iter().collect(),
    )
    .parse_complete(input)
}

pub fn json_parser(input: &str) -> IResult<&str, Json> {
    alt((
        value(Json::Null, tag("null")),
        map(i16, Json::Num),
        map(boolean, Json::Bool),
        map(string, Json::Str),
        map(array, Json::Arr),
        map(object, Json::Obj),
    ))
    .parse_complete(input)
}
