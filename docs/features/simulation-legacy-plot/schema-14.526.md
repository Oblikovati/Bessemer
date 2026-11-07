### 14.526 Legacy plot fixed wide-character buffers

*Status: five bounded wide-buffer conversions bound for versions >=700004; original suffix bytes retained — Evidence: C-908*

In the `POSTPLOTDLG` helper's >=700004 branch, native loader `181187EA0` reads four256-byte buffers and one128-byte buffer, then calls named `LPUSHORT2STRING` through the cwCommFuncs import. They occupy the fixed-layout positions previously labeled members+30,+13C,+23C,+33C (128 code units each) and+454 (64 code units). The exact700003 branch instead reads these blocks directly into object storage; the conversion claim here does not generalize that earlier branch.

The provider helper `cwCommFuncs:180005BF0..180005C9A` reads unsigned16-bit units and stops at the first zero unit or the supplied unit limit. At `180005C34/180005C38`, a zero unit branches out of the copy loop; `180005C4D/180005C4F` bounds the loop by the explicit count. It writes a terminal zero after the copied prefix before assigning the temporary CString. These bounds establish a supplementary text-prefix view without Unicode normalization or assumptions about well-formed surrogate pairs.

`cosworks_legacy_plot_record.py::fixed_wide_views` retains the original entire buffer, its prefix before the first zero, whether a terminator occurs, and the complete remainder starting at that terminator. The remainder shall not be discarded or labeled padding. Fresh native replay recovers240 such buffers from48 helper-compatible streams; all240 contain a terminator and all240 contain nonzero bytes after that first terminator. Prefix lengths vary from0 through63 code units in this retained set.

The framing totals remain60 named plot streams/81744 bytes:48 helper-compatible complete streams/65472 bytes, of which40/54560 bytes also satisfy the named-stream version gate. Twelve1356-byte streams remain truncated at the guarded final words; eight helper-compatible streams still have incompatible caller-version context. The new text views do not change those results or force an older saved version. Four plot tests pass, including embedded-NUL suffix preservation, unterminated synthetic buffers and the historical-branch guard.

Evidence: C-908.
