use crate::{BessemerError, FailureClass, Result};

pub(super) fn validate_triangle_group_snapshot(
    operation: &'static str,
    count: usize,
    available: usize,
    maximum: usize,
) -> Result<()> {
    if count > maximum {
        return Err(BessemerError::new(
            operation,
            "group_count",
            FailureClass::Invalid,
            format!("group count {count}; expected at most maximum_groups {maximum}"),
        ));
    }
    if count > available {
        return Err(BessemerError::new(
            operation,
            "group snapshots",
            FailureClass::Missing,
            format!(
                "{available} provided groups; expected at least {count} complete group snapshots"
            ),
        ));
    }
    Ok(())
}
