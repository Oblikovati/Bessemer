# Weldment features and custom properties

Bessemer reads, edits and writes the complete ordinary modern weldment feature
above version `0x27CB` in non-DB external-XML model archives. Its inherited body is
the existing model-feature serializer, including word +354. It adds neither the
atom-feature layer nor the body-feature arrays, keeper or suffixes. The weldment
continuation is one direct custom-property list and a preserved signed raw word.

The direct list reuses the existing checked custom-property reader. Its signed
count is bounded by the 24-byte minimum property body and aggregate entry limits.
Direct records remain ordered, including duplicate names and equal record values;
they allocate no MFC object slots. The tagged `moCustomPropertyList` keeps its
existing reader, and tagged `moCustomProperty` instances now use the same complete
record body with an explicit ordinary-context guard.

Four CStrings and five words retain their documented order and widths: three
strings, a signed word, an unsigned word, the fourth string and three signed
words. Newlines, CR/LF pairs, at-sign expressions, Unicode and raw unsigned values
above the signed range remain unchanged. Native localization, choice-label
rewriting and raw-word interpretations are not executed offline.

Independent readers verified 42 complete cases and 2,367 primitive observations.
Three versions cover the ordinary lower boundary, current fixtures and the full
u32 version domain. Cases include empty lists, minimum records, the existing
populated property list, three equal direct records and tagged lists/records with
cached aliases. The original custom-property fixture builder now exposes its
single-record constructor for reuse; both previous Python regression tests pass
with unchanged original, edited and primitive-oracle bytes.

Four owned streams import the blank part's actual 97 slots and finish at 101
slots regardless of property count. A separately constructed edit changes only
the first duplicate record's value while retaining its neighbors and stored
expressions. Public tests reconstruct complete documents, compare that edit and
verify source isolation after rejected count edits. Four private tests compare
every field/byte/slot, reject every truncated prefix and trailing byte, check
negative/impossible counts and require exactly three entries for three minimum
direct records. Three Python tests regenerate every fixture and check the matrix.

`reference.json` binds stable before/after sibling reader hashes, generators,
reused builders, primitive oracles and map slots. The schema snapshot retains the
native model-feature/direct-property boundary evidence. Structural members and
other newly encountered nested classes remain separate complete-record work.
Native application acceptance, raw property meanings, native text rewriting,
geometric validity, DB/deferred archives, historical formats and full documented
schema parity remain unproven. No dependency or package version changed. The
humanizer skill is unavailable; its prose pass remains with the maintainer.

Two fresh native processes retain all 686 complete configurations, with identical
outcomes and causes. Ten failures advance: nine reach structural-member features
and one reaches a two-entity reference dimension. No additional configuration
completes yet; 220 explicit stops remain. The unchanged accepted input set matches
all four hashes per case in the previous independent exact-EOF audit of
40,016,822 feature bytes. The verification record binds that report and both
new replay results.

The staged Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0: 535 library
tests, four xtask tests, every integration suite and 265 doctests. Three external
corpus gates remain ignored by default; both native replays above ran explicitly.
