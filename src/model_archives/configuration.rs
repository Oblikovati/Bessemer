use crate::{BessemerError, FailureClass, Result};

/// Normalized unsigned identifier for one canonical `Contents/Config-N` stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConfigurationId(u32);

impl ConfigurationId {
    /// Uses an already normalized configuration number; zero is a valid configuration.
    ///
    /// ```
    /// assert_eq!(bessemer::model_archives::ConfigurationId::new(0).value(), 0);
    /// ```
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the configuration number without reparsing its stream path.
    ///
    /// ```
    /// assert_eq!(bessemer::model_archives::ConfigurationId::new(9).value(), 9);
    /// ```
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Parses a canonical model stream path, rejecting aliases, suffixes and signed IDs.
    ///
    /// ```
    /// let id = bessemer::model_archives::ConfigurationId::from_stream_name("Contents/Config-9")?;
    /// assert_eq!(id.value(), 9);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn from_stream_name(name: &str) -> Result<Self> {
        let number = name.strip_prefix("Contents/Config-").unwrap_or("");
        let value = number.parse::<u32>().map(Self).ok();
        value.filter(|id| id.stream_name() == name).ok_or_else(|| {
            BessemerError::new(
                "configuration selection",
                name,
                FailureClass::Invalid,
                format!(
                    "configuration path {name:?}; expected canonical Contents/Config-<u32 decimal>"
                ),
            )
        })
    }

    /// Returns the exact canonical model-stream path for this identifier.
    ///
    /// ```
    /// assert_eq!(bessemer::model_archives::ConfigurationId::new(9).stream_name(), "Contents/Config-9");
    /// ```
    pub fn stream_name(self) -> String {
        format!("Contents/Config-{}", self.0)
    }

    /// Returns the matching configuration-header path.
    ///
    /// ```
    /// assert_eq!(bessemer::model_archives::ConfigurationId::new(9).header_name(), "Contents/Config-9-ModelHeader");
    /// ```
    pub fn header_name(self) -> String {
        format!("{}-ModelHeader", self.stream_name())
    }
}
