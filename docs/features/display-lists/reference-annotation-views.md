# Annotation-view serializer reference snapshot

Copied from the approved evolving ../inventor-interface working tree, based on
commit dd0537b3a142d8dda00ef846891efc736a6aac46, on 2026-10-25.
The complete original dl_doc.py SHA-256 at capture was
`6342a5285d5261d7ef5d07430d021aeea5aae878b0e61de9c7488b187f2aa8bc`.

```python
def read_AnnotationViewTransform(a):
    """uiModelDoc_c::SerializeAnnotationViewTransform (slduiu 0x497735e0), reading side.  The
    reader fills the visibility map (view_id -> is_shown) later passed to
    SetVisibilityForAnnotViewItems; everything else is discarded."""
    if a.version <= 0x1C39:
        return None
    n = _count(a, a.i32(), 'annotation view')
    views = []
    for _ in range(n):
        v = dict(view_name=a.cstring(), view_id=a.i32(),   # view +0x358
                 is_shown=a.i32(),                          # isAnnotationViewShown(id)
                 is_current=a.i32())                        # getCurrentViewId() == id
        if a.version > 0x1C40:
            v['unknown_flag'] = _unk(a, a.i32)              # id == 0xc6 || vfunc +0x1c98
        v['unknown_name2'] = _unk(a, a.cstring)             # view name + fixed suffix
        v['model_to_view_xform'] = read_mgXform(a)
        v['pan_zoom_xform'] = read_mgXform(a)
        v['unknown_string'] = _unk(a, a.cstring)
        cnt = _count(a, a.i32(), 'section plane')
        planes = []
        for _ in range(cnt):
            p = dict(root_point=a.point(), normal=a.point())
            s = a.p
            p['unknown_d'] = a.f64()
            p['unknown_i'] = a.i32()
            a.unk(s)
            planes.append(p)
        v['section_planes'] = planes
        views.append(v)
    out = dict(views=views)
    if a.version > 0x1FA8:
        out['unknown_bitset'] = (_unk(a, a.i32), _unk(a, a.i32))   # doc +0xaa8 utBitSet
    return out
```
