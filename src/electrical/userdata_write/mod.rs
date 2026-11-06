mod types;
use super::{
    ElectricalArchive, ElectricalArchiveEdit, ElectricalEditCompletion, ElectricalMemberId,
    ElectricalMemberRelation, ElectricalUserdataKey,
};
use crate::sqlite_image::{SqliteLimits, SqliteUserdataMutation};
use crate::{BessemerError, FailureClass, Result};
pub use types::*;

impl ElectricalArchive {
    /// Commits one unique userdata TEXT assignment on an isolated copy; e.g. `let edit = archive.edit_userdata_text(&member, key, slot, "value", limits)?`.
    pub fn edit_userdata_text(
        &self,
        member: &ElectricalMemberId,
        key: ElectricalUserdataKey,
        slot: ElectricalUserdataSlot,
        text: &str,
        limits: SqliteLimits,
    ) -> Result<ElectricalUserdataWrite> {
        let source = self.sqlite_member(member, limits).map_err(write_error)?;
        let mutation = source
            .edit_electrical_userdata([key.object_id, key.object_no], slot.0, text)
            .map_err(write_error)?;
        let carrier = self
            .edit_member(member, mutation.image.as_bytes())
            .map_err(write_error)?;
        publish_userdata_write(carrier, member, key, slot, mutation)
    }
}

fn publish_userdata_write(
    carrier: ElectricalArchiveEdit,
    member: &ElectricalMemberId,
    key: ElectricalUserdataKey,
    slot: ElectricalUserdataSlot,
    mutation: SqliteUserdataMutation,
) -> Result<ElectricalUserdataWrite> {
    let coverage = write_coverage(&mutation);
    let history = cell_write_history(&carrier, member, key, slot, mutation.before, mutation.after)?;
    Ok(ElectricalUserdataWrite {
        carrier,
        history,
        coverage,
    })
}

fn write_coverage(mutation: &SqliteUserdataMutation) -> ElectricalUserdataWriteCoverage {
    ElectricalUserdataWriteCoverage {
        mode: ElectricalUserdataWriteCheckMode::ExactTextAndStoredDatabase,
        checked_tables: mutation.checked_tables,
        checked_rows: mutation.checked_rows,
        checked_cells: mutation.checked_cells,
        accounted_bytes: mutation.accounted_bytes,
        achieved_byte_error: 0,
    }
}

fn cell_write_history(
    carrier: &ElectricalArchiveEdit,
    member: &ElectricalMemberId,
    key: ElectricalUserdataKey,
    slot: ElectricalUserdataSlot,
    before: crate::sqlite_image::SqliteValue,
    after: crate::sqlite_image::SqliteValue,
) -> Result<ElectricalUserdataCellHistory> {
    let mapping = find_member_mapping(carrier, member)?;
    Ok(ElectricalUserdataCellHistory {
        source_member: member.clone(),
        result_member: mapping.result.clone(),
        key,
        slot,
        before,
        after,
        relation: mapping.relation,
    })
}

fn find_member_mapping<'a>(
    carrier: &'a ElectricalArchiveEdit,
    member: &ElectricalMemberId,
) -> Result<&'a super::ElectricalMemberHistory> {
    carrier
        .history()
        .iter()
        .find(|entry| entry.parent == *member)
        .ok_or_else(|| {
            BessemerError::new(
                "Electrical userdata write",
                "member history",
                FailureClass::Missing,
                "source member absent; expected complete source/result occurrence relation",
            )
        })
}

impl ElectricalUserdataWrite {
    /// Borrows the exact immutable source checkpoint; e.g. `let original = edit.source().to_bytes()`.
    pub fn source(&self) -> &ElectricalArchive {
        self.carrier.source()
    }
    /// Borrows the committed and revalidated carrier; e.g. `let output = edit.result().to_bytes()`.
    pub fn result(&self) -> &ElectricalArchive {
        self.carrier.result()
    }
    /// Reports complete changed versus byte-identical output; e.g. `let status = edit.completion()`.
    pub fn completion(&self) -> ElectricalEditCompletion {
        self.carrier.completion()
    }
    /// Borrows complete composed ZIP member history/checkpoint coverage; e.g. `let members = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &ElectricalArchiveEdit {
        &self.carrier
    }
    /// Reports full stored-image/text-assignment checks; e.g. `let checked = edit.coverage()`.
    pub fn coverage(&self) -> ElectricalUserdataWriteCoverage {
        self.coverage
    }
    /// Borrows exact source/result slot provenance; e.g. `let cell = edit.history()`.
    pub fn history(&self) -> &ElectricalUserdataCellHistory {
        &self.history
    }
    /// Borrows physical/application exclusions; e.g. `let excluded = edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[ElectricalUserdataWriteDiagnostic] {
        &[
            ElectricalUserdataWriteDiagnostic::PhysicalImageMayBeRewritten,
            ElectricalUserdataWriteDiagnostic::NativeProviderConversionsNotApplied,
            ElectricalUserdataWriteDiagnostic::NativeApplicationNotChecked,
            ElectricalUserdataWriteDiagnostic::FreshReverseWireIdentityNotGuaranteed,
        ]
    }
}

fn write_error(mut error: BessemerError) -> BessemerError {
    error.operation = "Electrical userdata write";
    error
}
