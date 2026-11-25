//! `fs-jscontact`: the JSContact `Card` (RFC 9553) of FreeSynergy, without storage.
//!
//! A typed model of the card, its validation and JSON import/export. Unknown and vendor
//! properties are kept. The crate holds no data and knows no store: `uid` and time come in as
//! parameters. Other formats plug in through the [`Importer`] / [`Exporter`] traits.
//!
//! Pattern: Facade — this file only declares modules and re-exports the public API.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

pub mod card;
mod error;
pub mod format;
pub mod validate;

pub use card::Card;
pub use error::{Error, Issue, Result};
pub use format::{Exporter, ImportItems, ImportedCard, Importer, JsContactFormat, Original};
