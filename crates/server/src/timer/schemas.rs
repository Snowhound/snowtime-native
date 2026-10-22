use crate::entries::schemas::Entry;
pub use crate::inputs::timer::*;
use crate::schemas::*;

#[derive(Serialize)]
pub struct RunningTimerProject {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Serialize)]
pub struct RunningTimer {
    #[serde(flatten)]
    pub entry: Entry,
    pub project: Option<RunningTimerProject>,
}

#[derive(Serialize)]
pub struct StartedTimer {
    pub started: Entry,
    pub stopped: Option<Entry>,
}
