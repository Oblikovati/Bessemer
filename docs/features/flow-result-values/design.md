# Paired IMS-value interoperability

Issue: #241. This is planned work; no record-501 public API is shipped yet.

The reader will require a typed context derived from complete record-500 IMS
definitions and an explicit fresh-object declaration. Context creation will
validate every definition's version two and allocation byte zero once. It will
retain the exact immutable definition checkpoint and physical duplicate order.
A copied identifier list, caller-supplied count or equivalent new revision will
not stand in for that provenance. Pairing across separate physical carriers is
permitted only through this explicit retained context; the native corpus itself
uses CFLD definitions with FLD values.

Independent bounds will cover the entire inflated value body, actual objects,
total shared-array patterns, actual BL rows and total BL patterns. Multiplication
and allocation must follow count/profile/remaining-byte checks. A zero-object
context will admit the complete one-byte decoded payload `02`. Zero value and row
bounds must preserve physically present empty arrays/BL bodies. Shared-array and
BL counts must stay within the documented signed native-transfer range even when
the configured bounds are larger.

Views will retain byte version two, signed value version three, raw flag bytes,
all shared headers/intervals, array values and complete 18-word BL rows. Array
counts are not inferred from definition arrays or assigned to engineering grids.
The constant native BL state observation remains separate from stored bytes and
from any claim that native initialization ran.

Existing-field editors will replace raw patterns and flag bytes while retaining
the zero/nonzero presence class. A presence transition requires a structural
operation that supplies or removes a complete body; it must not reinterpret
following bytes silently. Each edit will copy the carrier, reparse under the
same retained definition context and bounds, verify complete decoded bytes and
layout, and return both checkpoints, diagnostics, coverage and physical history.
No-ops will preserve original compressed bytes. Fresh opposite edits will restore
decoded bytes without requiring a canonical compressed representation.

Acceptance will compare all owned field/presence/context projections, independent
literal byte patches, raw no-ops, fresh reversals and complete physical history.
It will cover exact/one-below bounds, unsupported or mismatched contexts, malformed
compressed and nested frames, input isolation, public thread traits, concurrent
operations and two-process byte agreement. Native runtime mutation, nonfresh
object loads, engineering columns and native application acceptance are separate
requirements and will remain explicit exclusions until independently verified.

The humanizer skill is unavailable; maintainer prose review remains pending.
