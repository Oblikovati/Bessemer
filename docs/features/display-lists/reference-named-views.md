# Named-view serializer reference snapshot

Copied from the approved evolving ../inventor-interface working tree, based on
commit dd0537b3a142d8dda00ef846891efc736a6aac46, on 2026-10-24.
The complete original dl_doc.py SHA-256 at capture was
`6342a5285d5261d7ef5d07430d021aeea5aae878b0e61de9c7488b187f2aa8bc`.

```python
def _read_section_planes_named(a, lo_a, lo_b):
    """Section planes as NamedView writes them: i32 count, per plane mgPoint (rootPoint),
    mgVector (normal), double, i32, [v > lo_a: i32, i32], [v > lo_b: u32]."""
    n = _count(a, a.i32(), 'section plane')
    planes = []
    for _ in range(n):
        p = dict(root_point=a.point(), normal=a.point())
        s = a.p
        p['unknown_d'] = a.f64()
        p['unknown_i0'] = a.i32()
        if a.version > lo_a:
            p['unknown_i1'] = a.i32()
            p['unknown_i2'] = a.i32()
        if a.version > lo_b:
            p['unknown_u'] = a.u32()
        a.unk(s)
        planes.append(p)
    return n, planes

def read_NamedViewTransform(a):
    """uiModelDoc_c::SerializeNamedViewTransform (slduiu 0x497750e0), reading side (all values are
    read into temporaries and discarded).  Names from the saver: moView_c::getViewName,
    getModelToViewXform, getPanZoomXform, mgPlane_c::rootPoint/normal, hasSectionSelections."""
    if a.version <= 0x1BE7:
        return None
    n = _count(a, a.i32(), 'named view')
    views = []
    for _ in range(n):
        v = dict(view_name=a.cstring(), model_to_view_xform=read_mgXform(a),
                 pan_zoom_xform=read_mgXform(a))
        v['unknown_string'] = _unk(a, a.cstring)   # Format(...) of doc +0x968/+0x970
        if a.version > 0x1BF4:
            cnt, v['section_planes'] = _read_section_planes_named(a, 0x2001, 0x2007)
            if cnt > 0 and a.version > 0x2003:
                v['has_section_selections'] = a.i32()
                if v['has_section_selections'] != 0:
                    v['unknown_sel0'] = _unk(a, a.i32)     # view +0x38c
                    v['is_part'] = a.i32()                 # isKindOf(moPart_c)
                    m = _count(a, a.i32(), 'section selection')
                    v['section_selections'] = [a.cstring() for _ in range(m)]
        if a.version > 0x200D:
            v['unknown_bitset'] = (_unk(a, a.i32), _unk(a, a.i32))  # doc +0xaa8 utBitSet words 0,1
        views.append(v)
    return views
```
