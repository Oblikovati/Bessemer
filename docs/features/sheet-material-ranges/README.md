# Sheet-metal material parameters and thickness ranges

Issue #214 adds the stored body of `moSMParamsFromMat_c` reached through the
existing allowance material pointer. The ordinary modern reader retains two
signed words, the table-name CString, the signed count bits and direct range
records. Each record is 37 bytes: i32, binary64, i32, binary64, i32, u8,
binary64. There is no per-record tag, schema, padding or runtime vector stride.

The count bits are zero-extended for bounds checks before reading records.
The reader preserves operator, unit and method codes, floating-point payloads,
ordering and duplicate records. It does not perform native interval matching,
provider calls, unit conversion, normalization or material application.

The first literal corpus contains all 256 method octets, signed boundary values
and 12 binary64 patterns including signed zeros, subnormals, infinities and NaNs.
These are synthetic expected bytes derived independently from the static
specification. No original Source production parser, newly observed native
populated material, geometry or SolidWorks application acceptance is claimed.

Forty-four complete containing streams exercise both sheet-metal feature
classes and both saved manager states. Forty independently expected edits and
fresh reversals preserve revision-bound history, exact stored-field result
contracts and isolated inputs. Eight workers repeat those edits concurrently.
Context, budget, duplicate-record and prior sheet-metal regressions are checked
separately from native geometry and application acceptance.

The full local gate and exact-head remote checks remain required before
integration. This feature is excluded from PR212. The humanizer skill is
unavailable; its prose review remains with the maintainer.
