use super::{ExpandedName, invalid, unsupported};
use crate::Result;

/// Documented XML stream family; numeric configuration/summary identities are parsed once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataFamily {
    /// Package content-type declarations.
    ContentTypes,
    /// Package relationship declarations.
    Relationships,
    /// Core document properties.
    CoreProperties,
    /// Application/document summary properties.
    ApplicationProperties,
    /// Custom property sections.
    CustomProperties,
    /// Configuration property sections, with their normalized configuration number.
    ConfigurationProperties(u32),
    /// SolidWorks information property sections.
    SolidWorksInformation,
    /// Unqualified configuration/cutlist properties.
    CutlistProperties(u32),
    /// Open-time properties in their distinct namespace.
    OpenTime,
    /// Component-instance tree summary.
    ComponentInstanceTree,
    /// Feature summary.
    Features,
    /// Keyword summary with optional opaque leading byte.
    KeyWords,
    /// Material-tree summary; an empty stream is allowed.
    MaterialTree,
    /// Custom-property display summary; an empty stream is allowed.
    CustomPropertySummary,
    /// Table summary; an empty stream is allowed.
    Tables,
    /// Numbered XML summary stream.
    NumberedSummary(u32),
}

/// Validated stream path and its parsed family; aliases are not guessed or case-folded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataStream {
    name: String,
    family: MetadataFamily,
}

impl MetadataStream {
    /// Classifies a documented XML path while retaining its exact spelling.
    ///
    /// ```
    /// let stream=bessemer::metadata::MetadataStream::parse("docProps/Config-2-Properties.xml")?;
    /// assert_eq!(stream.family(),bessemer::metadata::MetadataFamily::ConfigurationProperties(2));
    /// # Ok::<(),bessemer::BessemerError>(())
    /// ```
    pub fn parse(name: &str) -> Result<Self> {
        let family = classify(name)?;
        Ok(Self {
            name: name.into(),
            family,
        })
    }

    /// Returns the original stream path for Document extraction/replacement.
    ///
    /// ```
    /// assert_eq!(bessemer::metadata::MetadataStream::parse("docProps/core.xml")?.name(),"docProps/core.xml");
    /// # Ok::<(),bessemer::BessemerError>(())
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the normalized stream family.
    ///
    /// ```
    /// assert_eq!(bessemer::metadata::MetadataStream::parse("swXmlContents/KeyWords")?.family(),bessemer::metadata::MetadataFamily::KeyWords);
    /// # Ok::<(),bessemer::BessemerError>(())
    /// ```
    pub fn family(&self) -> MetadataFamily {
        self.family
    }
}

fn classify(name: &str) -> Result<MetadataFamily> {
    known_family(name).map_or_else(|| classify_numbered(name), Ok)
}

fn known_family(name: &str) -> Option<MetadataFamily> {
    use MetadataFamily::*;
    match name {
        "[Content_Types].xml" => Some(ContentTypes),
        "_rels/.rels" => Some(Relationships),
        "docProps/core.xml" => Some(CoreProperties),
        "docProps/app.xml" => Some(ApplicationProperties),
        "docProps/custom.xml" => Some(CustomProperties),
        "docProps/ISolidWorksInformation.xml" => Some(SolidWorksInformation),
        "docProps/OpenTime.xml" => Some(OpenTime),
        "swXmlContents/COMPINSTANCETREE" => Some(ComponentInstanceTree),
        "swXmlContents/Features" => Some(Features),
        "swXmlContents/KeyWords" => Some(KeyWords),
        "swXmlContents/MATERIALTREE" => Some(MaterialTree),
        "swXmlContents/CustomProperties" => Some(CustomPropertySummary),
        "swXmlContents/Tables" => Some(Tables),
        _ => None,
    }
}

fn classify_numbered(name: &str) -> Result<MetadataFamily> {
    if let Some(tail) = name.strip_prefix("docProps/Config-") {
        if let Some(number) = tail.strip_suffix("-Cutlist-Properties.xml") {
            return number_id(number, name).map(MetadataFamily::CutlistProperties);
        }
        if let Some(number) = tail.strip_suffix("-Properties.xml") {
            return number_id(number, name).map(MetadataFamily::ConfigurationProperties);
        }
    }
    if let Some(number) = name.strip_prefix("swXmlContents/xmlstream-") {
        return number_id(number, name).map(MetadataFamily::NumberedSummary);
    }
    Err(unsupported(format!(
        "XML stream path {name:?}; expected a documented docProps, package or swXmlContents metadata family"
    )))
}

fn number_id(number: &str, path: &str) -> Result<u32> {
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid(format!(
            "XML stream numeric identity {number:?} in {path:?}; expected decimal u32"
        )));
    }
    number.parse().map_err(|_| {
        invalid(format!(
            "XML stream numeric identity {number:?} in {path:?}; expected 0..=4294967295"
        ))
    })
}

impl MetadataFamily {
    pub(super) fn allows_empty(self) -> bool {
        matches!(
            self,
            Self::MaterialTree | Self::CustomPropertySummary | Self::Tables
        )
    }
    pub(super) fn property_sections(self) -> bool {
        matches!(
            self,
            Self::CustomProperties | Self::ConfigurationProperties(_) | Self::SolidWorksInformation
        )
    }
    pub(super) fn expected_root(self) -> ExpandedName {
        let (namespace, local) = self.root_parts();
        ExpandedName {
            namespace: namespace.map(std::sync::Arc::from),
            local: local.into(),
        }
    }

    fn root_parts(self) -> (Option<&'static str>, &'static str) {
        (self.root_namespace(), self.root_local())
    }

    fn root_local(self) -> &'static str {
        use MetadataFamily::*;
        match self {
            ContentTypes => "Types",
            Relationships => "Relationships",
            CoreProperties => "coreProperties",
            ConfigurationProperties(_) => "ConfigProperties",
            ComponentInstanceTree | Features | NumberedSummary(_) => "swSolidWorks",
            MaterialTree => "materials",
            CutlistProperties(_) => "Configuration",
            KeyWords => "Keywords",
            CustomPropertySummary => "DisplayItems",
            Tables => "Tables",
            _ => "Properties",
        }
    }

    fn root_namespace(self) -> Option<&'static str> {
        use MetadataFamily::*;
        match self {
            ContentTypes => Some("http://schemas.openxmlformats.org/package/2006/content-types"),
            Relationships => Some("http://schemas.openxmlformats.org/package/2006/relationships"),
            CoreProperties => {
                Some("http://schemas.openxmlformats.org/package/2006/metadata/core-properties")
            }
            ApplicationProperties => {
                Some("http://schemas.openxmlformats.org/officeDocument/2006/extended-properties")
            }
            CustomProperties => {
                Some("http://schemas.openxmlformats.org/officeDocument/2006/custom-properties")
            }
            _ => self.solidworks_namespace(),
        }
    }

    fn solidworks_namespace(self) -> Option<&'static str> {
        use MetadataFamily::*;
        match self {
            ConfigurationProperties(_) => Some("http://www.solidworks.com/config-properties"),
            SolidWorksInformation => Some(
                "http://schemas.openxmlformats.org/officeDocument/2006/SolidworksInformation-properties",
            ),
            OpenTime => {
                Some("http://schemas.openxmlformats.org/officeDocument/2006/SolidworksOpenTime")
            }
            ComponentInstanceTree | Features | NumberedSummary(_) => {
                Some("http://www.solidworks.com/sw2003/schema")
            }
            MaterialTree => Some("http://www.solidworks.com/sldmaterials"),
            _ => None,
        }
    }
}
