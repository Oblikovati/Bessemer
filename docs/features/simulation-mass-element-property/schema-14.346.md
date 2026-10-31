### 14.346 Mass-element child Property body and separate Material payload

*Status: both retained child Property streams complete; sibling Material grammar and physical field meanings remain open · Evidence: C-722*

The MassElementManager child loader from 14.345 constructs each object through **0x1810C9F40** and invokes storage slot **+0x260**. The constructor installs primary table **0x18202FB80**, whose slot **0x18202FDE0** selects **0x1810CE5D0**. This method opens separate **Property** and **Material** streams. C-722 verifies their installed literal bytes, constructor/storage function-span hashes, serialization slot and version constants against the installed module hash.

**One body, no collection count.** Child `Property` first serializes common load base **0x180B754B0** on the object's **+8 subobject**. Unlike an LBC collection, this stream contains one base body directly and does not prepend a record-count word. The bounded parser retains the common reference/archive dependencies and historical gates; treating its first word as a collection count would misframe the body.

After the base, the following fields occur in order. Offsets in this table refer to the **outer mass-element object**, unlike the +8-subobject-relative LBC tail offsets in 14.338–14.341:

| Order | Stored fields |
|---|---|
| 1 | Three signed four-byte words **+0x440/+0x420/+0x424** |
| 2 | Two persisted strings **+0x430/+0x428** |
| 3 | Three signed four-byte words **+0x338/+0x33C/+0x38C** |
| 4 | Six doubles **+0x340..+0x368**, stride eight |
| 5 | Six signed four-byte words **+0x370..+0x384**, stride four |
| 6 | Sixteen doubles **+0x390..+0x408**, stride eight |
| 7 | At **v >= 890204**, signed selector **+0x2F0**; exactly **1** adds a signed four-byte count and that many pairs of four-byte words |
| 8 | Exactly six persisted strings from the runtime string array |
| 9 | Signed four-byte word **+0x410**, then six signed words **+0x320..+0x334**, stride four |

The selector threshold double at **0x18202FF50** is **890204**. The current writer emits the selector/pair form, whereas the older read branch omits it and initializes runtime state instead. Another double at **0x18202FF58** is **891107**; its comparison controls a runtime assignment at **+0x1A0**, not another persistence field. The parser does not consume bytes for that state repair or emulate the runtime tree's duplicate-pair insertion behavior. It retains every pair in wire order and rejects negative/over-limit counts.

**Material is independent.** After the Property body, the read path separately opens `Material`, constructs a material object through **0x1816D5780**, and delegates its stream to **0x1816E15A0**. A successful Property parse does not complete that material grammar. Neither header words nor mass/inertia-like arrays are assigned physical roles merely from the enclosing manager/class name.

**Native validation.** The exact retained child paths are **.../MassElementManager/MassElements/MassElement 1/Property** in two distinct documents sharing the basename `frequency_16.sldasm`. Fresh envelope/nested hash checks and explicit candidate **830036** complete both payloads: **1144 bytes** and **1132 bytes**, totaling **2276 bytes**. Each has one common subtype word **0**, with the derived continuation starting at stream offsets **768** and **766**, respectively. Both are below the selector gate, so native populated selector/pair tables are not claimed here.

The two sibling Material streams total **7096 bytes**. Their payload hashes and raw bytes are retained with `decoded=false`, without schema or coverage credit. This makes the remaining child-object dependency explicit rather than calling the entire manager complete.

`cosworks_mass_element_layout.py` implements the bounded single-body layout; `cosworks_mass_element_audit.py` retains native results and static bindings in `cosworks-mass-element-native-evidence.json`. Three tests pass for **890203/890204** and a modern version, selector equality/pair bounds, the absence of a collection header, every-byte truncation/trailing data and unsupported-kind rejection. No target code or native writer executes. Material bodies, physical meanings, historical/context variants and full installed-format coverage remain incomplete; global semantic byte credit stays zero.

Evidence: C-722.
