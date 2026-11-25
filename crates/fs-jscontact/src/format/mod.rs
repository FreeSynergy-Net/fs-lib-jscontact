//! Import / export formats behind traits: module map.
//!
//! Pattern: Adapter (declarations) — each format file wraps a foreign representation (JSContact
//! JSON here; vCard and others plug in the same way) in the `Importer` / `Exporter` traits.

mod items;
mod jscontact;
mod traits;

pub use jscontact::JsContactFormat;
pub use traits::{Exporter, ImportItems, ImportedCard, Importer, Original};
