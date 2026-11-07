# Shared text-data prefix

Issue #54. Section 11.358 / C-400 identifies the existing character reader's
first 36 bytes as the common text-data base. The final character u16 at +40
belongs to its subclass. This extraction keeps both the stored field paths
and byte order unchanged before adding border and symbol callers.

The baseline inventory records all existing display-note fixture hashes at
commit 556969d. The seven existing note/text tests must pass without changing
those fixtures. No new class is installed by this structural change. The
maintainer humanizer skill was unavailable; prose review remains with review.
