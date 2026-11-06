use super::*;
/// Explicit modern mode assumption and independent physical resource bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMessageListOptions {
    /// Required for versions zero/two/three, ignored for version one.
    pub string_mode: Option<FlowMetadataStringMode>,
    /// Complete raw payload bound, at least eight bytes.
    pub maximum_payload_bytes: usize,
    /// Actual item descriptors.
    pub maximum_items: usize,
    /// Actual string descriptors, including primary and empty strings.
    pub maximum_strings: usize,
    /// Aggregate transferred text bytes; modern units cost two bytes each.
    pub maximum_text_bytes: usize,
    /// All stored scalars, including framing and text elements.
    pub maximum_values: usize,
    /// Root, item, list and string scopes, including empty scopes.
    pub maximum_scopes: usize,
}
/// Immutable complete message payload under a normalized text representation.
#[derive(Clone, Debug)]
pub struct FlowMessageLists {
    pub(super) wire: TableWire,
    pub(super) options: FlowMessageListOptions,
    pub(super) encoding: FlowMessageTextEncoding,
    pub(super) items: Arc<[MessageItemSpan]>,
    pub(super) texts: Arc<[MessageTextSpan]>,
    pub(super) totals: [usize; 3],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct MessageItemSpan {
    pub interval: (usize, usize),
    pub primary: usize,
    pub first: MessageListSpan,
    pub second: Option<MessageListSpan>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct MessageListSpan {
    pub interval: (usize, usize),
    pub texts: (usize, usize),
}
#[derive(Clone, Copy, Debug)]
pub(super) struct MessageTextSpan {
    pub role: FlowMessageTextRole,
    pub interval: (usize, usize),
    pub count: usize,
    pub prefix_units: Option<usize>,
}
/// Borrowed complete item, preserving both independent list counts and second-list absence.
#[derive(Clone, Copy, Debug)]
pub struct FlowMessageItem<'a> {
    pub(super) owner: &'a FlowMessageLists,
    pub(super) ordinal: usize,
    pub(super) span: &'a MessageItemSpan,
}
/// Borrowed complete counted list, including stored-empty lists.
#[derive(Clone, Copy, Debug)]
pub struct FlowMessageStringList<'a> {
    pub(super) owner: &'a FlowMessageLists,
    pub(super) span: &'a MessageListSpan,
}
/// Borrowed lossless string, with a prefix projection only for modern UTF16.
#[derive(Clone, Copy, Debug)]
pub struct FlowMessageText<'a> {
    pub(super) owner: &'a FlowMessageLists,
    pub(super) ordinal: usize,
    pub(super) span: &'a MessageTextSpan,
}
/// Text identity bound to its selected occurrence and source revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowMessageTextId {
    pub(super) record: FlowRecordId,
    pub(super) ordinal: usize,
}
/// Exact raw framing and actual stored-value accounting under the selected representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMessageCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Effective stored text representation, determined once from the version.
    pub encoding: FlowMessageTextEncoding,
    /// Applied modern assumption; None for the legacy branch even if context was supplied.
    pub applied_string_mode: Option<FlowMetadataStringMode>,
    /// Complete payload bytes.
    pub payload_bytes: usize,
    /// Physical item count.
    pub items: usize,
    /// All primary and list strings, including empty strings.
    pub strings: usize,
    /// Actual transferred text bytes.
    pub text_bytes: usize,
    /// All stored scalars, including framing, members, counts and text elements.
    pub stored_values: usize,
    /// Root, item, list and string scopes.
    pub scopes: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Completed isolated edit with immutable typed checkpoints and complete physical history.
#[derive(Clone, Debug)]
pub struct FlowMessageListEdit {
    pub(super) source: FlowMessageLists,
    pub(super) result: FlowMessageLists,
    pub(super) carrier_edit: FlowCarrierEdit,
}
