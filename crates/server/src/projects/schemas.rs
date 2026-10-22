pub use crate::inputs::projects::*;
use crate::schemas::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub archived_at: Option<Timestamp>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListedProject {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub archived_at: Option<Timestamp>,
    pub has_entries: bool,
    pub team_ids: Vec<String>,
}
