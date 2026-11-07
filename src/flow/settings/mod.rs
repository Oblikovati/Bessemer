//! Static-settings identity and the documented normal-return cache-selection projection.
mod types;
mod view;
use super::*;
pub use types::*;

const NIL_GUID: FlowGuid = FlowGuid::from_bytes([0; 16]);

impl FlowCarrier {
    /// Reads the gated optional identity; e.g. `let identity = carrier.static_settings_identity()?`.
    pub fn static_settings_identity(&self) -> Result<FlowSettingsIdentity> {
        let settings = self.header_metadata()?.and_then(|header| header.settings);
        let (record, guid) = settings.map_or((None, NIL_GUID), |field| {
            (Some(field.record), field.cfld_uuid)
        });
        Ok(FlowSettingsIdentity {
            source: self.clone(),
            record,
            guid,
        })
    }
}

/// Projects native cache selection without executing loaders; e.g. `let plan = select_flow_static_settings(incoming, cached, 0)`.
pub fn select_flow_static_settings(
    incoming: FlowGuid,
    cached: FlowGuid,
    from_fld: u8,
) -> FlowSettingsSelection {
    let action = selection_action(incoming, cached, from_fld);
    let load_required = action == FlowSettingsAction::InvokeLoaderSequence;
    FlowSettingsSelection {
        incoming,
        cached_before: cached,
        from_fld,
        action,
        cached_after_normal_return: if load_required { incoming } else { cached },
        overwrite_warning: load_required && cached != NIL_GUID,
    }
}

fn selection_action(incoming: FlowGuid, cached: FlowGuid, from_fld: u8) -> FlowSettingsAction {
    if from_fld != 0 {
        return FlowSettingsAction::RetainFldSettings;
    }
    if incoming == cached {
        return FlowSettingsAction::SkipCachedIdentity;
    }
    FlowSettingsAction::InvokeLoaderSequence
}
