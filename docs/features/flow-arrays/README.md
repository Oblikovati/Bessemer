# Flow compressed record 433 (#241)

`FlowCarrier::record_433_arrays` reads one revision-bound occurrence using the
strict serialized profile in specification 14.30. It checks a single complete
RFC 1950 stream, its checksum and compressed EOF under an explicit decoded-byte
bound. The decoded payload stores a nonnegative signed selector and count.
Selector zero contains two signed i32 arrays of that count; every positive
selector contains only the second array. Exact decoded EOF is required before
allocating either array. Omitted first arrays remain distinct from present empty
arrays. Negative values and duplicates inside the arrays are preserved.

```rust,ignore
let options = FlowArrayOptions {
    maximum_decoded_bytes: 64 * 1024 * 1024,
    maximum_elements: 1_000_000,
    expected_count: None,
};
let arrays = carrier.record_433_arrays(&record_id, options)?;
let edit = arrays.replace_value(FlowArrayGroup::Second, 0, -1)?;
let output = edit.result().source().as_bytes();
```

An optional `expected_count` enforces exact equality with an externally supplied
owner count. Coverage explicitly reports `NotSupplied` or `MatchedCallerCount`;
matching does not verify the supplied value's provenance. Specification 14.44
proves a conditional initializer assigning G.Fluid + G.Solid to the native count
slot, but an arbitrary owner cannot be assumed to have taken that path. This
decoder performs no owner initialization and never infers context from the count
it is meant to check.

`replace_value` edits one existing signed cell, reparses the complete carrier and
array result, and verifies that exactly the selected four decoded bytes changed.
The result retains source/result arrays, complete carrier history, selected-cell
history and the exact original carrier checkpoint. Failed bounds, missing arrays,
bad indexes and compressed growth beyond carrier limits publish no result.
No-ops preserve the source revision and original compressed bytes. A fresh opposite
edit restores logical array bytes but need not recover the original zlib encoding;
the edit diagnostic states that limitation explicitly.

Coverage is `ExactZlibAndArrayFrames`, with compressed and decoded sizes, element
counts, physically present arrays, explicit count-check status and zero achieved
byte error. Array roles, units, entity domains, runtime pointer population and
native application/geometry acceptance remain unverified. Length/selector changes,
new record construction and the later Flow payload schemas remain required work.

The frozen independent packet contains all 5,952 values in each native array and
twelve owned selector/count profiles, including positive selectors above one,
empty arrays and signed boundaries. Python regeneration uses a temporary injected
root; tests never rewrite the reviewed fixtures. Existing PNG contracts continue
to exercise the shared bounded decoder. Native byte bindings authenticate twelve
retained viewer/provider ranges and the exact delay import, including the sixteen-byte
initializer that selects zlib window bits 15; they add no native execution credit.

Bug #242 corrected a test precondition, preserving the frozen arrays and strict
wire limit: native recompression shrinks the sample, so a constructed zero-array
carrier that demonstrably grows is used for the failure/isolation regression.

The humanizer skill is unavailable; its prose pass remains deferred to review.
