# Retained source: clause 8

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`, Edition 0.121 · 2026-10-24 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py), working tree at `dd0537b3a142d8dda00ef846891efc736a6aac46`, read 2026-10-24.

## 8 Preview streams

<a id="st-preview-dib"></a>
### 8.1 Preview

Definition: stream `Preview`; one per document (1,115/1,115). `Config-<n>-Preview` and 11 of 22 `ModelBasedImages/ModelBasedImage<n>` streams use the same layout.
*Status: Verified · 1,115/1,115 · Evidence: F.2*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 0x00 | `size` | u32 | | Byte count of the rest of the stream. | V |
| 0x04 | `header` | BITMAPINFOHEADER | | [DIB]; `biSize` = 40, `biPlanes` = 1. Observed 8 bits per pixel with RLE8 compression (`biCompression` = 1). | V |
| 0x2C | `palette` | RGBQUAD[] | | `biClrUsed` entries, or 2^bpp when `biClrUsed` = 0. | V |
| +0 | `pixels` | bytes[biSizeImage] | | Bitmap data, compressed as `biCompression` says. | V |

<a id="st-preview-png"></a>
### 8.2 PreviewPNG

Definition: stream `PreviewPNG`; one per document (1,115/1,115). `Images/Sheet_<n>` (drawings), `Config-<n>-PreviewPNG` and the other 11 `ModelBasedImages/ModelBasedImage<n>` streams use the same layout.
*Status: Verified · 1,115/1,115 · Evidence: F.2*

The stream data is a [PNG] datastream, from the signature to the end of the IEND chunk.

<a id="cl-streams"></a>
