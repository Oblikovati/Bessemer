# Weld symbols

Feature #54 reference-fixture preparation covers `moWeldSymbol_c`,
`moCosmeticWeldBeadSymbol_c`, `moSFSymbol_c` and
`moCosmeticWeldBeadAttachPoint_c` from 11.345 / C-387 and 11.346 / C-388.
Native constructors prove three directly owned dimension-text bodies in a weld
and one in a surface-finish symbol; none adds an outer tag or array count.

The independent fixture packet contains 216 complete graphs, 57,278 primitive
observations and 28 owned streams across nine versions and six identity styles.
They include populated nested point/curve text, actual shared fonts, shared finish
symbols, subtype parents, empty direct texts and strict surface-finish version
gates. The runtime implementation is recorded in the following behavior commit;
this preparation changes no production parser. Humanizer is unavailable;
maintainer prose review remains open.
