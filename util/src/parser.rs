//! Common parsing and converting utilities

use std::fmt::{Debug, Display};

use anyhow::Result;
use ndarray::Array2;
use nom::{
    Parser,
    character::complete::line_ending,
    combinator::{all_consuming, opt},
    error::ParseError,
    sequence::terminated,
};

/// Convert a nested Vec (`Vec<Vec<T>>`) into a 2D ndarray `Array2<T>`
///
/// # Errors
/// This function will return an error if the nested Vec does not have a
/// consistent number of columns in each row.
pub fn nested_vec_to_array2<T>(grid: Vec<Vec<T>>) -> Result<Array2<T>> {
    let row_count = grid.len();
    let col_count = grid.first().map_or(0, Vec::len);
    let flat_data = grid.into_iter().flatten().collect::<Vec<T>>();
    Ok(Array2::from_shape_vec((row_count, col_count), flat_data)?)
}

/// Parse the whole input string using the provided parser and return the
/// result.
///
/// # Errors
/// This function will return an error if:
/// - The parser fails to parse during the process.
/// - The parser does not consume the entire input string.
pub fn parse_input_str<'a, O, E, F>(input: &'a str, parser: F) -> Result<O>
where
    E: ParseError<&'a str> + Debug + Display,
    F: Parser<&'a str, Output = O, Error = E>,
{
    all_consuming(terminated(parser, opt(line_ending)))
        .parse_complete(input)
        .map(|(_, result)| result)
        .map_err(|err| anyhow::anyhow!("Failed to parse input: {err}"))
}

#[cfg(test)]
mod tests {
    use ndarray::prelude::*;

    use super::*;

    #[test]
    fn test_nested_vec_to_array2() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let array = nested_vec_to_array2(vec)
            .unwrap_or_else(|e| panic!("Failed to convert nested vec to array2: {e}"));
        assert_eq!(array.shape(), &[2, 3]);
        assert_eq!(array, array![[1, 2, 3], [4, 5, 6]]);

        let vec_inconsistent = vec![vec![1, 2], vec![3, 4, 5]];
        let result = nested_vec_to_array2(vec_inconsistent);
        assert!(result.is_err());
    }
}
