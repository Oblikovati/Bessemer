# Shared bounded zlib decoder (#241)

Flow record 433 and PNG compressed ancillary fields both require exactly one
complete RFC 1950 stream under an explicit output bound. The bounded decoder
formerly belonged to the PNG module. It now lives in private `bounded_zlib`;
the PNG reader remains its shipped caller. Record-433 parsing will use this same
decoder instead of maintaining a second streaming implementation.

The extraction preserves the 8,192-byte scratch buffer, one-byte over-budget
probe, saturating budget arithmetic, progress check, checksum validation and
exact compressed-input consumption. Typed internal errors distinguish budget
exhaustion, codec errors, stalled/truncated input and trailing input. The PNG
adapter reconstructs the original operation, entity, failure class and messages.
No public API, dependency or encoded-output behavior changes in this refactor.

Before extraction, owned tests pinned exact remaining-budget acceptance and
one-byte rejection, zero-byte output, exact trailing/concatenated consumption
messages, empty/truncated streams, corrupted checksums and raw/gzip rejection.
The four existing PNG integration suites and these contracts passed together
before extraction. The baseline report records the unchanged production decoder
and new test hashes. The same frozen tests are run after extraction; fixture and
contract expectations are not updated for the refactor.

The record-433 work still requires exact selector/count/array decoding, explicit
runtime-count context, bounded typed edits, independent fixtures, provenance and
carrier integration. The conditional count-source proof in specification 14.44
does not authorize guessing that initialization occurred for an arbitrary owner.

The humanizer skill is unavailable; its prose pass remains deferred to review.
