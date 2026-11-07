#### 14.88.2 Packed vector and index blocks

*Status: Native widths, component order and interpolation established; eight-bit vectors are native/synthetic only · Evidence: C-464*

Let **V3(N)** denote N three-component vectors. Its wire order is: one-byte mode; **three one-byte constant-axis flags** in X/Y/Z order; one f32 pattern for each constant axis; for modes `b`/`c`, two f32 endpoint patterns for each varying axis; then the packed values. Constant values precede all endpoint pairs. Packed values are **per vector**, in X/Y/Z order with constant axes omitted. With A varying axes, there are exactly **N×A** stored scalar values. Even N=0 still consumes the mode, flags, constants and any endpoint fields.

| Mode byte | Packed scalar width | Native interpretation |
|---|---|---|
| `a` (0x61) | 4 | Direct f32 patterns; no endpoint pairs |
| `b` (0x62) | 2 | Unsigned u16 codes with endpoint interpolation |
| `c` (0x63) | 1 | Unsigned u8 codes with endpoint interpolation |

**V2(N)** has a mode byte and N two-component vectors, with **no constant-axis flags or constant-value fields**. Modes `b`/`c` store four endpoint patterns (two per component) before their 2N packed codes. Mode `a` stores 2N direct f32 patterns. Array readers **0x18000B5F0 / 0x18000B2C0** and reconstruction helpers **0x18000BEE0 / 0x18000C430 / 0x18000C7D0** establish these counts and component order.

Exact leaves **0x18000BC80** and **0x18000BCB0** convert u8/u16 code q using **t=q/Q**, then **(1−t)×A+t×B**, with Q=**255** or **65535** and the stored endpoint pair A/B. Native scalar single-precision instructions perform the operations; endpoint ordering, finiteness, normal-vector length and floating-environment behavior are not constrained here. The decoder retains every endpoint pattern and packed code; it does not emulate runtime floating reconstruction or replace original codes with rounded Python floats. The scoped constant-axis flags are 0/1; noncanonical bytes are rejected rather than assigned unverified runtime behavior.

Let **Ix(N,D)** denote N unsigned index/length values, where D is a caller-supplied domain parameter. **No count, width tag or D is stored inside this block.** Reader **0x180005180** uses:

| D | Stored width per value |
|---|---|
| 0..255 | 1 byte |
| 256..65535 | 2 bytes |
| 65536..0xFFFFFFFF | 4 bytes |

The threshold uses D itself, not D−1 or the largest observed code. Values expand without delta coding. The decoder preserves values outside D; it does not attribute an additional rejection rule to the native helper. For coordinate/normal lists D is their stored count; for strip-length lists it is the separate stored length-domain word.

### 14.97 Composer packed-vector values under an explicit binary32 arithmetic profile

*Status: Scalar instruction order and full installed vector-array reconstruction verified under declared nearest-even/gradual profile; actual native control state and later geometry processing remain open · Evidence: C-473*

This continues the vector framing in 14.88 with reconstructed component patterns. The raw reader remains unchanged and retains every endpoint, constant and packed code. `solidworks/composervectors.py` provides `reconstruct(block, version=7.9, fp_profile="nearest-even-gradual")`, returning a little-endian binary32 component array, count and dimension metadata, and original source span. The profile argument is mandatory. It describes numeric evaluation of the stored array before later operations such as geode vertex snapping, transformations or rendering; it is not a complete loaded-mesh reconstruction.

**Exact native scalar sequence.** `DecodeFloat8` at **0x18000BC80** and `DecodeFloat16` at **0x18000BCB0** in the hash-pinned `litinteropsmg.dll` from C-464 are complete ten-instruction leaves. They zero-extend the code, convert it to float, divide by the respective binary32 constant **255.0** or **65535.0**, subtract that result from binary32 **1.0**, perform the two separate scalar multiplications and add them. There is no fused multiply-add in these leaves. With R denoting binary32 rounding under the declared profile, A/B the stored endpoint values and q the unsigned code, the operations are:

1. `t = R(q / Q)`, with Q=255 or 65535.
2. `u = R(1.0 − t)`.
3. `right = R(t × B)`.
4. `left = R(u × A)`.
5. `result = R(left + right)`.

The integer-to-binary32 conversion is exact for every supported code. Intermediate rounding is significant: for an eight-bit code **65** with endpoints **−1.0 / +1.0**, the modeled native sequence yields **0xBEFAFAF9**, while evaluating the interpolation as one binary64 expression and rounding only once yields **0xBEFAFAFB**. A reimplementation must not replace the sequence with `A + t×(B−A)`, one final rounding, or a fused operation while claiming the same component patterns.

