use crate::parser::{ParseError, Parser};

/// A [`Parser`] specialized for `char` inputs, with combinators for bridging into
/// string-slice parsing.
///
/// Any type that already implements `Parser<char>` can implement this trait to gain
/// [`single_char`](CharParser::single_char) and [`chars`](CharParser::chars).
pub trait CharParser: Parser<char> {
    /// Wraps this parser so it can parse a `&str` that must contain exactly one character.
    ///
    /// Returns [`ParseError::EmptyInput`] if the string is empty, or
    /// [`ParseError::WrongLength`] if it contains more than one character.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{CharParser, Parser};
    /// let p = parser::digit::<10>.single_char();
    /// assert_eq!(p.parse("7"), Ok(7));
    /// assert!(p.parse("42").is_err()); // more than one char
    /// ```
    fn single_char(self) -> SingleChar<Self>
    where
        Self: Sized,
    {
        SingleChar { parser: self }
    }

    /// Wraps this parser so it applies to every character in a `&str`, collecting
    /// results into a `Vec`.
    ///
    /// Fails fast on the first character that does not parse.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{CharParser, Parser};
    /// let p = parser::digit::<10>.chars();
    /// assert_eq!(p.parse("123"), Ok(vec![1, 2, 3]));
    /// assert!(p.parse("12x").is_err());
    /// ```
    fn chars(self) -> Chars<Self>
    where
        Self: Sized,
    {
        Chars { parser: self }
    }
}

/// Blanket [`CharParser`] implementation for functions `Fn(char) -> Result<T, ParseError>`.
///
/// Mirrors the blanket `Parser` impl, so any compatible function gains the
/// [`single_char`](CharParser::single_char) and [`chars`](CharParser::chars) combinators
/// without needing a wrapper type.
impl<T, F: Fn(char) -> Result<T, ParseError>> CharParser for F {}

/// A `&str` parser that delegates to an inner [`CharParser`], requiring the input to be
/// exactly one character long.
///
/// Constructed via [`CharParser::single_char`].
pub struct SingleChar<P> {
    parser: P,
}

impl<P> Parser<&str> for SingleChar<P>
where
    P: Parser<char>,
{
    type Output = P::Output;

    /// Parses `input` as a single character.
    ///
    /// Returns [`ParseError::EmptyInput`] if `input` is empty, or
    /// [`ParseError::WrongLength`] if `input` contains more than one character.
    /// Otherwise, delegates to the inner parser.
    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        let mut chars = input.chars();
        let c = chars.next().ok_or(ParseError::EmptyInput)?;
        if chars.next().is_some() {
            return Err(ParseError::WrongLength {
                expected: 1,
                got: input.chars().count(),
                input: input.to_string(),
            });
        }
        self.parser.parse(c)
    }
}

/// A `&str` parser that applies an inner [`CharParser`] to every character in the input,
/// collecting the results into a `Vec`.
///
/// Constructed via [`CharParser::chars`].
pub struct Chars<P> {
    parser: P,
}

impl<P: Parser<char>> Parser<&str> for Chars<P> {
    type Output = Vec<P::Output>;

    /// Applies the inner parser to each character of `input` in order.
    ///
    /// Returns the first error encountered, if any.
    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input.chars().map(|c| self.parser.parse(c)).collect()
    }
}

/// Constructs a `char -> Result` parser from a set of character-to-value mappings.
///
/// Each arm of the form `'x' => expr` is compiled into a `match` branch. Any character
/// not covered by the provided literals produces a [`ParseError::Other`] with a
/// descriptive message.
///
/// # Example
/// ```
/// # use aoc_lib::char_match;
/// # #[derive(Debug, PartialEq)]
/// # enum Dir { North, South, East, West }
/// let direction = char_match! {
///     'N' => Dir::North,
///     'S' => Dir::South,
///     'E' => Dir::East,
///     'W' => Dir::West,
/// };
/// assert_eq!(direction('N'), Ok(Dir::North));
/// assert!(direction('X').is_err());
/// ```
///
/// The resulting closure implements [`CharParser`], so it can be composed further:
/// ```
/// # use aoc_lib::char_match;
/// # use aoc_lib::utils::parser::{CharParser, Parser};
/// let p = char_match!('0' => false, '1' => true).chars();
/// assert_eq!(p.parse("0110"), Ok(vec![false, true, true, false]));
/// ```
#[macro_export]
macro_rules! char_match {
    ($($c:literal => $val:expr),+ $(,)?) => {
        |c: char| match c {
            $($c => Ok($val),)+
            _ => Err($crate::parser::ParseError::Other(
                format!("Unexpected character: '{c}'")
            ))
        }
    };
}
