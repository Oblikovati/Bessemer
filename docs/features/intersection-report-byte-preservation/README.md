# Intersection report byte preservation

Bug [#186](https://github.com/Oblikovati/Bessemer/issues/186) affected the
unpublished C-3307, C-3308 and C-3310 reports. Git's default text rules converted
their CRLF line endings to LF, so committed objects differed from their
published hashes.

Scoped `-text` attributes now preserve both intersection report directories.
The regression record retains the previous Git hashes and checks all three
corrected objects against untouched Source bytes and the existing expected
publication hashes. Their only content difference was line endings. Runtime
code and expected reader results were unchanged. Humanizer is unavailable;
prose review remains pending.
