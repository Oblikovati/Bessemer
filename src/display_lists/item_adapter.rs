use super::archive_decode::DisplayDecoder;
use crate::Result;
use crate::archive_codec::item_reader::ItemRead;

impl ItemRead for DisplayDecoder<'_> {
    fn item_text_value(
        &mut self,
        label: &str,
    ) -> Result<std::sync::Arc<crate::archive_codec::ArchiveString>> {
        self.text_value(label)
    }

    fn item_count(&mut self, label: &str, minimum: usize) -> Result<usize> {
        self.count(label, minimum)
    }
}
