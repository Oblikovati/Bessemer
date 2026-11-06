# Kind-one curve evaluation

Issue: #241. Reference: `../inventor-interface/solidworks/flowmediumevaluate.py`.

The documented numeric profile covers kind-one curve ordinals 1, 2 and 4.
A finite query is required. Empty tables return the finite stored scalar.
Populated tables require every selected input coordinate and output value to be
finite, and coordinates must be strictly increasing. Validation precedes endpoint
clamping, so an invalid later value still rejects an outside-range query.
The populated table's scalar and its conditional extra arrays are not evaluated.
Other curves and shared arrays retain their raw bits without numeric validation.

Queries at or below the first coordinate return the first output; queries at or
above the last coordinate return the last output. Interior search uses the count
as an exclusive upper bound and selects the adjacent stored interval. Arithmetic
follows the documented subtraction, division, multiplication and addition order.
An exact interior knot still executes interpolation rather than returning its
stored output directly.

The FEV1 packet freezes 5,424 requests with every body field, query and accepted
result or rejection. It includes native-file queries, every finite constant
pattern in the existing bit corpus, all 256 extra-array flags, count-zero and
single-point cases, nonuniform tables through 1,024 points, deterministic random
tables, signed zeros and subnormals. Poison patterns in unused fields demonstrate
the selected scope. Unsupported ordinals, nonfinite queries or selected values,
duplicate coordinates and descending coordinates account for 240 rejections.

The 5,184 accepted requests include fifteen nonfinite arithmetic results.
Finite ordered inputs can overflow during interpolation: opposing large outputs
produce infinity, and an overflowing slope at an exact interior knot can produce
NaN. A coordinate difference can overflow and produce a finite result with lost
accuracy. These are documented arithmetic outcomes; accepting only finite results
would implement a different profile.

An independent oracle uses exact rational barycentric interpolation and a
separate interval search. Affine tables also have closed-form expected values.
The packet records the correctly rounded exact-real value and an upward-rounded
measurement of the actual absolute error when the documented result is finite.
These measurements describe each case; they do not certify an error bound for
arbitrary future requests. Arithmetic NaNs are compared by classification because
their payload and sign are not portable across targets. All finite and infinity
results, including signed zero, retain exact expected bits.

This is an owned evaluation baseline. It adds no production numeric API yet.
Rust reconstructs every complete body and compares the existing typed body views,
then checks guards, branches, exact constant/clamp results and affine closed forms.
Python independently reconstructs all fields and verifies packet reproducibility
in a temporary directory. Native application execution, engineering units, other
accessors and model evaluation remain outside this evidence.
