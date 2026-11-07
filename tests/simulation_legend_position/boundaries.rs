use super::views::position_from_snapshot;
use crate::archive_codec::ArchiveRevision;
use crate::simulation::persistence::load_snapshot::BinaryLoadSnapshot;
use crate::simulation::*;
use crate::{FailureClass, Result};

#[derive(Clone)]
struct PositionSnapshotFake {
    fields: Vec<SimulationThermalField>,
    spans: Vec<(usize, usize)>,
    bytes: Vec<u8>,
    revision: ArchiveRevision,
}

impl PositionSnapshotFake {
    fn complete() -> Self {
        Self {
            fields: vec![SimulationThermalField {
                path: "legend.word_ac".into(),
                value: SimulationThermalValue::Unsigned(0x80000000),
            }],
            spans: vec![(0, 4)],
            bytes: 0x80000000u32.to_le_bytes().to_vec(),
            revision: ArchiveRevision::default(),
        }
    }
}

impl BinaryLoadSnapshot for PositionSnapshotFake {
    fn fields(&self) -> &[SimulationThermalField] { &self.fields }
    fn field_id(&self, ordinal: usize) -> Result<SimulationThermalFieldId> {
        assert!(ordinal < self.fields.len());
        Ok(SimulationThermalFieldId { revision: self.revision.clone(), ordinal })
    }
    fn field_offsets(&self) -> &[(usize, usize)] { &self.spans }
    fn wire_bytes(&self) -> &[u8] { &self.bytes }
    fn subject(&self) -> &'static str { "position snapshot fake" }
}

#[test]
fn explicit_single_factor_keeps_negative_zero_and_revision_bound_source_history() {
    let fake = PositionSnapshotFake::complete();
    assert_eq!(fake.subject(), "position snapshot fake");
    let view = position_from_snapshot(&fake).unwrap();
    let factor = view.horizontal().unwrap();
    assert!(view.vertical().is_none());
    assert_eq!(factor.raw_bits(), 0x80000000);
    assert_eq!(factor.finite_value().unwrap().to_bits(), 0x80000000);
    assert_eq!(factor.source_id(), &fake.field_id(0).unwrap());
    let history = factor.history();
    assert_eq!(history.result, fake.field_id(0).unwrap());
    assert!(matches!(&history.parents[..], [SimulationThermalHistoryParent::Source(id)] if id == factor.source_id()));
    assert_eq!(history.relation, SimulationThermalRelation::Copied);
}

#[test]
fn missing_and_invalid_source_spans_fail_instead_of_publishing_partial_factors() {
    let original = PositionSnapshotFake::complete();
    for spans in [vec![], vec![(0, 3)], vec![(2, 6)], vec![(3, 1)]] {
        let mut fake = original.clone();
        fake.spans = spans;
        let error = position_from_snapshot(&fake).unwrap_err();
        let expected = if fake.spans.is_empty() { FailureClass::Missing } else { FailureClass::Invalid };
        assert_eq!(error.operation, "legend position factor");
        assert_eq!(error.entity, "legend.word_ac");
        assert_eq!(error.class, expected);
        assert!(error.cause.contains("expected"));
    }
    assert_eq!(position_from_snapshot(&original).unwrap().horizontal().unwrap().raw_bits(), 0x80000000);
}

#[test]
fn byte_mismatch_and_wrong_value_kind_remain_errors_and_absence_remains_explicit() {
    let mut fake = PositionSnapshotFake::complete();
    fake.bytes.fill(0);
    assert_eq!(position_from_snapshot(&fake).unwrap_err().class, FailureClass::Invalid);
    fake.fields[0].value = SimulationThermalValue::Bytes(vec![0; 4].into());
    assert_eq!(position_from_snapshot(&fake).unwrap_err().class, FailureClass::Invalid);
    fake.fields.clear();
    let view = position_from_snapshot(&fake).unwrap();
    assert_eq!(view.completion(), SimulationLegendPositionCompletion::CompletedAbsent);
    assert_eq!(view.coverage().checked_bytes, 0);
    assert!(view.horizontal().is_none() && view.vertical().is_none());
}
