use super::context_reader::ArchiveGraphRead;
use crate::Result;

pub(crate) trait ItemRead: ArchiveGraphRead {
    fn item_text_value(&mut self, label: &str) -> Result<std::sync::Arc<super::ArchiveString>>;
    fn item_count(&mut self, label: &str, minimum: usize) -> Result<usize>;
}
