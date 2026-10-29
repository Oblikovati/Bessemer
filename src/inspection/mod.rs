//! Standalone Inspection encrypted XML envelopes with explicit recovered cipher profiles.
mod cbc;
mod rijndael;
mod tables;

#[cfg(test)]
#[path = "../../tests/inspection/cipher.rs"]
mod cipher_tests;

use crate::metadata::XmlLimits;
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;

/// Recovered fixed-key writer/loader profile; file suffix alone does not select it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionProfile {
    /// AES-128 CBC/PKCS#7, the current standalone writer branch.
    Current2020,
    /// Historical Framework Rijndael with two key words and ten rounds.
    Historical2015,
}

/// Immutable, complete standalone project/template XML and its detected cipher profile.
#[derive(Debug, Clone)]
pub struct InspectionEnvelope {
    profile: InspectionProfile,
    xml: Arc<[u8]>,
    limits: XmlLimits,
}

impl InspectionEnvelope {
    /// Read the documented current-then-historical loader order with bounded XML validation.
    ///
    /// ```no_run
    /// # fn example(bytes: &[u8], limits:bessemer::metadata::XmlLimits)->bessemer::Result<()> {
    /// let envelope=bessemer::inspection::InspectionEnvelope::from_bytes(bytes,limits)?;
    /// assert_eq!(envelope.to_bytes()?,bytes);
    /// # Ok(()) }
    /// ```
    pub fn from_bytes(bytes: &[u8], limits: XmlLimits) -> Result<Self> {
        cbc::require_length(bytes.len(), limits.maximum_bytes)?;
        let current = Self::read_profile(InspectionProfile::Current2020, bytes, limits);
        match current {
            Ok(envelope) => Ok(envelope),
            Err(current_error) => Self::read_profile(InspectionProfile::Historical2015, bytes, limits)
                .map_err(|historical_error| invalid(format!("{} ciphertext bytes; current: {}; historical: {}; expected a supported complete standalone Inspection XML envelope within {limits:?}", bytes.len(), current_error.cause, historical_error.cause))),
        }
    }

    fn read_profile(profile: InspectionProfile, bytes: &[u8], limits: XmlLimits) -> Result<Self> {
        let xml = cbc::decode(profile, bytes, limits.maximum_bytes)?;
        Self::from_xml(profile, &xml, limits)
    }

    /// Construct an envelope from exact UTF-8 standalone XML and an explicit writer profile.
    ///
    /// ```no_run
    /// # fn example(xml:&[u8],limits:bessemer::metadata::XmlLimits)->bessemer::Result<()> {
    /// use bessemer::inspection::{InspectionEnvelope,InspectionProfile};
    /// let envelope=InspectionEnvelope::from_xml(InspectionProfile::Current2020,xml,limits)?;
    /// assert_eq!(envelope.xml_bytes(),xml);
    /// # Ok(()) }
    /// ```
    pub fn from_xml(profile: InspectionProfile, xml: &[u8], limits: XmlLimits) -> Result<Self> {
        cbc::padded_length(xml.len(), limits.maximum_bytes)?;
        crate::metadata::validate_inspection_xml(xml, limits)?;
        Ok(Self {
            profile,
            xml: Arc::from(xml),
            limits,
        })
    }

    /// Return exact validated plaintext, including its original lexical XML framing.
    /// Example: `let xml = envelope.xml_bytes();`.
    pub fn xml_bytes(&self) -> &[u8] {
        &self.xml
    }

    /// Return the normalized profile retained for deterministic reconstruction.
    /// Example: `let profile = envelope.profile();`.
    pub fn profile(&self) -> InspectionProfile {
        self.profile
    }

    /// Replace complete XML on a new snapshot while retaining the original cipher profile.
    ///
    /// ```no_run
    /// # fn example(envelope:&bessemer::inspection::InspectionEnvelope,xml:&[u8])->bessemer::Result<()> {
    /// let edited=envelope.with_xml(xml)?;
    /// let stored=edited.to_bytes()?;
    /// # Ok(()) }
    /// ```
    pub fn with_xml(&self, xml: &[u8]) -> Result<Self> {
        Self::from_xml(self.profile, xml, self.limits)
    }

    /// Reconstruct the complete ciphertext using the profile's fixed IV and PKCS#7 padding.
    /// Example: `let stored = envelope.to_bytes()?;`.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        cbc::encode(self.profile, &self.xml, self.limits.maximum_bytes)
    }
}

fn invalid(cause: String) -> BessemerError {
    BessemerError::new(
        "Inspection envelope",
        "standalone encrypted XML",
        FailureClass::Invalid,
        cause,
    )
}
