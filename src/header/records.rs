use crate::Result;
use crate::archive_codec::ArchiveString;

use super::fields::{
    FieldRole, FieldRule, FieldType, FieldValue, FieldView, Phase, field_optional, field_type,
    field_view, optional, record, required,
};
use super::{HeaderBounds, HeaderReference, SharedFileName, invalid};

record! {
    /// Complete `moHeader_c` body; optional fields retain their version-dependent presence.
    HeaderRecord {
        /// Raw saved-model state before any embedded version override.
        saved_while_model_exists: optional i32 => (Outer, |v| v > 0x371b, Any);
        /// Stored user array, including any embedded-version marker.
        users: required HeaderReference => (Outer, |_| true, Strings);
        /// Company-name array.
        companies: required HeaderReference => (Body, |_| true, Strings);
        /// Revision-log list.
        revision_logs: required HeaderReference => (Body, |_| true, RevisionLogs);
        /// Uninterpreted stored time word.
        unknown_50: optional u32 => (Body, |v| v >= 0xf1, Any);
        /// Next identifier allocated by the native document.
        next_id: required i32 => (Body, |_| true, Any);
        /// External-reference list.
        external_references: required HeaderReference => (Body, |_| true, ExternalList);
        /// Stored update stamp.
        update_stamp: optional u32 => (Body, |v| v > 0x96, Any);
        /// Document's self-reference.
        self_reference: optional HeaderReference => (Body, |v| v > 0xd8, External);
        /// Stored body stamp.
        body_stamp: optional u32 => (Body, |v| v > 0x30b, Any);
        /// Raw native member +0x58, including values the native reader discards.
        unknown_58: optional i32 => (Body, |v| v > 0x4e4, Any);
        /// Raw native member +0x68.
        unknown_68: optional i32 => (Body, |v| v > 0x500, Any);
        /// Stored creation time.
        creation_time: optional u32 => (Body, |v| v >= 0x506, Any);
        /// Raw native member +0x60.
        unknown_60: optional i32 => (Body, |v| v >= 0x531, Any);
        /// Raw header-use code; its complete enum is unknown.
        header_usage: optional i32 => (Outer, |v| v > 0x859, Any);
        /// Raw native member +0xC0.
        unknown_c0: optional i32 => (Outer, |v| v > 0x91e, Any);
        /// Linked attachment names.
        linked_attachments: optional HeaderReference => (Outer, |v| v > 0xaa1, Strings);
        /// Stored component count, not necessarily inserted-component count.
        component_count: optional i32 => (Outer, |v| v > 0xb8c, Any);
        /// Uninterpreted license flags.
        license_flags: optional u32 => (Outer, |v| v > 0xcf6, Any);
        /// Stored update time stamp.
        update_time_stamp: optional u32 => (Outer, |v| v > 0xe5c, Any);
        /// Replaced-component list.
        replaced_components: optional HeaderReference => (Outer, |v| v > 0xf3a, ObjectList);
        /// Raw copied-document state.
        is_copied: optional i32 => (Outer, |v| v > 0x128f, Any);
        /// Raw native member +0x64.
        unknown_64: optional i32 => (Outer, |v| v >= 0x1304, Any);
        /// Raw box-presence word and conditional ten-double body.
        bounding_box: optional HeaderBounds => (Outer, |v| v > 0x130e, Any);
        /// Stored last-open values; units are unverified.
        last_open_times: optional [i32; 3] => (Outer, |v| v > 0x2783, Any);
        /// Raw license-type enum.
        license_type: optional u32 => (Outer, |v| v > 0x2bc2, Any);
        /// Native members +0x9C through +0xB0 in order.
        active_content: optional [i32; 6] => (Outer, |v| v > 0x2fa3, Any);
        /// Raw 3DEXPERIENCE icon override.
        icon_3dx_override: optional i32 => (Outer, |v| v > 0x336f, Any);
        /// Native members +0xF0 through +0x104 in order.
        external_reference_flags: optional [i32; 6] => (Outer, |v| v > 0x3b0a, Any);
        /// Raw annotation state.
        has_annotation: optional i32 => (Outer, |v| v > 0x3b1f, Any);
        /// Raw graphics-feature state.
        has_graphics_feature: optional i32 => (Outer, |v| v > 0x3db8, Any);
        /// Raw native member +0x98.
        unknown_98: optional i32 => (Outer, |v| v > 0x3f01, Any);
    }
}

