use super::*;

impl FlowSettingsIdentity {
    /// Borrows the original complete carrier; e.g. `let checkpoint = identity.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the gated record occurrence, distinct from an orphan 435; e.g. `let field = identity.record()`.
    pub fn record(&self) -> Option<&FlowRecordId> {
        self.record.as_ref()
    }
    /// Reports physical field presence, independently of nil bytes; e.g. `let stored = identity.present()`.
    pub fn present(&self) -> bool {
        self.record.is_some()
    }
    /// Returns exact stored bytes or the documented absent-field nil default; e.g. `let key = identity.guid()`.
    pub fn guid(&self) -> FlowGuid {
        self.guid
    }
    /// Reports the optional identity-field checks; e.g. `let checked = identity.coverage()`.
    pub fn coverage(&self) -> FlowSettingsIdentityCoverage {
        FlowSettingsIdentityCoverage {
            mode: FlowSettingsCheckMode::ExactOptionalHeaderIdentity,
            identity_bytes: if self.present() { 16 } else { 0 },
            achieved_byte_error: 0,
        }
    }
    /// Borrows native acceptance exclusions; e.g. `let exclusions = identity.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSettingsDiagnostic] {
        &[FlowSettingsDiagnostic::NativeApplicationNotChecked]
    }
}

impl FlowSettingsSelection {
    /// Returns the exact requested identity; e.g. `let key = plan.incoming()`.
    pub fn incoming(&self) -> FlowGuid {
        self.incoming
    }
    /// Returns the untouched input cache identity; e.g. `let old = plan.cached_before()`.
    pub fn cached_before(&self) -> FlowGuid {
        self.cached_before
    }
    /// Returns the unchanged runtime byte; e.g. `assert_eq!(plan.source_precedence_byte(), 255)`.
    pub fn source_precedence_byte(&self) -> u8 {
        self.from_fld
    }
    /// Returns the selected native branch; e.g. `let action = plan.action()`.
    pub fn action(&self) -> FlowSettingsAction {
        self.action
    }
    /// Reports required loader invocation rather than loader success; e.g. `let needed = plan.load_required()`.
    pub fn load_required(&self) -> bool {
        self.action == FlowSettingsAction::InvokeLoaderSequence
    }
    /// Returns the cache bytes after hypothetical normal loader return; e.g. `let proposed = plan.cached_after_loaders_return()`.
    pub fn cached_after_loaders_return(&self) -> FlowGuid {
        self.cached_after_normal_return
    }
    /// Reports the documented non-nil overwrite diagnostic; e.g. `let warn = plan.overwrite_warning()`.
    pub fn overwrite_warning(&self) -> bool {
        self.overwrite_warning
    }
    /// Reports exact projection coverage; e.g. `let checked = plan.coverage()`.
    pub fn coverage(&self) -> FlowSettingsSelectionCoverage {
        FlowSettingsSelectionCoverage {
            mode: FlowSettingsCheckMode::ExactNormalReturnSelection,
            guid_equality_considered: self.from_fld == 0,
            achieved_byte_error: 0,
        }
    }
    /// Borrows explicit execution exclusions; e.g. `let exclusions = plan.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSettingsDiagnostic] {
        &[
            FlowSettingsDiagnostic::LoaderSequenceNotExecuted,
            FlowSettingsDiagnostic::NativeApplicationNotChecked,
        ]
    }
}
