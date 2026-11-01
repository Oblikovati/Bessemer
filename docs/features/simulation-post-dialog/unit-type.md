# Numeric unit-type model

`SimulationUnitTypeInputs::unit_type_code()` implements C-881's numeric provider
policy over signed32 analysis, plot and component controls. It preserves unsigned
component bounds, early plot overrides, analysis-family differences and plot71's
initial fallback, including analysis7's subsequent override. Code9999 is a known
returned value; its physical or sentinel meaning is not assigned.

The remaining inputs are optional explicit loaded BE8, BEC and ActualFreqDispFlag.
Known BEC0 or BE8 different from2 proves the modifier false without inventing the
other word. A required unknown input returns a typed missing-context error. The
global gate opens for exactly1; arbitrary other nonzero values are preserved.

The parent projection reads stored BEC where framed and requires the caller to
supply BE8/global context. A similarly named archive reference does not establish
the loaded BE8 value. Callers with actual runtime state can construct all inputs
directly; no current machine registry or native global is queried by this model.

C-882 binds the raw global getter/setter and the distinct adjacent API-running
global. Virtual zero-fill describes image initialization, not per-file runtime
state. C-883 retains one registry caller that normalizes its converted integer
to0/1 before invoking the setter. That caller neither changes the setter's raw
word contract nor resolves invocation order, other writers or actual file state.
The retained conversion evidence is not a general registry, locale or atoi parser.

All224 original native inputs are retained:168 return numeric codes and56 still
require runtime context. The original168 conditional flag alternatives remain
separate what-if inputs. No unit names, native execution results or stored-byte
coverage are inferred from these computations.

Seven Rust tests, all-target Clippy and all 131 integrated reference checks passed.
Independent fixtures cover 202,840 signed/context tuples and all complete parent
projections under16 caller contexts. Eight workers matched serial output; two
fresh processes matched all 203,232 owned/native/alternative results against
Source byte for byte. [Verification](unit-operation-verification.json) records
the precise evidence, unresolved states and remaining publication gates.
