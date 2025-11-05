//! Defines the `Token` and `TokenType` enums.
//!
//! These are used as an intermediate representation between the
//! `Tokenizer` (lexer) and the `StreamingParser` (parser).
//! This module is part of the library's internal API.

// --- 2. Token Structs ---

/// The specific type of a `Token`.
///
/// This represents the smallest meaningful units of JSON grammar.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// A string, e.g., `"hello"`
    String(String),
    /// A number, e.g., `123.4`
    Number(f64),
    /// A boolean, `true` or `false`
    Boolean(bool),
    /// The `null` literal
    Null,
}

/// A single token produced by the `Tokenizer`.
///
/// It contains the `TokenType` and its location (line and column)
/// in the source string, which is crucial for error reporting.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// The type of the token.
    pub(crate) kind: TokenType,
    /// The 1-indexed line number where the token starts.
    pub(crate) line: usize,
    /// The 1-indexed column number where the token starts.
    pub(crate) column: usize,
}