record! {
    /// Modern external-object body, including its direct shared-file definition.
    ExternalFileReference {
        /// Stored shared-file path representation.
        path: required SharedFileName => (Outer, |_| true, Handle);
        /// Stored shared-file name representation.
        name: required SharedFileName => (Outer, |_| true, Handle);
        /// Raw shared-file member +0x20.
        shared_unknown_20: required u16 => (Outer, |_| true, Any);
        /// Raw shared-file member +0x08.
        shared_unknown_08: required u8 => (Outer, |_| true, Any);
        /// Shared-file timestamp.
        shared_timestamp: required u32 => (Outer, |_| true, Any);
        /// Raw byte present only in the documented narrow version interval.
        shared_unknown_34: optional u8 => (Outer, |v| v > 0x1427 && v < 0x146c, Any);
        /// Uninterpreted shared-file CString.
        shared_unknown_38: optional ArchiveString => (Outer, |v| v > 0x1fb3, Any);
        /// Imported source path.
        imported_path: optional ArchiveString => (Outer, |v| v > 0x23d9, Any);
        /// Stored PDM identifier text.
        pdm_id: optional ArchiveString => (Outer, |v| v > 0x2f82, Any);
        /// Raw shared-file member +0x35.
        shared_unknown_35: optional u8 => (Outer, |v| v >= 0x3330, Any);
        /// Uninterpreted external-object flags.
        flags: required u8 => (Outer, |_| true, Any);
        /// External-object timestamp.
        timestamp: required u32 => (Outer, |_| true, Any);
        /// Raw external-object member +0x48.
        unknown_48: optional i32 => (Outer, |v| v > 0x62, Any);
        /// Raw external-object member +0x4C.
        unknown_4c: optional i32 => (Outer, |v| v > 0x6c, Any);
        /// Raw external-object member +0x78.
        unknown_78: optional i32 => (Outer, |v| v > 0x8dd, Any);
        /// Referenced configuration's name, without its own object tag.
        configuration_name: optional ArchiveString => (Outer, |v| v > 0x8dd, Any);
        /// Referenced configuration's integer identifier.
        configuration_id: optional i32 => (Outer, |v| v > 0x8dd, Any);
        /// Raw external-object member +0x54.
        unknown_54: optional i32 => (Outer, |v| v > 0xad0, Any);
        /// Stored last-modification time.
        last_modification_time: optional u32 => (Outer, |v| v >= 0xfd2, Any);
        /// Raw external-object member +0x50.
        unknown_50: optional i32 => (Outer, |v| v > 0x3f12, Any);
    }
}

record! {
    /// CString array using MFC ReadCount framing.
    StringArray {
        /// Stored strings, including empty entries and marker strings.
        items: required Vec<ArchiveString> => (Outer, |_| true, Any);
    }
}

record! {
    /// Object-pointer list using the archive-wide map.
    ObjectList {
        /// Null pointers and shared object identities in order.
        items: required Vec<HeaderReference> => (Outer, |_| true, Any);
    }
}

record! {
    /// A revision-log list with its selector and uninterpreted string.
    LogRecord {
        /// Revision stamp objects.
        items: required Vec<HeaderReference> => (Outer, |_| true, Stamp);
        /// Revision-log selector.
        id: required i32 => (Outer, |_| true, Any);
        /// Raw native member +0x50.
        unknown_50: required ArchiveString => (Outer, |_| true, Any);
    }
}
