use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct OriginalContents<T> {
    contents: Arc<T>,
    encoded: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreservedContents<T> {
    current: Arc<T>,
    original: Option<Arc<OriginalContents<T>>>,
}

impl<T: PartialEq> PreservedContents<T> {
    pub(crate) fn decoded(bytes: &[u8], contents: T) -> Self {
        let current = Arc::new(contents);
        let original = Arc::new(OriginalContents {
            contents: current.clone(),
            encoded: bytes.to_vec(),
        });
        Self {
            current,
            original: Some(original),
        }
    }

    pub(crate) fn constructed(contents: T) -> Self {
        Self {
            current: Arc::new(contents),
            original: None,
        }
    }

    pub(crate) fn edited(&self, contents: T) -> Self {
        Self {
            current: Arc::new(contents),
            original: self.original.clone(),
        }
    }

    pub(crate) fn current(&self) -> &T {
        &self.current
    }

    pub(crate) fn original_bytes(&self) -> Option<&[u8]> {
        self.original
            .as_ref()
            .filter(|original| original.contents == self.current)
            .map(|original| original.encoded.as_slice())
    }
}
