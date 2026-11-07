use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{ObjectReader, context_reader::ArchiveGraphRead};

impl ArchiveGraphRead for ModelDecoder<'_> {
    type ObjectContext = ObjectReader;

    fn graph_object(
        &mut self,
        map: &mut ObjectReader,
        label: &str,
        classes: &[&str],
    ) -> Result<()> {
        self.object(label, map, ModelObjectRole::Declared(classes))
    }

    fn graph_transform(&mut self, label: &str) -> Result<()> {
        self.transform(label)
    }
}
