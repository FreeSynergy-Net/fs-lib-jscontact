//! Error type of the JSContact card.
//!
//! Pattern: none (plain error enum via `thiserror`) — one `Error` for every public operation,
//! with a `From` conversion for the JSON layer.

/// One validation problem, with a JSON-pointer-like path into the card.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Issue {
    /// JSON-pointer-like path, e.g. `/emails/e1`.
    pub path: String,
    /// What is wrong.
    pub message: String,
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

/// Result of every fallible operation of this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Everything that can go wrong when reading, writing or checking a card.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The card violates RFC 9553 (all problems listed).
    #[error("card violates RFC 9553: {}", .0.iter().map(ToString::to_string).collect::<Vec<_>>().join("; "))]
    Invalid(Vec<Issue>),
    /// JSON could not be read or written.
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    /// A number that plain JSON handling would change: refused where it comes in.
    #[error("the number {0} cannot be kept exactly; write it as a string")]
    InexactNumber(String),
}
