use super::codec::FacetCodec;
use super::context::FacetState;
use super::output::FacetOutput;
use super::{TessellationContext, TessellationLimits};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, PreservedContents};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ArchiveEnvelope<T> {
    context: TessellationContext,
    contents: PreservedContents<T>,
}

impl<T: FacetCodec + PartialEq> ArchiveEnvelope<T> {
    pub fn decoded(
        bytes: &[u8],
        context: TessellationContext,
        limits: TessellationLimits,
    ) -> Result<Self> {
        let mut cursor = ArchiveCursor::new(bytes);
        let contents = T::read(&mut cursor, &mut FacetState::new(context, limits))?;
        cursor.finish()?;
        validate(&contents, context)?;
        Ok(Self {
            context,
            contents: PreservedContents::decoded(bytes, contents),
        })
    }

    pub fn constructed(context: TessellationContext, contents: T) -> Result<Self> {
        validate(&contents, context)?;
        Ok(Self {
            context,
            contents: PreservedContents::constructed(contents),
        })
    }

    pub fn context(&self) -> TessellationContext {
        self.context
    }
    pub fn contents(&self) -> &T {
        self.contents.current()
    }

    pub fn edited(&self, contents: T) -> Result<Self> {
        validate(&contents, self.context)?;
        Ok(Self {
            context: self.context,
            contents: self.contents.edited(contents),
        })
    }

    pub fn encoded(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.contents.original_bytes() {
            return Ok(bytes.to_vec());
        }
        let mut output = FacetOutput::encoded();
        self.contents()
            .write(&mut output, &mut FacetState::writing(self.context))?;
        Ok(output.finish())
    }
}

fn validate<T: FacetCodec>(contents: &T, context: TessellationContext) -> Result<()> {
    contents.write(
        &mut FacetOutput::validation(),
        &mut FacetState::writing(context),
    )
}

macro_rules! archive_wrapper {
    ($(#[$doc:meta])* $name:ident, $contents:ident, $path:literal) => {
        $(#[$doc])* #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name($crate::tessellation::envelope::ArchiveEnvelope<$contents>);

        impl $name {
            /// Decodes a complete archive with explicit version, mode, and allocation limits.
            #[doc = concat!("\n```no_run\n# use bessemer::tessellation::*;\n# fn example(bytes: &[u8], context: TessellationContext, limits: TessellationLimits) -> bessemer::Result<()> {\nlet archive = ", $path, "::from_bytes(bytes, context, limits)?;\n# Ok(())\n# }\n```")]
            pub fn from_bytes(bytes: &[u8], context: TessellationContext, limits: TessellationLimits) -> $crate::Result<Self> {
                $crate::tessellation::envelope::ArchiveEnvelope::decoded(bytes, context, limits).map(Self)
            }

            /// Constructs an immutable archive after checking every field's shape and presence.
            #[doc = concat!("\n```no_run\n# use bessemer::tessellation::*;\n# fn example(contents: ", stringify!($contents), ", context: TessellationContext) -> bessemer::Result<()> {\nlet archive = ", $path, "::new(context, contents)?;\n# Ok(())\n# }\n```")]
            pub fn new(context: TessellationContext, contents: $contents) -> $crate::Result<Self> {
                $crate::tessellation::envelope::ArchiveEnvelope::constructed(context, contents).map(Self)
            }

            /// Returns the explicit archive context; the stream does not encode it.
            #[doc = concat!("\n```no_run\n# fn example(source: &", $path, ") {\nlet context = source.context();\n# }\n```")]
            pub fn context(&self) -> TessellationContext { self.0.context() }

            /// Borrows immutable stored values, including unknown and ignored fields.
            #[doc = concat!("\n```no_run\n# fn example(source: &", $path, ") {\nlet stored = source.contents();\n# }\n```")]
            pub fn contents(&self) -> &$contents { self.0.contents() }

            /// Validates an owned edit, leaving the source isolated after success or failure.
            #[doc = concat!("\n```no_run\n# fn example(source: &", $path, ") -> bessemer::Result<()> {\nlet edited = source.with_contents(source.contents().clone())?;\n# Ok(())\n# }\n```")]
            pub fn with_contents(&self, contents: $contents) -> $crate::Result<Self> { self.0.edited(contents).map(Self) }

            /// Rebuilds changed archives and preserves original bytes for equivalent edits or reverts.
            #[doc = concat!("\n```no_run\n# fn example(source: &", $path, ") -> bessemer::Result<()> {\nlet bytes = source.to_bytes()?;\n# Ok(())\n# }\n```")]
            pub fn to_bytes(&self) -> $crate::Result<Vec<u8>> { self.0.encoded() }
        }
    };
}

pub(super) use archive_wrapper;
