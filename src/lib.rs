//! This library provides a configurable FizzBuzz implementation.
//!
//! # Examples
//! ## FizzBuzz from 1 to 15
//! ```
//! use extended_fizzbuzz::{fizzbuzz, Matcher};
//!
//! let matchers = vec![
//!     Matcher::new(3, "Fizz").unwrap(),
//!     Matcher::new(5, "Buzz").unwrap(),
//! ];
//!
//! fizzbuzz(1, 15, &matchers).unwrap();
//! ```
//!
//! Output:
//! ```txt
//! 1
//! 2
//! Fizz
//! 4
//! Buzz
//! Fizz
//! 7
//! 8
//! Fizz
//! Buzz
//! 11
//! Fizz
//! 13
//! 14
//! FizzBuzz
//! ```
//!
//! ## FizzBuzz for single numbers
//! ```
//! use extended_fizzbuzz::{line, Matcher};
//!
//! let matchers = vec![
//!     Matcher::new(3, "Fizz").unwrap(),
//!     Matcher::new(5, "Buzz").unwrap(),
//! ];
//!
//! assert_eq!(line(6, &matchers), "Fizz".to_string());
//! assert_eq!(line(7, &matchers), "7".to_string());
//! assert_eq!(line(10, &matchers), "Buzz".to_string());
//! assert_eq!(line(15, &matchers), "FizzBuzz".to_string());
//! ```

mod matcher;

pub use matcher::*;
use thiserror::Error;

/// Provides a configurable version of FizzBuzz.
///
/// # Parameters
/// With `from` and `to` you can define the number area for which to run the operation. Both of
/// these values are inclusive. This means that if you specify 1 and 10, the numbers for which
/// output is produced are: 1,2,3,4,5,6,7,8,9,10
///
/// With `matchers` you can provide some `matcher::Matcher`s. These are used to configure how
/// numbers are substituted with words. The matchers are tested in the order of the vector.
///
/// # Errors
/// - Returns `FizzBuzzError::FromBiggerThanTo`, if the `from` parameters value is bigger than the
///   `to` parameters value.
///
/// # Example
/// ```
/// use extended_fizzbuzz::{fizzbuzz, Matcher};
///
/// let matchers = vec![
///     Matcher::new(3, "Fizz").unwrap(),
///     Matcher::new(5, "Buzz").unwrap(),
/// ];
///
/// assert!(fizzbuzz(1, 10, &matchers).is_ok());
/// assert!(fizzbuzz(10, 1, &matchers).is_err());
/// ```
pub fn fizzbuzz(from: usize, to: usize, matchers: &Vec<Matcher>) -> Result<(), FizzBuzzError> {
    if from > to {
        return Err(FizzBuzzError::FromBiggerThanTo { from, to });
    }

    for i in from..(to + 1) {
        println!("{}", line(i, matchers));
    }

    Ok(())
}

/// Provides a configurable version of FizzBuzz for a single number.
///
/// # Parameters
/// With `number`, you can provide the number to calculate the result for.
///
/// With `matchers` you can provide some `matcher::Matcher`s. These are used to configure how
/// numbers are substituted with words. The matchers are tested in the order of the vector.
///
/// # Example
/// ```
/// use extended_fizzbuzz::{line, Matcher};
///
/// let matchers = vec![
///     Matcher::new(3, "Fizz").unwrap(),
///     Matcher::new(5, "Buzz").unwrap(),
/// ];
///
/// assert_eq!(line(1, &matchers), "1".to_string());
/// assert_eq!(line(3, &matchers), "Fizz".to_string());
/// assert_eq!(line(5, &matchers), "Buzz".to_string());
/// assert_eq!(line(15, &matchers), "FizzBuzz".to_string());
/// assert_eq!(line(16, &matchers), "16".to_string());
/// ```
pub fn line(number: usize, matchers: &Vec<Matcher>) -> String {
    let mut out = String::new();

    for m in matchers.iter() {
        out += m.text(number);
    }

    if out.is_empty() {
        out += &number.to_string();
    }

    out
}

/// All errors the `fizzbuzz()` function can produce.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FizzBuzzError {
    /// The `from` parameter has a higher value than the `to` parameter. No valid range can be
    /// constructed.
    #[error("`from` value ({from}) is bigger than `to` value({to})")]
    FromBiggerThanTo { from: usize, to: usize },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[test]
    fn line_normal() {
        let text1 = "Fizz";
        let text2 = "Buzz";

        // u8 is used for rng, to ensure that the result of number1 * number2 is within the
        // boundaries of usize and keep relatively short rng times.
        let mut number1: u8 = 0;
        let mut number2 = 0;

        while number1 == 0 || number1 == 1 || number1 >= (u8::MAX / 2) {
            number1 = random();
        }
        while number2 < number1 || number2 % number1 == 0 {
            number2 = random();
        }

        let number1 = number1.into();
        let number2 = number2.into();

        let matchers = vec![
            Matcher::new(number1, text1).unwrap(),
            Matcher::new(number2, text2).unwrap(),
        ];

        assert_eq!(line(1, &matchers), "1");
        assert_eq!(line(number1, &matchers), text1.to_string());
        assert_eq!(line(number2, &matchers), text2.to_string());
        assert_eq!(
            line(number1 * number2, &matchers),
            format!("{}{}", text1, text2)
        );
        assert_eq!(
            line((number1 * number2) + 1, &matchers),
            ((number1 * number2) + 1).to_string()
        );
    }

    #[test]
    fn line_one() {
        let text1 = "Fizz";
        let text2 = "Buzz";

        let number1 = 1;
        let mut number2: u8 = 0;
        while number2 < number1 {
            number2 = random();
        }

        let number1 = number1.into();
        let number2 = number2.into();

        let matchers = vec![
            Matcher::new(number1, text1).unwrap(),
            Matcher::new(number2, text2).unwrap(),
        ];

        assert_eq!(line(1, &matchers), text1.to_string());
        assert_eq!(line(number2, &matchers), format!("{}{}", text1, text2));
        assert_eq!(line((number1 * number2) + 1, &matchers), text1.to_string());
    }
}
