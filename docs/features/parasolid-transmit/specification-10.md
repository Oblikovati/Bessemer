## 10 Parasolid transmit files

<a id="st-xt-overview"></a>
### 10.1 Overview

*Status: Verified · 2,869 transmit files in 1,115 samples parse to their last byte · Evidence: [C-9], F.3*

Every chunk of a body stream (9.3) inflates to a Parasolid transmit file: the model data of the Parasolid kernel that SOLIDWORKS uses. A partition stream holds a *partition* file (the bodies) and then a *deltas* file (the rollback history of the partition). The format is the kernel's own "neutral binary" transmit format. It is self-describing relative to a base schema: each file names the schema it was written with, and describes how each node type it uses differs from a base schema that the reader holds.

The base schemas are text files that Parasolid installs; SOLIDWORKS installs 100 of them as `SOLIDWORKS\data\pschema\sch_<n>.s_t`. Every sample file uses base schema 13006. A reader MUST have that schema file, or a table built from it, to read the files.

<a id="st-xt-header"></a>
### 10.2 File header

Definition: start of every transmit file.
*Status: Verified · 2,869/2,869 · Evidence: [C-9]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `format` | bytes[2] or bytes[1] | | `PS`: neutral binary, all values big-endian, in 2,869/2,869 samples. `B`: bare binary, values little-endian, written by the kernel for the machine format [C-9]. | V |
| 2 | `title` | String32 | | ": TRANSMIT FILE (partition) created by modeller version <m>" or ": TRANSMIT FILE (deltas) created by modeller version <m>". The word in parentheses selects the file kind (10.7, 10.8). | V |
| 3 | `schema_key` | String32 | | `SCH_<modeller>_<writer schema>_<base schema>`, e.g. `SCH_3701250_37102_13006`. | V |
| 4 | `n_types` | u16 | | Node-type table size of the writer schema: the first number of the writer schema file's count line (239 for 37102). | V |
| 5 | `user_field_size` | u32 | | Size of user fields on each node. 0 in 2,869/2,869; a reader that meets another value cannot rely on this clause (OI-011). | C 2869/2869 |

String32 is a u32 byte count followed by that many bytes of text. A reader MUST read the format byte or bytes first and then all values in the byte order they select [R-08].

<a id="st-xt-schema"></a>
### 10.3 Schemas and field types

*Status: Verified · Evidence: base schema files, F.3*

A schema file lists node types: `<code> <NAME>; <description>; <transmit> <n_fields> <variable>`, then one line per field: `<name>; <type>; <transmit> <pointer class> <elements>`. A field with `transmit` = 0 is not written. `elements` is 0 for a scalar, 1 for the variable-length array that ends a variable-length node, and n > 1 for a fixed array of n. The *base layout* of a node type is the list of its transmitted fields in schema order.

| Type | Bytes | Value |
|---|---|---|
| `d` | 4 | integer (i32) |
| `p` | 2 or 4 | pointer: a node index (10.4) |
| `u` | 1 | unsigned byte |
| `c` | 1 | character |
| `l` | 1 | logical: 0 false, 1 true |
| `n` | 2 | short integer |
| `w` | 2 | UTF-16 code unit |
| `f` | 8 | double |
| `i` | 16 | interval: 2 doubles |
| `v` | 24 | vector: 3 doubles |
| `h` | 24 | hvec in partitions: 3 doubles; 88 in delta records (10.8) |
| `b` | 48 | box: 6 doubles; not observed in transmitted layouts |

*Table 10-1 — Field types*

Types `q` and `t` occur only in fields that are not transmitted.

<a id="st-xt-index"></a>
### 10.4 Node indices and pointers

*Status: Verified · kernel file with 45,335 nodes · Evidence: F.3*

A node index i and a pointer to node i are both stored as v = i + 1; v = 1 is the null pointer. When v ≤ 0x7FFF it is one u16. Otherwise it is two u16: first −((v & 0x7FFF) + 1), which reads as a negative short, then v >> 15.

