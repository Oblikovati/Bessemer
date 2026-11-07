use super::context_reader::ArchiveGraphRead;
use crate::Result;

pub(crate) trait BucketRead: ArchiveGraphRead {
    fn bucket_file_version(&self) -> u32;
    fn bucket_polygons(
        &mut self,
        context: &mut Self::ObjectContext,
        label: &str,
        two_dimensional: bool,
    ) -> Result<()>;
    fn bucket_items(&mut self, context: &mut Self::ObjectContext, label: &str) -> Result<()>;
    fn bucket_edges(
        &mut self,
        context: &mut Self::ObjectContext,
        two_dimensional: bool,
    ) -> Result<()>;
    fn bucket_object(&mut self, context: &mut Self::ObjectContext, label: &str) -> Result<()>;
}
