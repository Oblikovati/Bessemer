use super::archive_decode::DisplayDecoder;
use crate::Result;
use crate::archive_codec::context_reader::ArchiveGraphRead;

impl ArchiveGraphRead for DisplayDecoder<'_> {
    type ObjectContext = ();

    fn graph_object(&mut self, _: &mut (), label: &str, classes: &[&str]) -> Result<()> {
        self.object(label, classes)
    }

    fn graph_transform(&mut self, label: &str) -> Result<()> {
        self.transform(label)
    }
}
