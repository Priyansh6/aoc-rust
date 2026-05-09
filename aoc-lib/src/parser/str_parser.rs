use crate::parser::{CharParser, ParseError, Parser};

/// A [`Parser`] specialized for `&str` inputs, with combinators for splitting and
/// iterating over string data.
///
/// Any type implementing `Parser<&str>` automatically gains these combinators via the
/// blanket impl below.
pub trait StrParser: for<'a> Parser<&'a str> {
    /// Applies this parser to each line of the input, collecting results into a `Vec`.
    ///
    /// Lines are split by `\n` or `\r\n` (via [`str::lines`]). Fails fast on the first
    /// line that does not parse.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{Parser, StrParser};
    /// let p = parser::from_str::<u32>.lines();
    /// assert_eq!(p.parse("1\n2\n3"), Ok(vec![1, 2, 3]));
    /// ```
    fn lines(self) -> Lines<Self>
    where
        Self: Sized,
    {
        Lines { parser: self }
    }

    /// Splits the input on `separator` and applies this parser to each part, collecting
    /// results into a `Vec`.
    ///
    /// Fails fast on the first part that does not parse.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{Parser, StrParser};
    /// let p = parser::from_str::<u32>.split(",");
    /// assert_eq!(p.parse("1,2,3"), Ok(vec![1, 2, 3]));
    /// ```
    fn split(self, separator: &str) -> Split<Self>
    where
        Self: Sized,
    {
        Split {
            parser: self,
            separator: separator.to_string(),
        }
    }

    /// Splits the input on any whitespace and applies this parser to each token,
    /// collecting results into a `Vec`.
    ///
    /// Uses [`str::split_whitespace`], so leading, trailing, and consecutive whitespace
    /// are all handled gracefully. Fails fast on the first token that does not parse.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{Parser, StrParser};
    /// let p = parser::from_str::<u32>.split_whitespace();
    /// assert_eq!(p.parse("1  2\t3"), Ok(vec![1, 2, 3]));
    /// ```
    fn split_whitespace(self) -> SplitWhitespace<Self>
    where
        Self: Sized,
    {
        SplitWhitespace { parser: self }
    }

    /// Splits the input on `separator` and applies this parser to each part, collecting
    /// results into a fixed-size array of length `N`.
    ///
    /// Returns [`ParseError::WrongLength`] if the number of parts is anything other
    /// than `N`. Fails fast on the first part that does not parse.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{Parser, StrParser};
    /// let p = parser::from_str::<u32>.split_array::<3>(",");
    /// assert_eq!(p.parse("1,2,3"), Ok([1, 2, 3]));
    /// assert!(p.parse("1,2").is_err());     // too few
    /// assert!(p.parse("1,2,3,4").is_err()); // too many
    /// ```
    fn split_array<const N: usize>(self, separator: &str) -> SplitArray<Self, N>
    where
        Self: Sized,
    {
        SplitArray {
            parser: self,
            separator: separator.to_string(),
        }
    }

    /// Strips a matching open/close delimiter pair from the start and end of the input,
    /// then applies this parser to the inner content.
    ///
    /// Returns [`ParseError::Other`] if the input does not start and end with the
    /// expected delimiters.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{Parser, StrParser};
    /// let p = parser::from_str::<u32>.wrapped("[", "]");
    /// assert_eq!(p.parse("[42]"), Ok(42));
    /// assert!(p.parse("42").is_err());
    /// ```
    fn wrapped(self, open: &str, close: &str) -> Wrapped<Self>
    where
        Self: Sized,
    {
        Wrapped {
            parser: self,
            open: open.to_string(),
            close: close.to_string(),
        }
    }
}

/// Blanket [`StrParser`] implementation for any type that implements `Parser<&str>`.
///
/// This means all combinators on [`StrParser`] are available automatically, including
/// on closures and the combinator structs from the parent module.
impl<P> StrParser for P where P: for<'a> Parser<&'a str> {}

/// A `&str` parser that applies an inner [`StrParser`] to each line of the input.
///
/// Constructed via [`StrParser::lines`].
pub struct Lines<P> {
    parser: P,
}

impl<P, T> Parser<&str> for Lines<P>
where
    P: StrParser<Output = T>,
{
    type Output = Vec<T>;

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input.lines().map(|l| self.parser.parse(l)).collect()
    }
}

/// A `&str` parser that splits the input on a fixed separator and applies an inner
/// [`StrParser`] to each part.
///
/// Constructed via [`StrParser::split`].
pub struct Split<P> {
    parser: P,
    separator: String,
}

impl<P, T> Parser<&str> for Split<P>
where
    P: StrParser<Output = T>,
{
    type Output = Vec<T>;

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input
            .split(&self.separator)
            .map(|v| self.parser.parse(v))
            .collect()
    }
}

/// A `&str` parser that splits the input on whitespace and applies an inner
/// [`StrParser`] to each token.
///
/// Constructed via [`StrParser::split_whitespace`].
pub struct SplitWhitespace<P> {
    parser: P,
}

impl<P, T> Parser<&str> for SplitWhitespace<P>
where
    P: StrParser<Output = T>,
{
    type Output = Vec<T>;

    fn parse(&self, input: &str) -> Result<Vec<T>, ParseError> {
        input
            .split_whitespace()
            .map(|v| self.parser.parse(v))
            .collect()
    }
}

/// A `&str` parser that splits the input on a fixed separator and applies an inner
/// [`StrParser`] to each part, collecting results into a fixed-size array of length `N`.
///
/// Constructed via [`StrParser::split_array`].
pub struct SplitArray<P, const N: usize> {
    parser: P,
    separator: String,
}

