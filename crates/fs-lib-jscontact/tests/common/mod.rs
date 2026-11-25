//! Shared helpers of the integration tests.

#![allow(dead_code)] // each test file uses a different subset

use fs_lib_jscontact::Card;

/// The demo cards: a copy of the documentation tree's `temp/demo-data/contacts.json`, kept here so the tests need nothing outside the repository.
pub const DEMO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/contacts.json");

/// A fixed time for cards made in tests.
pub const NOW: &str = "2026-11-25T10:00:00Z";

/// A person card with one e-mail address.
pub fn person(name: &str, email: &str) -> Card {
    let mut c = Card::individual(name, "urn:uuid:00000000-0000-7000-8000-000000000001", NOW);
    c.add_email(email);
    c
}
