#![allow(dead_code)]

mod char_parser;
mod error;
mod str_parser;

use std::fmt::Display;
use std::str::FromStr;

pub use char_parser::CharParser;
pub use error::ParseError;
pub use str_parser::{lsplit_once, rsplit_once, split_pair, uncons, StrParser};

// === Core Trait ===

/// A generic parser that consumes an input of type `I` and produces a typed result.
///
/// The `Parser` trait is the foundation of this library. Any type implementing it can
/// be composed using the combinator methods [`map`](Parser::map),
/// [`and_then`](Parser::and_then), and [`into_each`](Parser::into_each).
pub trait Parser<I> {
    /// The type produced on a successful parse.
    type Output;

    /// Attempt to parse `input`, returning `Ok(Self::Output)` on success or a
    /// [`ParseError`] on failure.
    fn parse(&self, input: I) -> Result<Self::Output, ParseError>;

    /// Transform the output of this parser by applying `f` to it.
    ///
    /// Analogous to [`Result::map`]: if parsing succeeds, `f` is called on the result;
    /// if it fails, the error is propagated unchanged.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::Parser;
    /// let p = parser::from_str::<u32>.map(|n| n * 2);
    /// assert_eq!(p.parse("21"), Ok(42));
    /// ```
    fn map<F, U>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> U,
    {
        Map { parser: self, f }
    }

    /// Chain this parser with a fallible transformation `f`.
    ///
    /// Analogous to [`Result::and_then`]: if parsing succeeds, `f` receives the output
    /// and may itself return an error, allowing validation or further parsing in a
    /// single step.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::{Parser, ParseError};
    /// let positive = parser::from_str::<i32>.and_then(|n| {
    ///     if n > 0 { Ok(n) } else { Err(ParseError::Other("expected positive".into())) }
    /// });
    /// assert_eq!(positive.parse("5"), Ok(5));
    /// assert!(positive.parse("-1").is_err());
    /// ```
    fn and_then<F, U>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> Result<U, ParseError>,
    {
        AndThen { parser: self, f }
    }

    /// Lift this parser to operate over every item in a collection.
    ///
    /// Returns a new parser that accepts any `C: IntoIterator<Item = I>` and applies
    /// `self` to each element, collecting the results into a `Vec`. Fails fast on the
    /// first element that does not parse.
    ///
    /// # Example
    /// ```
    /// # use aoc_lib::utils::parser;
    /// # use aoc_lib::utils::parser::Parser;
    /// let p = parser::from_str::<u32>.into_each();
    /// assert_eq!(p.parse(vec!["1", "2", "3"]), Ok(vec![1, 2, 3]));
    /// ```
    fn into_each(self) -> IntoEach<Self>
    where
        Self: Sized,
    {
        IntoEach { parser: self }
    }
}

/// Blanket [`Parser`] implementation for plain functions `Fn(I) -> Result<T, ParseError>`.
///
/// This lets any compatible function be used directly wherever a `Parser` is expected,
/// without needing a wrapper type.
impl<I, T, F> Parser<I> for F
where
    F: Fn(I) -> Result<T, ParseError>,
{
    type Output = T;

    fn parse(&self, input: I) -> Result<Self::Output, ParseError> {
        self(input)
    }
}

/// A parser that applies a mapping function to the output of an inner parser.
///
/// Constructed via [`Parser::map`].
pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<I, U, P, F> Parser<I> for Map<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> U,
{
    type Output = U;

    fn parse(&self, input: I) -> Result<Self::Output, ParseError> {
        self.parser.parse(input).map(|v| (self.f)(v))
    }
}

/// A parser that applies a fallible mapping function to the output of an inner parser.
///
/// Constructed via [`Parser::and_then`].
pub struct AndThen<P, F> {
    parser: P,
    f: F,
}

impl<I, U, P, F> Parser<I> for AndThen<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> Result<U, ParseError>,
{
    type Output = U;

    fn parse(&self, input: I) -> Result<Self::Output, ParseError> {
        self.parser.parse(input).and_then(|v| (self.f)(v))
    }
}

/// A parser that applies an inner parser to every item in a collection.
///
/// Constructed via [`Parser::into_each`].
pub struct IntoEach<P> {
    parser: P,
}

impl<P, C> Parser<C> for IntoEach<P>
where
    P: Parser<C::Item>,
    C: IntoIterator,
{
    type Output = Vec<P::Output>;

    /// Parses each item in `input` using the inner parser, collecting successes into a
    /// `Vec`. Returns the first error encountered, if any.
    fn parse(&self, input: C) -> Result<Self::Output, ParseError> {
        input
            .into_iter()
            .map(|item| self.parser.parse(item))
            .collect()
    }
}

// === Standalone parsers ===

/// Returns the input string slice unchanged.
///
/// Useful as a no-op parser when a `Parser<&str, Output = &str>` is required.
pub const fn as_str(s: &str) -> Result<&str, ParseError> {
    Ok(s)
}

/// Converts the input string slice into an owned [`String`].
pub fn as_string(s: &str) -> Result<String, ParseError> {
    Ok(s.to_string())
}

/// Returns any input value unchanged, always succeeding.
///
/// The equivalent of [`std::convert::identity`] in parser form.
pub const fn identity<T>(item: T) -> Result<T, ParseError> {
    Ok(item)
}

/// Discards the input string slice and returns `()`.
///
/// Useful when a parser is required for its side-structure (e.g. in a combinator)
/// but the value itself is not needed.
pub const fn unit(_s: &str) -> Result<(), ParseError> {
    Ok(())
}

/// Parses a string slice into any type that implements [`FromStr`].
///
/// The [`FromStr::Err`] type must implement [`Display`] so it can be converted into
/// a [`ParseError`].
///
/// # Example
/// ```
/// # use aoc_lib::utils::parser;
/// assert_eq!(parser::from_str::<u32>("42"), Ok(42));
/// assert!(parser::from_str::<u32>("abc").is_err());
/// ```
pub fn from_str<T>(s: &str) -> Result<T, ParseError>
where
    T: FromStr,
    T::Err: Display,
{
    s.parse::<T>().map_err(|e| e.to_string().into())
}

/// Interprets a character as a digit in the given `RADIX`, returning its numeric value.
///
/// `RADIX` is a const generic parameter (e.g. `10` for decimal, `16` for hex).
/// Returns [`ParseError::NotADigit`] if the character is not a valid digit in that base.
///
/// # Example
/// ```
/// # use aoc_lib::utils::parser;
/// assert_eq!(parser::digit::<10>('7'), Ok(7));
/// assert_eq!(parser::digit::<16>('f'), Ok(15));
/// assert!(parser::digit::<10>('z').is_err());
/// ```
pub fn digit<const RADIX: u32>(c: char) -> Result<u32, ParseError> {
    c.to_digit(RADIX).ok_or(ParseError::NotADigit(c))
}
