use super::{ModelField, ModelFieldValue, PartResolvedFeaturesArchive};
use crate::Result;
use std::sync::Arc;

/// Immutable raw geometry flag with the separately derived native input interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaneGeometryFlag {
    path: Arc<str>,
    raw: u8,
}

impl PlaneGeometryFlag {
    /// Identifies the stored flag occurrence without assigning document names.
    /// ```no_run
    /// # fn example(flag: &bessemer::model_archives::PlaneGeometryFlag) {
    /// let field_path = flag.path();
    /// # }
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns every original bit, including noncanonical Boolean bytes.
    /// ```no_run
    /// # fn example(flag: &bessemer::model_archives::PlaneGeometryFlag) {
    /// let stored_byte = flag.raw();
    /// # }
    /// ```
    pub fn raw(&self) -> u8 {
        self.raw
    }

    /// Matches native loading: exactly one is true; zero, two and 255 are false.
    /// ```no_run
    /// # fn example(flag: &bessemer::model_archives::PlaneGeometryFlag) {
    /// let native_boolean = flag.loaded();
    /// # }
    /// ```
    pub fn loaded(&self) -> bool {
        self.raw == 1
    }
}

impl PartResolvedFeaturesArchive {
    /// Derives owned geometry flags while leaving all raw fields and stored bytes unchanged.
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) -> bessemer::Result<()> {
    /// let flags = archive.plane_geometry_flags()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn plane_geometry_flags(&self) -> Result<Vec<PlaneGeometryFlag>> {
        read(self.fields())
    }
}

pub(super) fn read(fields: &[ModelField]) -> Result<Vec<PlaneGeometryFlag>> {
    fields
        .iter()
        .filter(|field| {
            [".raw_plane_geometry_flag_38", ".raw_plane_geometry_flag_40"]
                .iter()
                .any(|suffix| field.path.ends_with(suffix))
        })
        .map(flag)
        .collect()
}

fn flag(field: &ModelField) -> Result<PlaneGeometryFlag> {
    let ModelFieldValue::U8(raw) = field.value else {
        return Err(super::invalid(format!(
            "geometry flag {}; expected decoded u8 with native equality-to-one interpretation",
            super::field_mismatch::field_summary(Some(field))
        )));
    };
    Ok(PlaneGeometryFlag {
        path: Arc::clone(&field.path),
        raw,
    })
}
