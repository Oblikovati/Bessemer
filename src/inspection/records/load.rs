use super::*;
use crate::metadata::ExpandedName;

/// Selected DOM measurement input projection before runtime conversion or insertion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionLoadedMeasurement {
    /// Original element name; admission does not require Measurement.
    pub element_name: ExpandedName,
    /// Exact unqualified Name attribute, defaulting to an empty string.
    pub name: String,
    /// Exact unqualified Value attribute, defaulting to an empty string.
    pub value: String,
    /// Exact unqualified PartName attribute, defaulting to an empty string.
    pub part_name: String,
    /// Exact unqualified Type attribute, without discriminator inference.
    pub measurement_type: String,
    /// Exact unqualified CreationTime attribute, without date parsing.
    pub creation_time: String,
}

/// Selected custom-property input projection, before delegated collection policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionLoadedCustomProperty {
    /// Original element name; admission does not require CustomProperty.
    pub element_name: ExpandedName,
    /// Exact unqualified Name attribute, defaulting to an empty string.
    pub name: String,
    /// Exact unqualified Value attribute, defaulting to an empty string.
    pub value: String,
}

impl InspectionRecords {
    /// Project ordered DOM measurement inputs; e.g. `project.loaded_measurements(&characteristic.id)?`.
    pub fn loaded_measurements(
        &self,
        characteristic: &InspectionRecordId,
    ) -> Result<Vec<InspectionLoadedMeasurement>> {
        Ok(self
            .load_children(characteristic, "Measurements")?
            .into_iter()
            .map(|ordinal| InspectionLoadedMeasurement {
                element_name: self.xml.name(ordinal).clone(),
                name: self.load_attribute(ordinal, "Name"),
                value: self.load_attribute(ordinal, "Value"),
                part_name: self.load_attribute(ordinal, "PartName"),
                measurement_type: self.load_attribute(ordinal, "Type"),
                creation_time: self.load_attribute(ordinal, "CreationTime"),
            })
            .collect())
    }

    /// Project ordered property inputs without duplicate insertion policy; e.g. `project.loaded_custom_properties(&characteristic.id)?`.
    pub fn loaded_custom_properties(
        &self,
        characteristic: &InspectionRecordId,
    ) -> Result<Vec<InspectionLoadedCustomProperty>> {
        Ok(self
            .load_children(characteristic, "CustomProperties")?
            .into_iter()
            .map(|ordinal| InspectionLoadedCustomProperty {
                element_name: self.xml.name(ordinal).clone(),
                name: self.load_attribute(ordinal, "Name"),
                value: self.load_attribute(ordinal, "Value"),
            })
            .collect())
    }

    fn load_children(&self, id: &InspectionRecordId, container: &str) -> Result<Vec<usize>> {
        let ordinal = self.characteristic_ordinal(id)?;
        Ok(self
            .xml
            .children(ordinal)
            .iter()
            .copied()
            .filter(|child| {
                let name = self.xml.name(*child);
                name.namespace.is_none() && name.local.as_ref() == container
            })
            .flat_map(|child| self.xml.children(child).iter().copied())
            .collect())
    }

    fn characteristic_ordinal(&self, id: &InspectionRecordId) -> Result<usize> {
        let record = self.record(id)?;
        if record.kind != InspectionRecordKind::Characteristic {
            return Err(record_error(
                FailureClass::Invalid,
                format!(
                    "record {id:?} kind {:?}; expected Characteristic",
                    record.kind
                ),
            ));
        }
        Ok(id.ordinal)
    }

    fn load_attribute(&self, ordinal: usize, local: &str) -> String {
        self.xml.attribute(ordinal, local).unwrap_or("").to_owned()
    }
}
