# Vector style controls

Specification section14.504 and C-886 bind the following stored fields:

| Stored field | Bound role |
|---|---|
| style.word_28c | Raw vector size integer |
| style.word_290 | First raw vector density integer |
| style.word_294 | Second raw vector density integer |
| style.word_298 | Raw vector color selector integer |
| style.color_29c | All four original vector RGBA components |

The group exists only above version700329. Earlier absence is not an encoded
zero or an initializer default. Sizes, density arguments and selector values
remain raw; the bindings establish neither pixel units nor allowed enum values.

The three named CDspApp setters resolve an active object first. A null result
bypasses the target. Their nonnull paths preserve integer arguments and tail-call
the bound targets. The complete size leaf stores its argument and returns1.
Density writes both words before subsequent recalculation/update calls; those
downstream operations are outside this clause's model. Exact exported signatures
return integers, so a local-field model must not invent downstream return codes.

The color target stores its selector, constructs COGLColor from a packed unsigned
argument, then uses COGLColor **copy assignment**. Copying that constructed color
includes its alpha of one. The separate packed assignment overload preserves
existing alpha and is not this setter's operation. A conditional nearest-even
model can reuse the existing packed constructor; it cannot infer the current
native floating-point environment from an archive.

Complete-body fixture replay checks all224 original native sets, including each
integer, color component and wire offset. It also retains all425 owned and276
native parent/style contexts:385 owned and224 native groups are present;40 owned
and52 native groups remain absent. Corpus frequencies do not impose defaults
or value restrictions. Source's entire current catalog unit module is retained,
including its later fixed-palette check.

Static verification covers all seven complete chain functions, both object-base
links, named exports, the twelve-byte size leaf and both exact color imports.
Decoder presentation normalization preserves contiguous bytes and method hashes.
Only observed redundant NOP address spellings, byte-checked padding prefixes and
the exact6690 NOP alias are normalized. Meaningful prefixes and non-NOP operand
spellings remain strict; existing C-885 method bindings are regression-checked.

Parent and PostStyle `vector_style_controls()` queries return
`SimulationVectorStyleControls` from their canonical field snapshots. Older
missing groups return a typed missing-field error. The raw words remain signed
integers and the color retains every original bit.

`after_size_write`, `after_density_writes` and
`after_color_copy_nearest_even` return copied local state. Each requires explicit
active-object presence and preserves all input values on the null path. Only
the documented writes are modeled; downstream callbacks, renderer execution and
complete native failure/return behavior are not claimed. Persisting values uses
the existing typed archive edit APIs and their history contracts.

Independent fixtures cover2,922 active/null requests. They combine every packed
RGB byte and high-byte value, signed control boundaries and raw original alpha
patterns with the actual copied Source packed constructor. The color result
copies all four constructed channels; it never borrows the packed assignment
overload's alpha-preservation policy.

Six Rust tests, all-target Clippy and all150 integrated reference checks passed.
Eight workers matched serial requests and all425 owned views. Two fresh
processes matched all3,623 request/archive contexts and their76,437 output words
against the independent expected bytes. [Verification](vector-style-operation-verification.json)
records exact evidence, assumptions and the remaining publication and role gates.
