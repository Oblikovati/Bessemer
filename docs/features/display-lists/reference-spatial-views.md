# 3D-view serializer reference snapshot

Copied from the approved evolving ../inventor-interface working tree, based on
commit dd0537b3a142d8dda00ef846891efc736a6aac46, on 2026-10-25.
The complete original dl_doc.py SHA-256 at capture was
`6342a5285d5261d7ef5d07430d021aeea5aae878b0e61de9c7488b187f2aa8bc`.

```python
def read_3DViewTransform(a):
    """uiModelDoc_c::Serialize3DViewTransform (slduiu 0x4976e8f0), reading side (discarded)."""
    if a.version <= 0x1C18:
        return None
    n = _count(a, a.i32(), '3D view')
    views = []
    for _ in range(n):
        v = {'view_name': a.cstring()}     # getViewName + separator + configuration getName
        if a.version > 0x46C9:
            v['unknown_active'] = _unk(a, a.i32)   # 3dViewMgr +0x14 == view +0x358
        v['model_to_view_xform'] = read_mgXform(a)
        v['pan_zoom_xform'] = read_mgXform(a)
        v['unknown_string'] = _unk(a, a.cstring)
        if a.version > 0x1C1E:
            # Win32 BITMAP via GetObjectW: bmWidth, bmHeight, bmWidthBytes, bmPlanes, bmBitsPixel,
            # then GetBitmapBits (bmHeight * bmWidthBytes bytes) -- the preview bitmap.
            w, h, wb = a.i32(), a.i32(), a.i32()
            planes, bpp = _u16(a), _u16(a)
            nb = h * wb
            if nb < 0 or nb > 64 << 20:
                raise ArchiveError('3D view bitmap size %d at %#x' % (nb, a.p))
            v['preview_bitmap'] = dict(width=w, height=h, width_bytes=wb, planes=planes,
                                       bits_pixel=bpp, bits=a.raw(nb))
        if a.version > 0x1C42:
            v['unknown_ids_string'] = _unk(a, a.cstring)   # itoa list of view +0x420 ints
        cnt = _count(a, a.i32(), 'section plane')
        planes = []
        for _ in range(cnt):
            p = dict(root_point=a.point(), normal=a.point())
            s = a.p
            p['unknown_d'] = a.f64()
            p['unknown_i0'] = a.i32()
            if a.version > 0x2012:
                p['unknown_i1'] = a.i32()
                p['unknown_i2'] = a.i32()
                p['unknown_u'] = a.u32()
            a.unk(s)
            planes.append(p)
        v['section_planes'] = planes
        if cnt > 0 and a.version > 0x2012:
            v['unknown_after_planes'] = _unk(a, a.i32)
        if a.version > 0x1FB5:
            v['unknown_468'] = _unk(a, a.i32)                      # view +0x468
            v['unknown_bitset'] = (_unk(a, a.i32), _unk(a, a.i32))  # view +0x450 utBitSet
        if a.version > 0x3331:
            v['unknown_text_string'] = _unk(a, a.cstring)
        views.append(v)
    return views
```
