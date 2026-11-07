//! Fixed-order field groups shared by complete archive audits and owned version tests.
use bessemer::display_lists::DisplayArchive;

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut fields = crate::reference_trace::trace(archive);
    fields.extend(crate::scene_trace::trace(archive));
    fields.extend(crate::item_trace::trace(archive));
    fields.extend(crate::bucket_trace::trace(archive));
    fields.extend(crate::light_trace::trace(archive));
    fields.extend(crate::material_trace::trace(archive));
    fields
}
