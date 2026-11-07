# Attachment records

Feature #54 implements the distinct `moAttachPointData_c` and
`moParametricPoint_c` layouts from the camera attachment clause and
11.183 / C-211. Behavior version: `C211-attachment-records-v1`.

Attachment data has primary table `0x4CE7C940` and serializer `0x4C27A430`.
Its ordinary empty object base contributes no fields. It reads two direct
three-double points, one tagged `moDrView` pointer and two raw doubles. Its
null/cached-view body occupies 66 bytes. It is an independent object class,
not an attachment-point subtype, and adds no inherited four-point suffix.

Parametric point serializer `0x4C280A80` reuses the complete ordinary attachment
base, then reads two doubles, a generic tagged reference, u16 and i32. With null
pointers its body occupies 127 bytes. The reader preserves generic reference
identity and admits this exact native subtype through attachment-point roles.
The drawing-only five-point suffix above version `0x335C` remains outside the
existing ordinary part context. Other model roots and special condition-1/2
attachment layouts remain unsupported.

Independent fixtures cover 60 graphs, 2,160 primitive observations and fourteen
owned streams across five versions and six identity styles. They retain raw
signed zero, NaN payloads, infinities, signed words, shared generic references,
distinct/repeated roots, typed attachment parents and null fields. Two edit
goldens change only the selected data double or parametric signed word. Tests
compare all fields, map slots and bytes, every prefix and extra byte, old contexts,
exact body sizes and rejection of unrelated cached drawing-view pointers.

Run `cargo test --lib attachment_record_tests`,
`cargo test --test resolved_attachment_records` and `cargo xtask pr`.
Regenerate with
`python -B -X utf8 -m unittest discover -s docs/features/attachment-records`.
New view payloads remain independently dispatched and may stop explicitly if
unsupported; no view body is skipped. Geometry meanings, runtime repair, full
application acceptance and whole-schema parity remain incomplete. Humanizer is
unavailable; maintainer prose review remains open.
