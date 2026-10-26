use super::archive_decode::DisplayDecoder;
use crate::Result;
use crate::archive_codec::bucket_reader::BucketRead;

impl BucketRead for DisplayDecoder<'_> {
    fn bucket_file_version(&self) -> u32 {
        self.context.file_version
    }

    fn bucket_polygons(&mut self, _: &mut (), label: &str, two_dimensional: bool) -> Result<()> {
        self.scope(label, |archive| {
            super::modern_bucket_poly::list(archive, two_dimensional)
        })
    }

    fn bucket_items(&mut self, _: &mut (), label: &str) -> Result<()> {
        self.scope(label, super::modern_items::list)
    }

    fn bucket_edges(&mut self, _: &mut (), two_dimensional: bool) -> Result<()> {
        super::legacy_bucket_visibility::edge_array(self, two_dimensional)
    }

    fn bucket_object(&mut self, _: &mut (), label: &str) -> Result<()> {
        self.cobject(label)
    }
}
