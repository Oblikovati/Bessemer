### 14.487 Integrated legacy PostStyle parser view

*Status: verified legacy region views composed into the bounded parser; historical native fixtures and field purposes remain incomplete — Evidence: C-867*

For successful below700006 parses, **cosworks_post_style_layout.parse** now returns an additional **legacy_view** composed by **cosworks_post_v7_composed_view.py** from14.485–14.486. Its38 contiguous spans partition the368-byte record into18 RGBA records and20 raw four-byte pieces. Each color retains exact source bits and supplementary channel views; old-to-current member annotations remain separate from historical purpose, which is not invented for unmapped fields.

The original **style.legacy_raw** span and reconstruction hash remain available. The detailed spans are an overlapping alternate view of those same bytes, not additional consumed data. Consumers and coverage calculations must not sum the raw outer span together with its detailed view. The field catalog in14.479 continues to describe the parser's outer span labels; the nested legacy view is a separate structural refinement, not a reduction in its unresolved-purpose count.

All detailed offsets are absolute within the input supplied to the outer parser, including when parsing begins at a nonzero offset. Component-local offsets in channel dictionaries retain their existing within-color meaning. The composed view checks contiguous boundaries, span widths, exact byte reconstruction and its final end offset. It does not construct a native current-class object, apply unproven constructor state or replace original wire bytes with converted values.

Three new integration tests exercise a nonzero embedding offset and marker payload, all38 boundaries through exact reconstruction, preservation of the outer raw span, eighteen channel views, strict EOF versus prefix parsing, truncation rejection and absence of the legacy view at the700006 modern boundary. The four existing style/color tests repeat successfully, with legacy dependency hashes added to replay provenance.

C-867 repeats native replay:52 complete legacy-named style streams and224 modern dialog prefixes remain accepted, while eight legacy style failures remain explicit. Those accepted interpretations are not pre700006 historical fixtures; the populated integrated legacy view remains synthetic evidence. No new global coverage credit is added, retained version/prefix qualifications remain unchanged and no target code executes.

Evidence: C-867.
