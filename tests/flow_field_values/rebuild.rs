use super::types::*;

pub fn decoded(case: &FieldCase) -> Vec<u8> {
    assert!(case.version >= 5);
    assert!(!case.frame.restriction || case.version <= 7);
    match &case.projection {
        FieldProjection::Legacy {
            table,
            checked,
            objects,
        } => legacy_projection(case, table, *checked, objects),
        FieldProjection::Regions {
            count_checked,
            widths_checked,
            regions,
        } => {
            assert!(case.version >= 7 && *count_checked && *widths_checked);
            region_projection(case, regions)
        }
    }
}

fn legacy_projection(
    case: &FieldCase,
    table: &[i32],
    checked: bool,
    objects: &[FieldObject],
) -> Vec<u8> {
    let FieldContext::Legacy {
        table_count,
        object_widths,
        expected,
    } = &case.frame.context
    else {
        panic!("{} context; expected legacy object layout", case.frame.name);
    };
    assert!([5, 6].contains(&case.version));
    legacy(
        *table_count,
        object_widths,
        expected.as_deref(),
        table,
        checked,
        objects,
    )
}

fn region_projection(case: &FieldCase, regions: &[FieldRegion]) -> Vec<u8> {
    let FieldContext::Regions {
        rows,
        widths,
        active,
    } = &case.frame.context
    else {
        panic!("{} context; expected region layout", case.frame.name);
    };
    regions_body(rows, widths, active, regions)
}

fn legacy(
    table_count: u32,
    widths: &[u32],
    expected: Option<&[Option<u32>]>,
    table: &[i32],
    checked: bool,
    objects: &[FieldObject],
) -> Vec<u8> {
    assert_eq!(table.len(), table_count as usize);
    assert_eq!(objects.len(), widths.len());
    assert_eq!(checked, expected.is_some());
    check_table(table, expected);
    let mut raw: Vec<u8> = table.iter().flat_map(|word| word.to_le_bytes()).collect();
    for (ordinal, object) in objects.iter().enumerate() {
        assert_eq!(object.ordinal as usize, ordinal);
        assert_eq!(object.width, widths[ordinal]);
        assert!(object.width <= i32::MAX as u32);
        assert_eq!(object.bits.len(), object.width as usize);
        assert_eq!(raw.len(), object.start as usize);
        append_wide(&mut raw, &object.bits);
        assert_eq!(raw.len(), object.end as usize);
    }
    raw
}

fn check_table(table: &[i32], expected: Option<&[Option<u32>]>) {
    let Some(expected) = expected else {
        return;
    };
    assert_eq!(table.len(), expected.len());
    for (actual, wanted) in table.iter().zip(expected) {
        if let Some(wanted) = wanted {
            assert!(*wanted <= i32::MAX as u32);
            assert_eq!(*actual, *wanted as i32);
        }
    }
}

fn regions_body(rows: &[u32], widths: &[u32], active: &[bool], regions: &[FieldRegion]) -> Vec<u8> {
    assert_eq!(rows.len(), widths.len());
    assert_eq!(rows.len(), active.len());
    assert_eq!(rows.len(), regions.len());
    let mut raw = (regions.len() as i32).to_le_bytes().to_vec();
    for (ordinal, region) in regions.iter().enumerate() {
        assert_eq!(region.ordinal as usize, ordinal);
        assert_eq!(
            (region.rows, region.width, region.active),
            (rows[ordinal], widths[ordinal], active[ordinal])
        );
        append_region(&mut raw, region);
    }
    raw
}

fn append_region(raw: &mut Vec<u8>, region: &FieldRegion) {
    assert_eq!(raw.len(), region.start as usize);
    assert!(region.rows <= i32::MAX as u32 && region.width <= i32::MAX as u32);
    let count = if region.active {
        u64::from(region.rows) * u64::from(region.width)
    } else {
        0
    };
    assert_eq!(region.bits.len() as u64, count);
    raw.extend_from_slice(&(region.width as i32).to_le_bytes());
    append_wide(raw, &region.bits);
    assert_eq!(raw.len(), region.end as usize);
}

fn append_wide(raw: &mut Vec<u8>, bits: &[u64]) {
    for bits in bits {
        raw.extend_from_slice(&bits.to_le_bytes());
    }
}

pub fn projected_width(case: &WidthCase) -> Option<u32> {
    if !(-128..=127).contains(&case.code) {
        return None;
    }
    if case.code == -1 {
        return nonnegative(case.cached?);
    }
    if case.code != 1 {
        return Some(0);
    }
    let index = case.index?;
    let numbers = case.numeric.as_ref()?;
    if index < 0 || index as usize >= numbers.len() {
        return Some(0);
    }
    match numbers[index as usize] {
        WidthNumber::Signed(value) => nonnegative(value),
        WidthNumber::FloatBits(bits) => truncated_binary64(bits),
    }
}

fn nonnegative(value: i64) -> Option<u32> {
    (0..=i64::from(i32::MAX))
        .contains(&value)
        .then_some(value as u32)
}

fn truncated_binary64(bits: u64) -> Option<u32> {
    let exponent = ((bits >> 52) & 2047) as u32;
    if exponent < 1023 {
        return Some(0);
    }
    if exponent > 1053 || bits >> 63 != 0 {
        return None;
    }
    let significand = (1_u64 << 52) | (bits & ((1_u64 << 52) - 1));
    Some((significand >> (1075 - exponent)) as u32)
}
