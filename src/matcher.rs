use thiserror::Error;

/// A container for configuration values.
///
/// A vector of `Matcher`s are used to tell the `fizzbuzz()` function how to substitute words
///
/// # Example
/// ```
/// # use extended_fizzbuzz::Matcher;
/// let fizz = Matcher::new(3, "Fizz").unwrap();
///
/// let buzz_message = String::from("Buzz");
/// let buzz = Matcher::new(5, &buzz_message).unwrap();
/// ```
#[derive(Debug)]
pub struct Matcher {
    number: usize,
    word: String,
}

impl Matcher {
    /// Create a new matcher.
    ///
    /// # Parameters
    /// The `number` parameter is used to check wether a number should be substituted by the
    /// configured text. For this, the checked number is divided by the `number` parameters value,
    /// which therefore mustn't be 0.
    ///
    /// # Errors
    /// - Returns `MatcherError::NumberIsZero` if the `number` parameter is 0.
    pub fn new(number: usize, word: &str) -> Result<Self, MatcherError> {
        if number == 0 {
            return Err(MatcherError::NumberIsZero);
        }

        Ok(Matcher {
            number: number,
            word: word.to_owned(),
        })
    }

    /// Check wether the `number` should be substituted.
    ///
    /// # Example
    /// ```
    /// # use extended_fizzbuzz::Matcher;
    /// let number = 3;
    /// let matcher = Matcher::new(number, "Fizz").unwrap();
    ///
    /// assert_eq!(matcher.matches(number - 1), false);
    /// assert_eq!(matcher.matches(number), true);
    /// assert_eq!(matcher.matches(number + 1), false);
    /// ```
    pub fn matches(self: &Self, number: usize) -> bool {
        number % self.number == 0
    }

    /// Get the text the `number` should be substituted by.
    ///
    /// If the `number` should be substituted, returns the appropriate text. Otherwise returns an
    /// empty string.
    ///
    /// # Example
    /// ```
    /// # use extended_fizzbuzz::Matcher;
    /// let text = "Fizz";
    /// let number = 3;
    /// let matcher = Matcher::new(number, text).unwrap();
    ///
    /// assert_eq!(matcher.text(number - 1), "");
    /// assert_eq!(matcher.text(number), text);
    /// assert_eq!(matcher.text(number + 1), "");
    /// ```
    pub fn text(self: &Self, number: usize) -> &str {
        if self.matches(number) {
            return &self.word;
        }

        ""
    }
}

/// All errors a `Matcher` can produce.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MatcherError {
    /// The `number` parameter of a `Matcher` was set to 0. This would lead to divisions by zero,
    /// which is mathematically impossible.
    #[error("`number` is 0, but division by 0 is impossible")]
    NumberIsZero,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[test]
    fn new_normal() {
        let word = "Test";
        let mut number = 0;
        while number == 0 {
            number = random();
        }

        let matcher = Matcher::new(number, word).unwrap();
        assert_eq!(matcher.number, number);
        assert_eq!(matcher.word, word.to_string());
    }

    #[test]
    fn new_zero() -> Result<(), String> {
        let word = "Test";
        let number = 0;

        let matcher_res = Matcher::new(number, word);
        match matcher_res {
            Err(e) => match e {
                MatcherError::NumberIsZero => Ok(()),
                _ => Err("Wrong error kind".to_string()),
            },
            _ => Err("Wrongfully succeeded".to_string()),
        }
    }

    #[test]
    fn matches_normal() {
        let word = "Test";
        let mut number = 0;
        while number == 0 || number == 1 {
            number = random();
        }

        let matcher = Matcher::new(number, word).unwrap();

        assert!(!matcher.matches(number - 1));
        assert!(matcher.matches(number));
        assert!(!matcher.matches(number + 1));
    }

    #[test]
    fn matches_one() {
        let word = "Test";
        let number = 1;

        let matcher = Matcher::new(number, word).unwrap();

        assert!(matcher.matches(number - 1));
        assert!(matcher.matches(number));
        assert!(matcher.matches(number + 1));
    }

    #[test]
    fn text_normal() {
        let word = "Test";
        let mut number = 0;
        while number == 0 || number == 1 {
            number = random();
        }

        let matcher = Matcher::new(number, word).unwrap();

        assert_eq!(matcher.text(number - 1), "");
        assert_eq!(matcher.text(number), word);
        assert_eq!(matcher.text(number + 1), "");
    }

    #[test]
    fn text_one() {
        let word = "Test";
        let number = 1;

        let matcher = Matcher::new(number, word).unwrap();

        assert_eq!(matcher.text(number - 1), word);
        assert_eq!(matcher.text(number), word);
        assert_eq!(matcher.text(number + 1), word);
    }
}
