# Evaluation behavior decision

The first production behavior is `FiniteOrderedV1`, matching the documented
finite/ordered profile for kind-one accessors 1, 2 and 4. A typed selector must
reject every other signed ordinal. A populated table ignores its scalar and
conditional extra arrays; unrelated curves and shared arrays are not numeric
inputs to this operation. Preserve their serialized patterns unchanged.

Validate the finite query, all selected finite coordinates and outputs, and
strict coordinate ordering before clamps or search. Do not short-circuit those
checks for outside-range queries. Empty tables validate and return only their
scalar. Use the documented exclusive upper search bound and binary64 operation
order. Do not fuse the multiply/add, replace the formula with a numerically
rearranged expression, or add an exact-knot shortcut: FEV1 includes cases where
these choices change the result.

A complete supported evaluation may contain infinity or NaN caused by arithmetic
on finite inputs. Return its classification and explicit diagnostic rather than
quietly narrowing the accepted profile. NaN payload/sign is not a portable
arithmetic promise. Stored input bits and finite/infinity result bits remain
unchanged. Report copied scalar/clamp values as exact; report forward error as
unbounded for interpolation unless a separate certificate was actually computed.
Exact-rational measurements in the fixture are independent test evidence, not a
runtime numerical guarantee.

Publish an immutable Send/Sync result with the query bits, behavior version,
selected branch/bracket, record/body/curve provenance, executed validation scope,
arithmetic result and achieved-error information together. No file edit occurs;
retain the unchanged input checkpoint identity. Keep application acceptance,
engineering units, unsupported accessors and native floating-point environment
behavior distinct from this documented arithmetic operation.
