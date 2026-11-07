use crate::{BessemerError, FailureClass, Result};

/// Unsigned display-list archive version, distinct from the model/file version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayVersion(u32);

impl DisplayVersion {
    /// Takes an already normalized unsigned native archive version.
    ///
    /// ```
    /// assert_eq!(bessemer::display_lists::DisplayVersion::new(19000).value(),19000);
    /// ```
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the stored unsigned version used by native conditional fields.
    ///
    /// ```
    /// assert_eq!(bessemer::display_lists::DisplayVersion::new(u32::MAX).value(),u32::MAX);
    /// ```
    pub fn value(self) -> u32 {
        self.0
    }

    /// Resolves exact DLUpdateStamp markers, rejecting absence or conflicting versions.
    ///
    /// ```
    /// let version=bessemer::display_lists::DisplayVersion::from_stream_names(["_DL_VERSION_-1/DLUpdateStamp"])?;
    /// assert_eq!(version.value(),u32::MAX);
    /// # Ok::<(),bessemer::BessemerError>(())
    /// ```
    pub fn from_stream_names<'a>(names: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        let mut version = None;
        for name in names {
            let Some(number) = marker_number(name) else {
                continue;
            };
            let parsed = Self(parse_number(number)?);
            if version.is_some_and(|previous| previous != parsed) {
                return Err(error(format!(
                    "conflicting display versions {version:?} and {parsed:?}; expected one normalized DLUpdateStamp version"
                )));
            }
            version = Some(parsed);
        }
        version.ok_or_else(missing_marker)
    }
}

fn marker_number(name: &str) -> Option<&str> {
    name.strip_prefix("_DL_VERSION_")
        .and_then(|tail| tail.strip_suffix("/DLUpdateStamp"))
}

fn missing_marker() -> BessemerError {
    BessemerError {
        operation: "display-list version",
        entity: "DLUpdateStamp".into(),
        class: FailureClass::Missing,
        cause: "no exact version marker; expected _DL_VERSION_<integer>/DLUpdateStamp".into(),
    }
}

fn parse_number(number: &str) -> Result<u32> {
    if let Some(magnitude) = number.strip_prefix('-') {
        require_decimal(magnitude)?;
        return number
            .parse::<i32>()
            .map(|value| value as u32)
            .map_err(|_| {
                error(format!(
                    "signed display version {number:?}; expected -2147483648..=-0"
                ))
            });
    }
    require_decimal(number)?;
    number
        .parse()
        .map_err(|_| error(format!("display version {number:?}; expected decimal u32")))
}

fn require_decimal(number: &str) -> Result<()> {
    if !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(());
    }
    Err(error(format!(
        "display version component {number:?}; expected nonempty ASCII decimal digits"
    )))
}

fn error(cause: String) -> BessemerError {
    BessemerError {
        operation: "display-list version",
        entity: "DLUpdateStamp".into(),
        class: FailureClass::Invalid,
        cause,
    }
}
