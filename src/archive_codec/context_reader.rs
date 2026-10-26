use super::MaterialRead;
use crate::Result;

pub(crate) trait ArchiveGraphRead: MaterialRead {
    type ObjectContext;
    fn graph_object(
        &mut self,
        context: &mut Self::ObjectContext,
        label: &str,
        classes: &[&str],
    ) -> Result<()>;
    fn graph_transform(&mut self, label: &str) -> Result<()>;
}
