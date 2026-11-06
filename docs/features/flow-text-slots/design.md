# Record 311 typed interoperability

The reader will require explicit UTF-16 mode zero and complete EOF after fifty
slots. It will preserve the arbitrary signed header without version dispatch and
reuse the strict terminal-NUL framing helpers. Payload-byte and total transferred
unit limits will include hidden suffixes and terminal units. A zero unit budget
will admit fifty stored-empty strings within the payload bound.

Views will expose the raw stored units and first-NUL prefix separately. They will
retain empty versus NUL-only storage and support unpaired surrogates without lossy
Rust string conversion. No engineering slot names will be inferred from sample
values.

Existing state will have exactly fifty lossless UTF-16 text values. Projection
will produce immutable state, retaining complete previous values for empty native
prefixes and replacing other values with the native prefix. Caller state must
remain isolated after success, failure and caller-buffer mutation or drop.

Edits will replace the stored header or existing UTF-16 units, preserve counts
and slot order, revalidate terminal NULs and reopen the complete copied carrier.
Results will include coverage, diagnostics and full physical history. Acceptance
requires every owned read and state projection, literal byte patches, no-op and
fresh reversal checks, resource boundaries, malformed input rejection, source
isolation, public thread traits, concurrency and two-process agreement. Native
runtime application and engineering interpretation remain separate checks.