**Component order and direct copies.** `GetTabVector32` at **0x18000BEE0**, `GetTabVector16` at **0x18000C430** and `GetTabVector8` at **0x18000C7D0** corroborate the three-dimensional expansion. Output order is X,Y,Z for each vector. A canonical constant-axis flag selects its raw stored binary32 constant; that axis consumes no packed scalar. Other axes consume packed values in their existing interleaved order. Mode `a` copies each four-byte pattern without arithmetic. For V2, `ReadTabSglVec2f` at **0x18000B2C0** consumes interleaved X/Y codes, with separate endpoint pairs and no constant-axis flags. `ReadTabSglVec3f` at **0x18000B5F0** supplies the version-7.9 packing context and constants. Scalar call order within a row does not change these output positions; exception/trap effects are outside the model.

The reconstructed mode-`a` and constant-axis lanes preserve every original word, including signed zero, infinities and signaling/quiet NaN payloads, without converting those copied lanes through arithmetic. Quantized varying axes currently require **finite endpoints** when any output is requested. A zero-count block performs no component arithmetic, so its output is empty even if retained endpoint words are nonfinite. Unsupported nonfinite arithmetic does not invalidate or discard the original raw-reader result. Synthetic finite subnormal, signed-zero and large-endpoint cases exercise gradual underflow and overflow behavior; arithmetic results can include infinity when finite operands overflow.

**Control-state boundary.** The profile uses round-to-nearest with ties to even, gradual underflow, and separate binary32 results for each operation. The implementation performs those explicit binary32 steps using the Python host's float packing and numeric operations; the independent checks below validate the inspected host's results. The installed application's MXCSR rounding mode, flush-to-zero, denormals-are-zero and trap settings have not been measured. Alternative native control states, exception flags and nonfinite endpoint arithmetic are not reproduced. `native_control_state_verified` therefore remains false, as does complete-document status. The core scene reader's earlier `runtime_float_reconstruction_reproduced=False` is retained: this helper supplies a declared-profile interpretation, not proof of the running application's final values.

**Independent numeric checks.** The test/evidence oracle decodes finite words as exact integer ratios and rounds with integer quotient/remainder and tie-to-even rules. It does not use the production helper's binary32 packing path to choose rounded words. It verifies all **65,792 quantization weights** (all 256 byte codes and all 65,536 word codes), **1,869 synthetic component cases** spanning negative, adjacent, subnormal, signed-zero and extreme finite endpoints, and **32,286 component samples** from the first, middle and last rows of installed quantized blocks. This validates those domains and observed samples; it is not an exhaustive check of every possible binary32 endpoint pair or native control state.

Every installed vector block reconstructs under the declared profile:

| Measurement | Installed result |
|---|---:|
| Vector blocks | 4,367 |
| Mode `a` / `b` / `c` blocks | 578 / 3,789 / 0 |
| Vectors, including coordinate/normal/texture-coordinate arrays | 5,583,335 |
| Output components | 16,562,558 |
| Reconstructed binary32 array bytes | 66,250,232 |
| Already framed stored vector spans, including their headers | 35,263,999 |
| Nonfinite / subnormal output components under this profile | 0 / 0 |

The output expansion does not increase stored bytes or imply that every vector represents a position. No installed mode-`c` block or nonfinite/subnormal output was observed; those output cases have native/synthetic coverage. Units, coordinate frames, normal interpretation, quantization error against unavailable original geometry and later vertex snapping remain separate questions.

`composer_vector_values_evidence.py` pins **two exact scalar leaves and five complete reader/expansion ranges**, constant bytes, source/member hashes, independent comparisons and per-block reconstructed-array hashes in `composer-vector-values-native-evidence.json` and `composer-vector-values-corpus.json`. `composer-vector-values-synthetic.json` retains the synthetic arithmetic results. Seven focused tests cover the exhaustive weight domain, independent component rounding, the single-expression counterexample, raw nonfinite copies, constant lanes, V2/3 order, zero-count and finite-arithmetic limits, context/shape guards and every installed block. No global stored-byte coverage credit is added by this value reconstruction. Full project semantics and all installed-format schemas remain incomplete.

All **1,517 regression tests pass** (180.080 seconds). The native/corpus verifier passes; specification lint reports zero findings (659 headings, 96 anchors).

Evidence: C-473.
