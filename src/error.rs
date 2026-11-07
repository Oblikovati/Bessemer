use std::fmt;

/// Why a container operation failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureClass {
    /// The bytes do not satisfy the container layout.
    Invalid,
    /// A format variant is outside the implemented scope.
    Unsupported,
    /// The requested stream does not exist.
    Missing,
    /// Compression or integrity validation failed.
    Integrity,
}

/// A failed operation with its affected entity and cause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BessemerError {
    /// Operation that failed.
    pub operation: &'static str,
    /// Stream name or container region.
    pub entity: String,
    /// Failure category.
    pub class: FailureClass,
    /// Observed value and expected constraint.
    pub cause: String,
}

/// Result of a Bessemer operation.
pub type Result<T> = std::result::Result<T, BessemerError>;

impl BessemerError {
    pub(crate) fn new(
        operation: &'static str,
        entity: &str,
        class: FailureClass,
        cause: impl Into<String>,
    ) -> Self {
        Self {
            operation,
            entity: entity.into(),
            class,
            cause: cause.into(),
        }
    }
}

impl fmt::Display for BessemerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {}: {:?}: {}",
            self.operation, self.entity, self.class, self.cause
        )
    }
}

impl std::error::Error for BessemerError {}
