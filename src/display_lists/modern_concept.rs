use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.object("name", &["moNodeName_c"])?;
    archive.i32s(&["update_state", "flags", "id", "flags2"])?;
    archive.text("tool_tip_name")?;
    if archive.i32("has_merged_feature_list")? != 0 {
        archive.scope("merged_feature_list", node_list)?;
    }
    archive.object("viz_prop", &["moVisualProperties_c"])?;
    archive.scope("feature_list", node_list)
}

fn node_list(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    // The native suObList reads CObject pointers without a declared runtime class.
    archive.cobject_array("nodes")?;
    archive.i32("unknown_tail")?;
    Ok(())
}
