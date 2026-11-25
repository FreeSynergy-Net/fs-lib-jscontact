# Changelog

Versions follow ADR 0064 (0.0.N). No tag is set by the work branch.

## 0.0.1 (unreleased)

### English
- First version: the JSContact card (RFC 9553), its validation and JSON import/export, moved out of `fs-people-core` of the people program (source commit `b3e6381b5f7a77ae1cf892bcccced85e5b47e7c7`, files byte-equal in the first commit of this history).
- Changes against the source (second commit): own `Error`, `Issue`, `Result`; `Card::new` / `individual` / `application` / `group` take `uid` and `now` as parameters; `ImportedCard::original` holds a local `Original` value instead of the store's blob; no dependency on any other FreeSynergy crate.
- Not included: vCard, storage, the sensitive marking (`vendor`, `marks`), which move elsewhere.

### Deutsch
- Erste Version: die JSContact-Karte (RFC 9553), ihre Prüfung und JSON-Import/-Export, aus `fs-people-core` des Personen-Programms herausgenommen (Quell-Commit `b3e6381b5f7a77ae1cf892bcccced85e5b47e7c7`, Dateien im ersten Commit dieser Geschichte byte-gleich).
- Änderungen gegenüber der Quelle (zweiter Commit): eigene Typen `Error`, `Issue`, `Result`; `Card::new` / `individual` / `application` / `group` nehmen `uid` und `now` als Parameter; `ImportedCard::original` hält einen eigenen Wert `Original` statt des Blobs des Speichers; keine Abhängigkeit von einem anderen FreeSynergy-Crate.
- Nicht enthalten: vCard, Speicher, die Sensibel-Markierung (`vendor`, `marks`); sie ziehen anderswohin um.
