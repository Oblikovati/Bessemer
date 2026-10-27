# Base flanges

Bessemer reads, edits and writes complete ordinary modern `moSMBaseFlange_c`
records in non-DB external-XML model archives above version `0x27CB`. The reader
reuses the complete body-feature and common-bend layers, including validated
saved helper state. It retains the direct bend-group array before native runtime
filtering, preserving nulls, duplicate references and the following raw word.

The raw mode selects slot zero, slot one, both or neither. Equality comparisons
preserve signed extremes without overflowing a subtraction. Each selected slot
retains its raw type, the documented conditional value/reference payload, a
display dimension and an independent stored double. Type 3 reads its reference
even when the reference byte is zero; types 4 and 5 omit that tag when it is zero.
Unknown types proceed directly to the dimension. Noncanonical byte values remain
unchanged.

Four direct handle collections each use a signed count. Empty collections end
after the count. Nonempty collections store one sketch pointer and then their
handles, preserving repeated and null targets. Declared pointer roles and count
budgets reject unrelated targets and malformed extents before publishing an
archive. Late raw-word and tagged from-end fields have separate version gates.

Independent readers consumed 202 cases to exact EOF with 74,248 projected
primitive observations. Twelve versions cover the inherited and late-field
boundaries. The matrix includes both saved states, signed selector/type extremes,
all documented payload branches, shared complete sketches and handles, populated
allowance grids and direct-array aliases. The existing sheet-metal raw-span
projection preserves each original byte and its source provenance.

Four owned streams import the blank part's actual 97 slots and finish with
101, 119, 121 and 121 slots. Complete saved managers select the grammar. An
independent edit changes only the selected second direction's extra value from
negative zero to 123.5. Public tests verify whole-document reconstruction, the
edit and input isolation after success and rejection. Private tests compare every
field, byte and slot, reject every truncated prefix and trailing bytes, and check
malformed counts and wrong cached pointer classes. Python tests regenerate all
fixtures and check the branch matrix and constructor boundaries.

`reference.json` binds source, generator and reused-builder hashes. An immutable
workspace copy of the approved sibling readers avoids mixing revisions while
that repository changes. The source snapshot is checked before and after
generation. `schema-snapshot.md` retains the relevant specification clauses.

Two fresh native processes retain all 638 previously complete configurations
with identical outcomes and causes. Twenty base-flange failures advance: twelve
reach flat patterns, four edge flanges, two hems and two sketch bends. The
remaining 268 configurations still stop explicitly. The unchanged accepted
inputs retain the previous independent exact-EOF audit of 37,003,070 bytes;
`local-verification.json` binds that report and the new replay evidence. Existing
sheet-metal fixtures regenerate unchanged, and all five Python regression tests
pass with the extended observer's default behavior.

The Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0: 511 library tests,
four xtask tests, every integration suite and 265 doctests. Three external corpus
gates remain ignored by default; the two native replays above ran explicitly.

Native application acceptance, geometric validity, DB/deferred archives, older
formats and full schema parity remain unproven. Native runtime owner reassignment
and filtering do not mutate offline shared objects. No dependency or package
version changed. The humanizer skill is unavailable; its prose pass remains with
the maintainer.
