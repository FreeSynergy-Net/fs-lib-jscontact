//! Numbers stay what they were, or they do not come in: plain `serde_json` keeps these, and refuses nothing
//! that a plain reader of the same data would stumble over. (The store and vCard parts of the original
//! file stay with the program that has a store; this is the part that needs only the card.)

use fs_jscontact::{Exporter, Importer, JsContactFormat};
use serde_json::{Value, json};

const BIG: &str = "123456789012345678901234567890";

fn card(uid: &str, n: &str) -> String {
    format!(
        r#"{{"@type":"Card","version":"1.0","uid":"{uid}","name":{{"full":"{uid}"}},"example.org:n":{n}}}"#
    )
}

#[test]
fn unknown_properties_and_details_keep_what_comes_in() {
    let text = r#"{"@type":"Card","version":"1.0","uid":"u","name":{"full":"A"},"example.org:n":{"i":18446744073709551615,"f":1.5,"small":7}}"#;
    let cards = JsContactFormat.import(text).unwrap();
    let imported = cards.into_iter().next().unwrap().unwrap();
    assert_eq!(imported.card.extra.get("example.org:n").unwrap()["f"], json!(1.5));
    let out = JsContactFormat.export(&imported.card).unwrap();
    let back: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        back["example.org:n"].to_string(),
        r#"{"f":1.5,"i":18446744073709551615,"small":7}"#
    );
}

#[test]
fn an_import_names_the_cards_with_a_number_it_cannot_keep_and_reads_the_rest() {
    let text = format!(
        "[{},{},{},{}]",
        card("big", BIG),
        card("fine", "1.5"),
        card("zero", "-0"),
        card("long", "1.2345678901234567890")
    );
    let items = JsContactFormat.import(&text).unwrap();
    assert_eq!(items.len(), 4);
    assert!(matches!(items[0], Err(fs_jscontact::Error::InexactNumber(_))));
    assert!(items[1].is_ok());
    assert!(matches!(items[2], Err(fs_jscontact::Error::InexactNumber(_))));
    assert!(matches!(items[3], Err(fs_jscontact::Error::InexactNumber(_))));
}
