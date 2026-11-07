use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub(crate) struct ArchiveRevision(Arc<()>);

impl PartialEq for ArchiveRevision {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ArchiveRevision {}

#[cfg(test)]
mod tests {
    use super::ArchiveRevision;

    #[test]
    fn clones_share_ownership_but_independent_revisions_do_not() {
        let first = ArchiveRevision::default();
        assert_eq!(first, first.clone());
        assert_ne!(first, ArchiveRevision::default());
    }
}
