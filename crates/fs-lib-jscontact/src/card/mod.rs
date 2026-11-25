//! JSContact `Card` (RFC 9553): module map.
//!
//! Pattern: Facade — this module only declares the concept files and re-exports the model, so
//! callers keep writing `fs_lib_jscontact::card::Card`.

mod about;
mod build;
mod display;
mod legacy;
mod model;
mod parts;
mod relations;
mod resources;
mod sub_object;
mod types;

pub use about::{Anniversary, AnniversaryDate, Author, Note, PersonalInfo};
pub use model::Card;
pub use parts::{
    Address, AddressComponent, EmailAddress, LanguagePref, Name, NameComponent, Nickname,
    OnlineService, OrgUnit, Organization, Phone, Pronouns, SortAs, SpeakToAs, Title,
};
pub use resources::{Calendar, CryptoKey, Directory, Link, Media, SchedulingAddress};
pub use types::{DESCRIBES_KEY, Extra, Map, SENSITIVE_KEY, UNCLEAR_SENSITIVE, VENDOR};
