use super::*;

impl BitmapResourceArray {
    /// Replaces opaque payload bytes with isolated validation and history; e.g. `let edit = resource.edit_payload(&[3, 4])?;`.
    pub fn edit_payload(&self, payload: &[u8]) -> Result<BitmapResourceEdit> {
        codec::validate_payload(payload.len(), self.maximum_payload)?;
        let resource = Self {
            contents: self.contents.edited(payload.to_vec()),
            preferred_width: self.preferred_width,
            maximum_payload: self.maximum_payload,
        };
        let original = self.to_bytes()?;
        let encoded = resource.to_bytes()?;
        let coverage = resource.validate_encoded(&encoded)?;
        Ok(publish_edit(self, resource, &original, &encoded, coverage))
    }

    fn validate_encoded(&self, encoded: &[u8]) -> Result<BitmapResourceCoverage> {
        let reparsed = Self::from_bytes(encoded, self.maximum_payload)?;
        let error = payload_error(reparsed.payload(), self.payload());
        if error != 0 {
            return Err(invalid(format!(
                "reparsed payload differs by {error} bytes; expected exact payload equality"
            )));
        }
        Ok(BitmapResourceCoverage {
            mode: BitmapResourceCheckMode::ExactCountAndPayload,
            wire_bytes: encoded.len(),
            count_prefix_bytes: encoded.len() - self.payload().len(),
            payload_bytes: self.payload().len(),
            achieved_byte_error: error,
            object_map_slots_added: 0,
        })
    }
}

fn resource_history(
    source: &BitmapResourceArray,
    result: &BitmapResourceArray,
    old: &[u8],
    new: &[u8],
) -> [BitmapResourceHistory; 2] {
    let old_prefix = old.len() - source.payload().len();
    let new_prefix = new.len() - result.payload().len();
    let count_equal = old[..old_prefix] == new[..new_prefix];
    let payload_equal = source.payload() == result.payload();
    let count = history_field(
        BitmapResourceField::Count,
        count_equal,
        0..old_prefix,
        0..new_prefix,
    );
    let payload = history_field(
        BitmapResourceField::Payload,
        payload_equal,
        old_prefix..old.len(),
        new_prefix..new.len(),
    );
    [count, payload]
}

fn history_field(
    field: BitmapResourceField,
    copied: bool,
    parent: std::ops::Range<usize>,
    result: std::ops::Range<usize>,
) -> BitmapResourceHistory {
    let relation = if copied {
        BitmapResourceRelation::Copied
    } else {
        BitmapResourceRelation::Derived
    };
    BitmapResourceHistory {
        field,
        relation,
        parents: vec![parent],
        result,
    }
}

fn publish_edit(
    source: &BitmapResourceArray,
    resource: BitmapResourceArray,
    original: &[u8],
    encoded: &[u8],
    coverage: BitmapResourceCoverage,
) -> BitmapResourceEdit {
    let completion = if original == encoded {
        BitmapResourceCompletion::Unchanged
    } else {
        BitmapResourceCompletion::Changed
    };
    let history = resource_history(source, &resource, original, encoded);
    BitmapResourceEdit {
        source: source.clone(),
        resource,
        completion,
        coverage,
        history,
    }
}

fn payload_error(actual: &[u8], expected: &[u8]) -> usize {
    let mismatches = actual
        .iter()
        .zip(expected)
        .filter(|(left, right)| left != right)
        .count();
    mismatches + actual.len().abs_diff(expected.len())
}
