# Conditional PostStyle comparison

Feature166, specification14.483, evidenceC-863.

`CDspPostStyle::DiffOGLRender` checks loaded object state in native order and
returns1 on a reached difference, otherwise0. Its complete body spans
`0x180044020..0x180044221` (exclusive end):513bytes and107instructions.
The observed dependencies are:

| Matching control | Reached payload comparisons |
|---|---|
|1B8=1 |Color1BC |
|1B8=2 |Colors1BC and1CC |
|21C nonzero |Float22C,word230; color220 when230=0 |
|1EC nonzero |Word1F0 and color1F4 |
|204 nonzero |Word208 and color20C |

The method also delegates to the named legend comparator, compares words
28C/290/294/298 and color29C, and checks float2B8. Two scalar checks use
`UCOMISS` followed by`JNE`; seven sites call the named COGLColor inequality
operator. Color and floating-point policies must therefore remain distinct
from raw equality. Their existing independent models retain explicit SSE
assumptions; this evidence does not observe the application's FP environment.

An inactive payload remains stored even when this comparator skips it.
Its absence from a reached branch identifies neither a UI meaning nor every
application rendering dependency. Original wire words are also distinct from
loaded values after the legacy1F0 migration or explicit reader resets.

`CopyPreferences` spans`0x180043f40..0x180043fe3`:163bytes and38instructions,
including both returns. It calls its legend helper before inspecting the
runtime pointer at2D0. Present plot types70 or73 clear234/238 through one64-bit
zero store; other contexts use the final supplied integer arguments. This
cross-checks the existing local raw-preference projection and adds no second
copy implementation. The delegated legend work remains separate.

C863 retains284 original identities:276 accepted interpretations and eight
rejected candidates. Their2,136 control-word observations account for8,544
already parsed bytes. Raw patterns and absolute spans remain authoritative;
signed values are diagnostic views. Smaller later-field counts reflect
versioned presence and supply no defaults. Owned replay retains425 complete
parents, including20 raw-class contexts without these canonical fields.

The evidence checks full GNU/Ghidra bodies, exact export and import identities,
both decompilation hashes, the seven color IAT calls and the existing complete
legend/color comparison bodies. No target code executes, no global byte credit
is added and no native rejection is promoted into an accepted observation.
A complete typed style comparator and its independent operation oracle remain
separate work.

Targeted freeze passed12 checks in8.640seconds. Strict replay passed the same12
checks in7.626seconds; both processes exited successfully after snapshot cleanup.
The [verification record](style-comparison-verification.json) retains exact pins,
complete-body captures and original observation counts. Full integrated replay,
the typed operation, publication gates and remaining feature166 work are pending.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
