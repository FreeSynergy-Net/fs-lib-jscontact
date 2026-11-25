# Changelog

Versions follow ADR 0064 (0.0.N). No tag is set by the work branch.

## 0.0.1 (unreleased)

### English
- Name: the library is called `fs-lib-jscontact` (formerly `fs-jscontact`), repository `FreeSynergy-Net/fs-lib-jscontact`; libraries are named `fs-lib-<standard>`. Rename only, no logic changed. New tests through `JsContactFormat` (the message names the number, `1e400` is a single failure, doubles stay exact). README: vCard will come into this library later. Built with help from Claude (AI).
- First version: the JSContact card (RFC 9553), its validation and JSON import/export, moved out of `fs-people-core` of the people program (source commit `b3e6381b5f7a77ae1cf892bcccced85e5b47e7c7`, files byte-equal in the first commit of this history).
- Changes against the source (second commit): own `Error`, `Issue`, `Result`; `Card::new` / `individual` / `application` / `group` take `uid` and `now` as parameters; `ImportedCard::original` holds a local `Original` value instead of the store's blob; no dependency on any other FreeSynergy crate.
- Not included: vCard, storage, the sensitive marking (`vendor`, `marks`), which move elsewhere.

### Deutsch
- Name: Die Bibliothek heißt `fs-lib-jscontact` (früher `fs-jscontact`), Repo `FreeSynergy-Net/fs-lib-jscontact`; Bibliotheken heißen `fs-lib-<standard>`. Nur Umbenennung, keine Logik geändert. Neue Tests über `JsContactFormat` (Fehlermeldung nennt die Zahl, `1e400` ist ein einzelner Fehler, Doubles bleiben exakt). README: vCard kommt später in diese Bibliothek. Mit Hilfe von Claude (KI) gebaut.
- Erste Version: die JSContact-Karte (RFC 9553), ihre Prüfung und JSON-Import/-Export, aus `fs-people-core` des Personen-Programms herausgenommen (Quell-Commit `b3e6381b5f7a77ae1cf892bcccced85e5b47e7c7`, Dateien im ersten Commit dieser Geschichte byte-gleich).
- Änderungen gegenüber der Quelle (zweiter Commit): eigene Typen `Error`, `Issue`, `Result`; `Card::new` / `individual` / `application` / `group` nehmen `uid` und `now` als Parameter; `ImportedCard::original` hält einen eigenen Wert `Original` statt des Blobs des Speichers; keine Abhängigkeit von einem anderen FreeSynergy-Crate.
- Nicht enthalten: vCard, Speicher, die Sensibel-Markierung (`vendor`, `marks`); sie ziehen anderswohin um.
