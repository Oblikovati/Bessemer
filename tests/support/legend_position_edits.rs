use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn expected_factor_edit(raw: &[u8], factor: &SimulationLegendPositionFactor) -> Vec<u8> {
    let mut expected = raw.to_vec();
    let (start, end) = factor.wire_range();
    expected[start..end].copy_from_slice(&(factor.raw_bits() ^ 1).to_le_bytes());
    expected
}

fn edit_parent_factors(version: SimulationPersistVersion, raw: &[u8]) {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let original = parent.legend_position_factors().unwrap();
    for factor in [original.horizontal(), original.vertical()]
        .into_iter()
        .flatten()
    {
        let edit = parent
            .edit_field(
                factor.source_id(),
                SimulationPostDialogValue::Float32Bits(factor.raw_bits() ^ 1),
            )
            .unwrap();
        assert_eq!(edit.archive().to_bytes(), expected_factor_edit(raw, factor));
        assert_eq!(edit.source().to_bytes(), raw);
        assert!(edit.archive().field(factor.source_id()).is_err());
        assert_edited_factor(factor, &edit.archive().legend_position_factors().unwrap());
        assert_eq!(parent.legend_position_factors().unwrap(), original);
    }
}

fn edit_style_factors(version: SimulationPersistVersion, raw: &[u8]) -> bool {
    let style = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let original = style.legend_position_factors().unwrap();
    let present = original.horizontal().is_some();
    for factor in [original.horizontal(), original.vertical()]
        .into_iter()
        .flatten()
    {
        let edit = style
            .edit_field(
                factor.source_id(),
                SimulationPostStyleValue::Float32Bits(factor.raw_bits() ^ 1),
            )
            .unwrap();
        assert_eq!(edit.archive().to_bytes(), expected_factor_edit(raw, factor));
        assert_eq!(edit.source().to_bytes(), raw);
        assert!(edit.archive().field(factor.source_id()).is_err());
        assert_edited_factor(factor, &edit.archive().legend_position_factors().unwrap());
        assert_eq!(style.legend_position_factors().unwrap(), original);
    }
    present
}

fn assert_edited_factor(
    original: &SimulationLegendPositionFactor,
    after: &SimulationLegendPositionFactors,
) {
    let factor = match original.axis() {
        SimulationLegendPositionAxis::Horizontal => after.horizontal().unwrap(),
        SimulationLegendPositionAxis::Vertical => after.vertical().unwrap(),
    };
    assert_eq!(factor.raw_bits(), original.raw_bits() ^ 1);
    assert_ne!(factor.source_id(), original.source_id());
    assert_eq!(factor.wire_range(), original.wire_range());
}

pub fn assert_position_id_edits(mut cursor: &[u8]) {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    let mut completed = [false; 2];
    for _ in 0..count {
        let (family, version, raw) = read_position_edit_context(&mut cursor);
        if family == 0 && !completed[0] {
            edit_parent_factors(version, raw);
            completed[0] = true;
        }
        if family == 2 && !completed[1] {
            completed[1] = edit_style_factors(version, raw);
        }
        if completed == [true; 2] {
            return;
        }
    }
    panic!("expected complete parent/style factor edits; observed {completed:?}")
}

fn read_position_edit_context<'a>(
    cursor: &mut &'a [u8],
) -> (u32, SimulationPersistVersion, &'a [u8]) {
    let family = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    take_projection(cursor, 56);
    (family, version, raw)
}
