# FreeSynergy JSContact: one contact card for every program

**A building block that gives every FreeSynergy program the same idea of what a contact card is.**

FreeSynergy JSContact is a small library for the JSContact card (RFC 9553): the typed card, its checks, and reading and writing it as JSON. Without it, every program that knows people would keep its own copy of the card and slowly drift apart. It is not a program. It holds no data and has no storage.

Anything the library does not understand is kept as it came in, and numbers that plain JSON would change are refused instead of being quietly altered.

Built with help from Claude (AI). Licensed under MIT OR Apache-2.0.

## What it can do today

- The whole card of RFC 9553 as typed members: names, nicknames, organisations, e-mail, phones, addresses, links, relations, notes, dates and the rest. Nothing of the standard sits in a loose "other" map.
- Unknown and vendor properties are kept as they came in.
- Validation that reports all problems at once, with a path into the card.
- JSContact JSON as import and export (one card or an array); one item that cannot be read stays one failure among the others.
- Plug-in point for other formats: the traits `Importer` and `Exporter`.
- No clock and no id source: the caller gives `uid` and time.

## What is not there yet

- No vCard. It will come as a separate part, together with import and export in the program that needs it.
- No storage, no search, no program. Those stay with the modules that use the card.
- This is a first move out of the people program (see the CHANGELOG). Only the tests that need just the card live here: 18 files of the old test suite need a store and stay there.

## Part of FreeSynergy

FreeSynergy is a family of programs that are free for everyone, forever, and that keep your data yours. The documentation, the manifesto and the overall plan live in [fs-documentation](https://github.com/FreeSynergy/fs-documentation).

## For developers

- Workspace crate: `fs-jscontact`. Dependencies: `serde`, `serde_json`, `thiserror`; no other FreeSynergy crate.
- Build and test: `cargo build --workspace` and `cargo test --workspace`.
- Design: [DESIGN.md](DESIGN.md).
- Licence: MIT OR Apache-2.0.

---

## Deutsch

**Ein Baustein, damit jedes FreeSynergy-Programm unter einer Kontaktkarte dasselbe versteht.**

FreeSynergy JSContact ist eine kleine Bibliothek für die JSContact-Karte (RFC 9553): die typisierte Karte, ihre Prüfungen und das Lesen und Schreiben als JSON. Ohne sie hielte jedes Programm, das Personen kennt, eine eigene Kopie der Karte, und die Kopien liefen auseinander. Sie ist kein Programm, hält keine Daten und hat keinen Speicher.

Was die Bibliothek nicht versteht, bleibt so erhalten, wie es hereinkam. Zahlen, die einfaches JSON verändern würde, werden abgelehnt statt still verändert.

Mit Hilfe von Claude (KI) gebaut. Lizenz: MIT OR Apache-2.0.

### Was heute geht

- Die ganze Karte nach RFC 9553 als typisierte Felder (Namen, Spitznamen, Organisationen, E-Mail, Telefon, Adressen, Links, Beziehungen, Notizen, Daten und der Rest).
- Unbekannte und herstellereigene Eigenschaften bleiben erhalten.
- Prüfung, die alle Fehler auf einmal meldet, mit Pfad in der Karte.
- JSContact-JSON als Import und Export (eine Karte oder eine Liste); ein unlesbarer Eintrag bleibt ein einzelner Fehler.
- Einsteckstelle für weitere Formate: die Traits `Importer` und `Exporter`.
- Keine Uhr und keine Id-Quelle: `uid` und Zeit gibt der Aufrufer.

### Was noch fehlt

- Kein vCard. Es kommt später als eigener Teil, zusammen mit Import und Export im Programm, das es braucht.
- Kein Speicher, keine Suche, kein Programm. Das bleibt bei den Modulen, die die Karte nutzen.
- Dies ist der erste Umzug aus dem Personen-Programm (siehe CHANGELOG). Hier liegen nur die Tests, die allein die Karte brauchen.
