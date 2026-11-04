### 8.1 Preview

Definition: stream `Preview`; one per document (1,115/1,115). `Config-<n>-Preview` and 11 of 22 `ModelBasedImages/ModelBasedImage<n>` streams use the same layout.
*Status: Verified · 1,115/1,115 · Evidence: F.2*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 0x00 | `size` | u32 | | Byte count of the rest of the stream. | V |
| 0x04 | `header` | BITMAPINFOHEADER | | [DIB]; `biSize` = 40, `biPlanes` = 1. Observed 8 bits per pixel with RLE8 compression (`biCompression` = 1). | V |
| 0x2C | `palette` | RGBQUAD[] | | `biClrUsed` entries, or 2^bpp when `biClrUsed` = 0. | V |
| +0 | `pixels` | bytes[biSizeImage] | | Bitmap data, compressed as `biCompression` says. | V |

