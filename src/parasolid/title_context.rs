use crate::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TitleContext {
    Marked,
    // #49: model kernel framing declares a partition even when the native title omits a marker.
    #[cfg(test)]
    EmbeddedPartition,
}

impl TitleContext {
    pub fn is_delta(self, title: &str) -> Result<bool> {
        match self {
            Self::Marked => super::validate::is_delta_title(title),
            #[cfg(test)]
            Self::EmbeddedPartition if !title.contains("(deltas)") => Ok(false),
            #[cfg(test)]
            Self::EmbeddedPartition => Err(super::invalid(format!(
                "embedded title {title:?}; expected a partition without a contradictory (deltas) marker"
            ))),
        }
    }
}
