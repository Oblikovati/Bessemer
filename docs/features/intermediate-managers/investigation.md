# Version-11000 alignment investigation

The first complete native-manager test reached the derived configuration maps
at the wrong byte offset. Source placed the directly owned object at 16,988;
Bessemer initially placed it at 17,000. A temporary field trace and profiling
of the actual Source reader established the boundary without changing Source
reader functions.

The base configuration stores raw +848 only above version 0x2F72. At 11000,
the extra four-byte read shifted the following maps. A later map then consumed
what was actually the owned object's first word as its count and advanced eight
more bytes. The version gate removes that read; reviewed 18000/19000 behavior
is unchanged. The independent fixture's primitive partition remains fixed.

The native fixture also contains exactly 350 fresh objects in 370 archive slots.
Its tests use that independently measured object budget and reject a budget of
349. Production limits were not changed. Temporary diagnostic test code and
runner files were removed after the cause was established. The retained logs
are local build artifacts; this note records the finding.

The maintainer humanizer skill was unavailable; prose review remains with review.
