use super::{ModelDecoder, ModelFieldValue, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{
    ArchiveString, ObjectReader, bucket_reader::BucketRead, item_reader::ItemRead,
};
use std::sync::Arc;

impl ItemRead for ModelDecoder<'_> {
    fn item_text_value(&mut self, label: &str) -> Result<Arc<ArchiveString>> {
        let value = self.scalar(ArchiveString::decode)?;
        self.field(label, ModelFieldValue::Text(value.clone()))?;
        Ok(Arc::new(value))
    }

    fn item_count(&mut self, label: &str, minimum: usize) -> Result<usize> {
        self.signed_count(label, minimum)
    }
}

impl BucketRead for ModelDecoder<'_> {
    fn bucket_file_version(&self) -> u32 {
        // Ordinary model contexts declare the current runtime, independently of wire version.
        19000
    }

    fn bucket_polygons(
        &mut self,
        map: &mut ObjectReader,
        label: &str,
        two_dimensional: bool,
    ) -> Result<()> {
        self.scope(label, |reader| {
            crate::archive_codec::bucket_polygons::list(reader, map, two_dimensional)
        })
    }

    fn bucket_items(&mut self, _: &mut ObjectReader, label: &str) -> Result<()> {
        self.scope(label, crate::archive_codec::display_items::list)
    }

    fn bucket_edges(&mut self, map: &mut ObjectReader, two_dimensional: bool) -> Result<()> {
        crate::archive_codec::bucket_visibility::edge_array(self, map, two_dimensional)
    }

    fn bucket_object(&mut self, map: &mut ObjectReader, label: &str) -> Result<()> {
        self.object(label, map, ModelObjectRole::Any)
    }
}
