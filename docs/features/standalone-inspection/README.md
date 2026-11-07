# Standalone Inspection encrypted XML envelopes

Feature #92 implements the complete encrypted carriers documented in clauses
14.8 and 14.15. `bessemer::inspection::InspectionEnvelope` reads the recovered
current-then-historical loader order, returns exact UTF-8 XML, retains the detected
profile and reconstructs ciphertext deterministically. `with_xml` replaces the
complete XML on a new immutable snapshot while retaining that profile.

The current profile uses the documented AES-128 CBC key, zero IV and PKCS#7
padding. The historical Framework profile uses two key words and ten Rijndael
rounds; Framework does not expand the ninth supplied key byte. Padding to an
ordinary AES key would produce a different format. Both profiles use a scoped
fixed-profile block codec and expose no caller-supplied key API. No dependency
was added.

Input must be a positive multiple of sixteen bytes within the explicit budget.
Padding, strict UTF-8, XML structure, namespaces, node/attribute/depth budgets and
the standalone root are checked before publication. The existing bounded XML
parser provides syntax validation. Only the root is required to have expanded
name `{http://www.extensiblecad.com}SOLIDWORKSINSPECTION`; observed unqualified
children retain their own identities. DTD/entity declarations are rejected.

Standalone envelopes and the embedded drawing `ThirdPty/InspectionXpert` XML
remain separate carriers. The unencrypted native save branch, arbitrary cipher
profiles, native application reload/save acceptance and complete project value
semantics remain outside this implementation. Successful framing and XML syntax
checks do not establish authenticity or full schema conformance.

`freeze_native.ps1` runs on .NET Framework 4.x and verifies all 42 installed
templates against the retained stored/plaintext hashes. They contain two distinct
current payloads. It also verifies the historical tutorial, reconstructs each
distinct ciphertext and generates independent XML-replacement goldens. The
original 43 files contain 2,657,536 stored bytes and 2,657,237 XML bytes. Cargo
tests use three distinct owned fixtures and the complete 42-template inventory,
without requiring the installed application or user-home writes.

The cipher tests match the independent current Framework vector and all nine
historical Framework vectors in both directions. They check substitution
permutations, inverse rounds, every padding width, malformed padding, length
overflow and byte budgets. Public tests check all three native payloads, exact
reconstruction, independent edit goldens, every small-envelope prefix, rejected
edits, namespace aliases/BOM, unqualified children, source isolation and parallel
deterministic writes. Both published types are checked for `Send` and `Sync`.

An immutable copy of the independent Python historical reader also checks all
nine vectors and the complete tutorial plaintext. The bundled Python has no
PyCryptodome, so current-file oracle generation uses .NET Framework directly;
no reader was replaced or mocked. JSON reports retain the source/evidence hashes,
Framework version, complete inventory and fixture identities.

The humanizer skill was unavailable; its prose review is left to the maintainer.

Local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: formatting, Clippy,
build, 1,002 library tests, all integration tests and 277 doctests. Three external
fixture tests remain intentionally ignored by the default gate. Two fresh public
test processes reconstructed all three distinct native ciphertexts and plaintexts
exactly; eight concurrent writers also matched the current native fixture.
