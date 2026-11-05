# Flow legacy block trees — issue #241

Specification 14.40 defines record 400's legacy eight-child wire grammar for
versions 0–3 and the inactive prefix of later delegated versions. The FBT1 fixture
pins that grammar before a typed bounded reader and editor are introduced.

The 81 cases contain one native CPT zlib payload and 80 owned raw/zlib cases.
The native payload has 340 stored bytes and 8,908 decoded bytes. Its 792 present
roots contain 4,440 wire nodes, 456 splits and 3,984 leaves. The complete parsed
projection matches the source corpus; source and payload hashes are verified
before extraction. Neither other native carrier contains record 400.

Versions zero and one require explicit legacy bounds and have implicit present
roots. Versions two and three store inclusive bounds and root presence bytes.
Root order advances the first index fastest, then second, then third. Exactly
marker 255 means split, and every split has eight children. The discarded child
prefixes in versions two/three are raw bytes, even when zero; they do not make
children absent. Version two's extra leaf words remain raw u32 patterns.

Owned cases cover leaves, splits, nested forests, mixed present/absent roots,
reversed ranges, signed bounds, raw marker/presence/prefix values and depth 255/256.
The depth-256 case retains wire depth 256 alongside the native level byte zero.
Inactive delegated cases include a mathematical grid count of 2^96 without
materializing roots. Missing roots for an inactive body differ from an active
legacy forest with zero positions. Active delegated layouts remain unsupported.

Python freezes complete root intervals and flat wire-preorder node metadata from
the sibling `flowblocktrees.py`. Rust independently rebuilds payloads with an
explicit task stack, validates first-index-fast root positions, exact intervals,
native-level wrapping and the full eight-child forest identities. All discarded
bytes and words participate in reconstruction. Regeneration tests use temporary
directories and retain the frozen golden bytes.

Run `cargo test --test flow_block_tree_fixtures` and
`python -m unittest discover -s docs/features/flow-block-trees -p test_trees.py`.
The typed reader/editor remain next work, with bounded decoded bytes, roots,
nodes and depth checked before publication. Caller bounds and compression must
stay explicit. Native parallel block-vector order, validation against global
base-grid dimensions, delegated active layouts and application acceptance remain
separate requirements. Wire node/leaf counts alone do not identify record 406's
runtime context or record 433's expected count.

Decoded/stored tree measures use different byte bases and supply no additional
global semantic coverage claim. This fixture baseline adds no production API.
The maintainer humanizer pass remains for review because that skill is unavailable.
