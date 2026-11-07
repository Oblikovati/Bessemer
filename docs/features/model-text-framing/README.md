# Physical CString field framing (#200)

Model-field equality previously delegated to semantic CString equality, which
compares text and encoding but omits retained count-prefix width. A request that
changed only Byte/Word/Dword framing could therefore disappear as an unchanged
edit and omit its derived field history.

Physical model fields now compare the CString representation as well. The public
`ArchiveString` semantic comparison remains unchanged. This makes existing model
archive no-op detection and field history recognize a framing-only replacement
without changing the reader, writer, string contents or admission policy.

The focused regression covers all 18 transitions among three prefix widths in
Latin1 and UTF16, including six exact no-ops. It verifies requested whole-stream
bytes, completion, changed/unchanged counts, source/result parent identities,
complete document reopen, exact reversal and input isolation. Both regression
tests failed before the fix and passed afterward. The unchanged original Source
reader accepted all six literal frames through EOF under a frozen import/content
guard. No native application or geometry acceptance is claimed.

This is comparison behavior profile 2 for unreleased physical model fields.
Profile 1 is retained by the pre-fix source revision and red test log identified
in `focused-verification.json`; no on-disk migration or package version change
occurs. Exact prior input bytes remain accepted and unchanged when no framing
replacement is requested. Full repository and exact-head remote checks remain
required before integration.