impl<T, P, const N: usize> Parser<&str> for SplitArray<P, N>
where
    P: StrParser<Output = T>,
{
    type Output = [T; N];

    /// Splits `input` on the stored separator, parses each part, and converts the
    /// resulting `Vec` into a `[T; N]`.
    ///
    /// Returns [`ParseError::WrongLength`] if the split does not yield exactly `N` parts.
    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input
            .split(&self.separator)
            .map(|v| self.parser.parse(v))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v: Vec<T>| ParseError::WrongLength {
                expected: N,
                got: v.len(),
                input: input.to_string(),
            })
    }
}

/// A `&str` parser that strips a matching open/close delimiter pair from the input
/// before applying an inner [`StrParser`] to the remaining content.
///
/// Constructed via [`StrParser::wrapped`].
pub struct Wrapped<P> {
    parser: P,
    open: String,
    close: String,
}

impl<P, T> Parser<&str> for Wrapped<P>
where
    P: StrParser<Output = T>,
{
    type Output = T;

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        let inner = input
            .strip_prefix(&self.open)
            .and_then(|s| s.strip_suffix(&self.close))
            .ok_or_else(|| ParseError::NotWrapped {
                open: self.open.clone(),
                close: self.close.clone(),
            })?;
        self.parser.parse(inner)
    }
}

/// Splits a string on `separator` and parses the left and right halves independently.
///
/// Expects exactly one occurrence of `separator`, producing a [`ParseError::WrongLength`]
/// if the split yields any number of parts other than two.
///
/// # Example
/// ```
/// # use aoc_lib::utils::parser;
/// # use aoc_lib::utils::parser::Parser;
/// let p = parser::split_pair(parser::from_str::<u32>, parser::from_str::<u32>, "-");
/// assert_eq!(p.parse("10-20"), Ok((10, 20)));
/// ```
pub fn split_pair<T, U>(
    left: impl StrParser<Output = T>,
    right: impl StrParser<Output = U>,
    separator: &str,
) -> impl StrParser<Output = (T, U)> {
    let separator = separator.to_string();
    move |input: &str| {
        let elems: Vec<&str> = input.split(&*separator).collect();
        match elems.as_slice() {
            [l, r] => Ok((left.parse(l)?, right.parse(r)?)),
            _ => Err(ParseError::WrongLength {
                expected: 2,
                got: elems.len(),
                input: input.to_string(),
            }),
        }
    }
}

/// Splits off the first character of the input and parses it and the remainder separately.
///
/// The first character is passed to the `first` [`CharParser`]; the rest of the string
/// slice is passed to `rest`. Returns [`ParseError::EmptyInput`] if the input is empty.
///
/// # Example
/// ```
/// # use aoc_lib::utils::parser;
/// # use aoc_lib::utils::parser::Parser;
/// let p = parser::uncons(parser::digit::<10>, parser::from_str::<u32>);
/// assert_eq!(p.parse("142"), Ok((1, 42)));
/// ```
pub fn uncons<T, U>(
    first: impl CharParser<Output = T>,
    rest: impl StrParser<Output = U>,
) -> impl StrParser<Output = (T, U)> {
    move |input: &str| {
        let mut chars = input.chars();
        let c = chars.next().ok_or(ParseError::EmptyInput)?;
        let a = first.parse(c)?;
        let b = rest.parse(chars.as_str())?;
        Ok((a, b))
    }
}

/// Splits a string on the *first* occurrence of `separator`, parsing each part separately.
///
/// The leading segment is passed to `first`; everything after the separator is passed
/// to `rest`. Returns [`ParseError::EmptyInput`] if `separator` is not found.
///
/// # Example
/// ```
/// # use aoc_lib::utils::parser;
/// # use aoc_lib::utils::parser::Parser;
/// let p = parser::lsplit_once(parser::as_string, parser::as_string, "/");
/// assert_eq!(p.parse("a/b/c"), Ok(("a".to_string(), "b/c".to_string())));
/// ```
pub fn lsplit_once<T, U>(
    first: impl StrParser<Output = T>,
    rest: impl StrParser<Output = U>,
    separator: &str,
) -> impl StrParser<Output = (T, U)> {
    move |input: &str| {
        let (first_part, rest_part) = input.split_once(separator).ok_or(ParseError::EmptyInput)?;
        Ok((first.parse(first_part)?, rest.parse(rest_part)?))
    }
}

/// Splits a string on the *last* occurrence of `separator`, parsing each part separately.
///
/// Everything before the final separator is passed to `body`; the trailing segment is
/// passed to `last`. Returns [`ParseError::EmptyInput`] if `separator` is not found.
///
/// # Example
/// ```
/// # use aoc_lib::utils::parser;
/// # use aoc_lib::utils::parser::Parser;
/// let p = parser::rsplit_once(parser::as_string, parser::as_string, "/");
/// assert_eq!(p.parse("a/b/c"), Ok(("a/b".to_string(), "c".to_string())));
/// ```
pub fn rsplit_once<T, U>(
    body: impl StrParser<Output = T>,
    last: impl StrParser<Output = U>,
    separator: &str,
) -> impl StrParser<Output = (T, U)> {
    move |input: &str| {
        let (rest, last_line) = input.rsplit_once(separator).ok_or(ParseError::EmptyInput)?;
        Ok((body.parse(rest)?, last.parse(last_line)?))
    }
}
