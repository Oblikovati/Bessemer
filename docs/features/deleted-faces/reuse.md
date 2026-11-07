# Reused framing and ownership contracts

The feature uses `body_features::base`. Native RTTI proves its body/atom ancestry,
so the existing finite atom-owner family includes this exact concrete class.
The separate data class retains its `su_CObject` ancestry and is not added to that
owner family.

Nullable arrays reuse `body_features::nullable_array`, with its signed presence
word and conditional direct body. Direct arrays and DWORDs reuse the existing
bounded model-list readers. These are generic `suObArray` containers; restricting
them to classes seen in four samples would reject valid persisted objects.

Fixtures reuse the independent body-feature wire, shared object identities,
nullable-array builder, complete base-surface reader and packed-count builder.
The surface adapter invokes the actual scoped source function, with the explicit
source root supplied once through the fixture family. Public document and edit
checks reuse the existing contract helpers. Earlier owned baselines remain
unchanged.