EXAMPLE  Pointer to node 32,770: v = 32,771, stored `FF FC 00 01`.

<a id="st-xt-record"></a>
### 10.5 Node record

Definition: one per node, in the record lists of 10.7 and 10.8.
*Status: Verified · 2,396,293 records · Evidence: F.3*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `type` | u16 | | Node type code; 1 is the terminator of a record list, followed only by an index (10.4). | V |
| 2 | `layout` | Layout | first record of this type in the file | 10.6. | V |
| 3 | `length` | u32 | node type has a variable-length array | Element count of that array. | V |
| 4 | `index` | Index | | Node index (10.4); pointers in other records refer to it. | V |
| 5 | `fields` | per layout | | The node's fields in layout order, each per Table 10-1; an array field repeats its type `elements` or `length` times. | V |

<a id="st-xt-layout"></a>
### 10.6 Layout descriptor

*Status: Verified · Evidence: F.3*

The first record of each node type carries a layout descriptor. It starts with a u8 `n`.

- `n` = 0xFF: the type's layout is its base layout.
- Otherwise, if the base schema has the type, `n` is the field count of the writer's layout and edit codes follow, each one ASCII byte, until `Z`. A reader MUST build the writer's layout by walking the base layout with them [R-09]: `C` copies the next base field, `D` skips the next base field, `I` and `A` add the field definition that follows, and `Z` ends the edits; base fields not yet consumed are dropped. The result has `n` fields.
- Otherwise the type is new to the base schema and is defined in full: name (String8), description (String8), `n` field definitions, then a u8 that is 1 when the last field is a variable-length array.

A field definition is: name (String8), pointer class (u16; 0 when not a pointer), `elements` + 1 (u16: 1 scalar, 2 variable-length array, n + 1 fixed array of n), and when the class is 0 the type letter as String8 (Table 10-1). String8 is a u8 byte count and the bytes.

EXAMPLE  SCH_3701250_37102 files describe BODY (type 12) with 36 fields: `CCCI lattice … A mesh_offset_data Z`. Type 204 INTERSECTION_DATA, which base schema 13006 lacks, is defined in full in 235 files.

<a id="st-xt-partition"></a>
### 10.7 Partition file

*Status: Verified · 1,851/1,851 · Evidence: F.3*

After the header, a partition file is one record list: node records until the terminator (type 1 and an index). Nothing follows. The records hold the partition's bodies, their topology (shells, faces, loops, edges, vertices) and geometry, attributes and lists; their node types and fields are those of the base schema as edited by the file.

<a id="st-xt-deltas"></a>
### 10.8 Deltas file

*Status: Typed · 1,018/1,018 parse to their last byte; meaning of two items open (OI-012, OI-013) · Evidence: F.3*

A deltas file records the partition's history between pmarks. After the header it is a sequence of sections; the last section stops after its record list when the file ends.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `records` | record list | | Records until a terminator. PARTITION (3) and PMARK (4) records are written as in 10.5. All other records are *delta records*: every pointer is followed by a u8 flag, and each hvec element is 88 bytes (11 doubles). | V |
| 2 | `new_nodes` | (Index, u16)[] | not at file end | (node index, node type) pairs until a null index. | V |
| 3 | `count` | u32 | not at file end | Equal to the number of records in field 4 in 3,263 of 7,150 sample sections and in all kernel output; larger in the other 3,887, never smaller. Meaning OI-012. Readers MUST NOT use it to count field 4. | S 3263/7150 |
| 4 | `modified` | record list | not at file end | Delta records until a terminator. | V |

A pointer flag is 1 in 1,024,495 of 1,086,500 sampled delta pointers, and 0 only on non-null pointers (OI-012). The 11 doubles of a delta hvec are, as the kernel's text form prints them, a position (the 3 doubles a partition stores), one parameter, two 3-vectors and one scalar that is often undefined (OI-013).

EXAMPLE  A history of one block created between two pmarks gives two sections: PARTITION and PMARK, no delta records, 87 new nodes, one modified WORLD record; then two PMARK records, no new nodes, one modified WORLD record.
