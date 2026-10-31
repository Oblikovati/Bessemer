# Binary thermal collections

Feature [#134](https://github.com/Oblikovati/Bessemer/issues/134) covers complete
Temperature, Convection, HeatFlux, HeatPower and Radiation collections. Their
reference containers require an explicit Simulation embedded archive context.
The ordinary part and assembly archive guards must retain their existing behavior.

## Reference baseline

`freeze_thermal.py` copies the complete approved SolidWorks Python source tree
and the external `reader/cfb.py` adapter before importing the actual readers.
It copies the existing document signature cache into that temporary directory;
the replay does not load or execute a SolidWorks DLL or COM object.

The actual document and CFB adapters recover all 68 C-716 streams, totaling
267820 bytes. Each outer envelope and thermal payload must match its retained
SHA-256. The actual additive archive callback reproduces all 68 successful and
seven failed candidate attempts, including complete spans, archive class counts,
body counts and transform traces. Successful attempts contain 242 embedded
archives and 19 component transform wrappers. Native payloads remain under
ignored `target/`; committed evidence records their identities.

The owned baseline adds 198 complete transform wrapper archives across all
historical wrapper thresholds and their neighbors. Both optional transforms
are populated; both direct suffix transforms cover matrix presence and the
base perspective, extended perspective and 5×5 matrix payloads. Literal double
patterns include signed zero, infinities and a NaN payload. The actual direct
codec independently verifies the boundary between the two suffix transforms.
Every truncated prefix of a populated matrix case must fail, along with
unsupported archive versions. Complete relevant schema clauses are frozen
separately so later documentation changes are visible in strict replay.

Install the reference-only import dependency and run the strict replay from
the repository root:

```powershell
python -m pip install --target target/reference-python --only-binary=:all: --no-deps --require-hashes -r docs/features/simulation-binary-thermal/reference-requirements.txt
python -B -X utf8 docs/features/simulation-binary-thermal/freeze_thermal.py
```

`--write` creates initial baselines for review. Normal verification compares
them without rewriting them. The source snapshot records every copied Python
file and imported reader origin. The external CFB adapter appears as `cfb.py`
in that snapshot and originates from `../inventor-interface/reader/cfb.py`.

## Remaining implementation

The replay establishes the independent reference baseline. Rust support remains
open: first complete embedded archive read, construction and isolated edit;
then the full shared load base and five derived thermal grammars. Inner archive
versions and outer Simulation versions need separate typed acceptance rules.
Legacy 4281 requires its documented separate reader, while modern embedded
archives accept 4307 through 19000. Unknown bodies and unsupported optional
reference states must produce explicit errors.

Acceptance also requires owned populated transforms, threshold and malformed
cases, complete field and history projections, source isolation, concurrency
checks, two fresh-process exports, the local PR gate and all five CI checks.
The reference replay alone establishes neither native application acceptance
nor complete SolidWorks schema parity.
