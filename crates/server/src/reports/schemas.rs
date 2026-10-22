use crate::calendar::{WeekStart, Zone};
pub use crate::inputs::reports::*;
use crate::schemas::*;

// Longest range a report covers: a year of weeks.
pub const MAX_REPORT_DAYS: i64 = 371;
// Longest piece of an export: a calendar month.
const MAX_EXPORT_PIECE_DAYS: i64 = 31;

// Hooks of the generated validators (crate::inputs::reports). IsoDate's form is checked
// first, so a day that doesn't parse is one the month lacks.
pub(crate) fn date_unknown(text: &str) -> bool {
    Day::parse(text).is_some()
}
pub(crate) fn report_input_range_end_before_start(input: &ReportInput) -> bool {
    input.to > input.from
}
pub(crate) fn report_input_range_too_long(input: &ReportInput) -> bool {
    input.to.0 - input.from.0 <= MAX_REPORT_DAYS
}
pub(crate) fn member_or_team(input: &ReportInput) -> bool {
    !(input.user_id.is_some() && input.team_id.is_some())
}
pub(crate) fn report_export_input_range_end_before_start(input: &ReportExportInput) -> bool {
    input.to > input.from
}
// The piece lies inside the report's range.
pub(crate) fn report_export_input_check(input: &ReportExportInput) -> bool {
    input.from >= input.report.from && input.to <= input.report.to
}
pub(crate) fn report_export_input_range_too_long(input: &ReportExportInput) -> bool {
    input.to.0 - input.from.0 <= MAX_EXPORT_PIECE_DAYS
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Unit {
    #[default]
    Day,
    Week,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Totals {
    pub total: i64,
    // Milliseconds per bucket, in the order of Report.buckets.
    pub per_bucket: Vec<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRow {
    pub project_id: Option<String>,
    #[serde(flatten)]
    pub totals: Totals,
}

#[derive(Serialize)]
pub struct TicketRow {
    pub ticket: Option<String>,
    #[serde(flatten)]
    pub totals: Totals,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberRow {
    pub user_id: String,
    #[serde(flatten)]
    pub totals: Totals,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamRow {
    pub team_id: String,
    #[serde(flatten)]
    pub totals: Totals,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormerMember {
    pub user_id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    #[serde(flatten)]
    pub totals: Totals,
    pub buckets: Vec<Day>,
    pub tracked_days: usize,
    pub entries: usize,
    pub projects: Vec<ProjectRow>,
    pub tickets: Vec<TicketRow>,
    pub members: Vec<MemberRow>,
    pub teams: Vec<TeamRow>,
    pub time_zone: Zone,
    pub week_start: WeekStart,
    pub unit: Unit,
    pub from: Timestamp,
    pub to: Timestamp,
    pub now: Timestamp,
    pub former_members: Vec<FormerMember>,
}

pub const ENTRY_PAGE_SIZE: usize = 100;
pub const DESCRIPTION_PAGE_SIZE: usize = 25;

#[derive(Clone, Deserialize)]
pub struct EntryRow {
    pub group: String,
    pub id: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayCursor {
    pub date: Day,
    pub user_id: String,
    #[serde(serialize_with = "cursor_from")]
    pub from: f64,
    pub entry_id: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectBreakdown {
    pub project_id: Option<String>,
    pub user_id: String,
    pub total: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketBreakdown {
    pub ticket: Option<String>,
    pub user_id: String,
    pub total: i64,
}
#[derive(Serialize)]
pub struct ReportBreakdown {
    pub projects: Vec<ProjectBreakdown>,
    pub tickets: Vec<TicketBreakdown>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportEntryPiece {
    pub entry_id: String,
    pub user_id: String,
    pub project_id: Option<String>,
    pub description: String,
    pub ticket: Option<String>,
    pub date: Day,
    pub from: Timestamp,
    pub to: Timestamp,
    pub started_at: Timestamp,
    pub stopped_at: Option<Timestamp>,
    pub running: bool,
    pub ms: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionRow {
    pub project_id: Option<String>,
    pub ticket: Option<String>,
    pub description: String,
    pub total: i64,
    pub entries: usize,
    pub days: usize,
    pub user_ids: Vec<String>,
}
#[derive(Serialize)]
pub struct DayTotal {
    pub date: Day,
    pub total: i64,
}
#[derive(Serialize)]
#[serde(tag = "view", rename_all = "lowercase")]
pub enum ReportEntries {
    Day {
        days: Vec<DayTotal>,
        pieces: Vec<ReportEntryPiece>,
        next: Option<DayCursor>,
    },
    Description {
        rows: Vec<DescriptionRow>,
        #[serde(rename = "rowCount")]
        row_count: usize,
        next: Option<usize>,
    },
}
#[derive(Serialize)]
pub struct ReportEntryTotals {
    pub count: usize,
    pub total: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportEntry {
    pub user_id: String,
    pub project_id: Option<String>,
    pub description: String,
    pub ticket: Option<String>,
    pub date: Day,
    pub from: Timestamp,
    pub running: bool,
    pub ms: i64,
}
#[derive(Serialize)]
pub struct ReportExport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<Report>,
    pub entries: Vec<ExportEntry>,
}

fn cursor_from<S: serde::Serializer>(
    from: &f64,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    if from.fract() == 0.0 {
        serializer.serialize_i64(*from as i64)
    } else {
        serializer.serialize_f64(*from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validates_nested_filters_before_list_fields_and_export_checks() {
        let error = decode::<ReportEntriesInput>(json!({
            "report": { "from": "2026-09-28", "to": "2026-09-21" },
            "view": "bad",
        }))
        .err()
        .unwrap();
        assert!(
            matches!(error, Error::Invalid(invalid) if invalid.message == "The range must end after it starts.")
        );
        let error = decode::<ReportExportInput>(json!({
            "report": { "from": "2026-09-21", "to": "2026-09-28" },
            "from": "2026-09-20", "to": "2026-09-24",
        }))
        .err()
        .unwrap();
        assert!(
            matches!(error, Error::Invalid(invalid) if invalid.message == "Invalid input: Received Object")
        );
        let error = decode::<ReportEntryTotalsInput>(json!({
            "report": { "from": "2026-09-21", "to": "2026-09-28" },
            "row": {},
        }))
        .err()
        .unwrap();
        assert!(
            matches!(error, Error::Invalid(invalid) if invalid.message == r#"Invalid type: Expected ("project" | "team" | "member" | "ticket") but received undefined"#)
        );
    }

    #[test]
    fn serializes_cursor_milliseconds_like_javascript_numbers() {
        let cursor = DayCursor {
            date: Day::parse("2026-09-21").unwrap(),
            user_id: "u".into(),
            from: 123.0,
            entry_id: "e".into(),
        };
        assert_eq!(
            serde_json::to_string(&cursor).unwrap(),
            r#"{"date":"2026-09-21","userId":"u","from":123,"entryId":"e"}"#
        );
    }
}
