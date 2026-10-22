pub use crate::inputs::entries::*;
use crate::schemas::*;

// Hooks of the generated validators (crate::inputs::entries).
pub(crate) fn end_before_start(input: &CreateEntryInput) -> bool {
    input.stopped_at > input.started_at
}
pub(crate) fn entry_too_long(input: &CreateEntryInput) -> bool {
    input.stopped_at.0 - input.started_at.0 <= MAX_ENTRY_MS
}
pub(crate) fn list_entries_input_range_end_before_start(input: &ListEntriesInput) -> bool {
    input.to > input.from
}
pub(crate) fn list_entries_input_range_too_long(input: &ListEntriesInput) -> bool {
    input.to.0 - input.from.0 <= MAX_LIST_DAYS * 86_400_000
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: String,
    pub organization_id: String,
    pub user_id: String,
    pub project_id: Option<String>,
    pub description: String,
    pub ticket: Option<String>,
    pub started_at: Timestamp,
    pub stopped_at: Option<Timestamp>,
}

#[derive(Serialize)]
pub struct DeletedEntry {
    pub id: String,
}
