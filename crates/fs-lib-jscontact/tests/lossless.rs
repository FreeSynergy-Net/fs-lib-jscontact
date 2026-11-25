//! Numbers stay what they were, or they do not come in: plain `serde_json` keeps these, and refuses nothing
//! that a plain reader of the same data would stumble over. (The store and vCard parts of the original
//! file stay with the program that has a store; this is the part that needs only the card.)

use fs_lib_jscontact::{Exporter, Importer, JsContactFormat};
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
    assert_eq!(
        imported.card.extra.get("example.org:n").unwrap()["f"],
        json!(1.5)
    );
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
    assert!(matches!(
        items[0],
        Err(fs_lib_jscontact::Error::InexactNumber(_))
    ));
    assert!(items[1].is_ok());
    assert!(matches!(
        items[2],
        Err(fs_lib_jscontact::Error::InexactNumber(_))
    ));
    assert!(matches!(
        items[3],
        Err(fs_lib_jscontact::Error::InexactNumber(_))
    ));
}

#[test]
fn the_message_names_the_number_that_was_refused() {
    for token in [BIG, "-0", "1.2345678901234567890"] {
        let items = JsContactFormat.import(&card("x", token)).unwrap();
        let message = items[0].as_ref().unwrap_err().to_string();
        assert!(message.contains(token), "{message}");
    }
}

#[test]
fn a_number_too_large_for_a_double_is_one_failure_and_the_next_card_is_read() {
    let text = format!("[{},{}]", card("inf", "1e400"), card("next", "2"));
    let items = JsContactFormat.import(&text).unwrap();
    assert_eq!(items.len(), 2);
    assert!(items[0].is_err());
    assert!(items[1].is_ok());
}

#[test]
fn a_double_comes_back_exactly_through_export() {
    let text = r#"{"@type":"Card","version":"1.0","uid":"u","name":{"full":"A"},"example.com:lat":13.049435775302797}"#;
    let imported = JsContactFormat
        .import(text)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    let out = JsContactFormat.export(&imported.card).unwrap();
    assert!(out.contains("13.049435775302797"), "{out}");
}
