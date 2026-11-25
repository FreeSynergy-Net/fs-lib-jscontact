# fs-lib-jscontact – Design

The JSContact card (RFC 9553) as a plain library: typed model, validation, JSON import/export. It holds no data, stores nothing and is not a program. The point of truth for people stays with the person module; this crate is the one shared description of the card (one module for the standard, no copies).

## State

Version 0.0.1, moved out of `fs-people-core` (see CHANGELOG). No clock, no id source, no store, no fs-* dependency.

## Pattern map (ADR 0043)

| Part | Pattern | Why |
|---|---|---|
| `lib.rs`, `card/mod.rs`, `format/mod.rs` | Facade | declare modules, re-export the API |
| `card/` model files | Builder (convenience constructors), Chain of responsibility (display name) | build a valid card step by step; first non-empty name wins |
| `validate/` | Specification, Visitor-style walk with a sink | pure predicates over the card; all problems at once |
| `format/traits.rs` | Strategy + Adapter | `Importer` / `Exporter` are the plug-in point of every foreign format |
| `format/jscontact.rs` | Adapter | the native wire form as an `Importer` / `Exporter` |
| `format/items.rs` | Iterator-style scan | one unreadable item stays one failure |
| `error.rs` | none (plain error enum) | one `Error` for every operation |
| `Original` | none (plain value) | media type and bytes of a source |

## Not here

vCard (comes with import/export), storage, search, the sensitive marking.
