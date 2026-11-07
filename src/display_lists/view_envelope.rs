use super::{DisplayVersion, DisplayViewLimits, view_support::require_size};
use crate::{Result, archive_codec::PreservedContents};

type ViewEncoder<C> = fn(DisplayVersion, &C, DisplayViewLimits) -> Result<Vec<u8>>;

#[derive(Debug, Clone)]
pub(super) struct ViewEnvelope<C> {
    version: DisplayVersion,
    source: PreservedContents<C>,
    limits: DisplayViewLimits,
    encoder: ViewEncoder<C>,
    preserved_label: &'static str,
}

impl<C: PartialEq> ViewEnvelope<C> {
    pub fn decoded(
        bytes: &[u8],
        contents: C,
        version: DisplayVersion,
        limits: DisplayViewLimits,
        encoder: ViewEncoder<C>,
        label: &'static str,
    ) -> Self {
        Self::assemble(
            PreservedContents::decoded(bytes, contents),
            version,
            limits,
            encoder,
            label,
        )
    }

    pub fn new(
        contents: C,
        version: DisplayVersion,
        limits: DisplayViewLimits,
        encoder: ViewEncoder<C>,
        label: &'static str,
    ) -> Result<Self> {
        encoder(version, &contents, limits)?;
        Ok(Self::assemble(
            PreservedContents::constructed(contents),
            version,
            limits,
            encoder,
            label,
        ))
    }

    fn assemble(
        source: PreservedContents<C>,
        version: DisplayVersion,
        limits: DisplayViewLimits,
        encoder: ViewEncoder<C>,
        preserved_label: &'static str,
    ) -> Self {
        Self {
            version,
            source,
            limits,
            encoder,
            preserved_label,
        }
    }

    pub fn contents(&self) -> &C {
        self.source.current()
    }
    pub fn version(&self) -> DisplayVersion {
        self.version
    }

    pub fn edited(&self, contents: C, limits: DisplayViewLimits) -> Result<Self> {
        (self.encoder)(self.version, &contents, limits)?;
        let source = self.source.edited(contents);
        // #29: CString-equivalent fields can retain a longer authentic prefix encoding.
        if let Some(bytes) = source.original_bytes() {
            require_size(bytes.len(), limits.maximum_bytes, self.preserved_label)?;
        }
        Ok(Self::assemble(
            source,
            self.version,
            limits,
            self.encoder,
            self.preserved_label,
        ))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.source.original_bytes() {
            return Ok(bytes.to_vec());
        }
        (self.encoder)(self.version, self.contents(), self.limits)
    }
}
