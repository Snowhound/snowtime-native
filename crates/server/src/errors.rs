//! Errors the rules return on purpose (src/server/errors.ts). The code tells the client what
//! went wrong; the key names the message, which the client shows in the user's language.
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Code {
    Unauthenticated,
    Forbidden,
    NotFound,
    Conflict,
    Invalid,
    LimitReached,
    RateLimited,
    Unavailable,
}

impl Code {
    // statusOf in src/server/http.server.ts.
    pub fn status(self) -> u16 {
        match self {
            Code::Unauthenticated => 401,
            Code::Forbidden => 403,
            Code::NotFound => 404,
            Code::Conflict => 409,
            Code::Invalid | Code::LimitReached => 422,
            Code::RateLimited => 429,
            Code::Unavailable => 503,
        }
    }
}

// The keys this slice of the contract uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    SignInRequired,
    NotOrganizationMember,
    MemberNotFound,
    EntryNotFound,
    EntryIdTaken,
    EntryForbidden,
    EntriesForbidden,
    EntryRunning,
    EntryEndBeforeStart,
    EntryTooLong,
    EntryLimit,
    TimerNotRunning,
    TimerStartedElsewhere,
    TimerRunningInLeftOrganization,
    ProjectNotFound,
    ProjectArchived,
    ProjectsForbidden,
    ProjectLimit,
    ProjectIdTaken,
    ProjectNameTaken,
    ProjectHasEntries,
    TeamNotFound,
    TeamsForbidden,
    TeamNameTaken,
    TeamLimit,
    TeamMemberNotFound,
    TeamReportForbidden,
    SettingsNotFound,
    ScenePinNotInCollection,
    OrganizationForbidden,
    ApiKeyLimit,
    ApiKeyNotFound,
    RateLimited,
    DatabaseUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct AppError {
    pub code: Code,
    pub key: Key,
}

/// Input its schema refuses, as validationRefusal in src/server/http.server.ts answers it:
/// the message's key and parameter, which the client translates, and the message in English.
#[derive(Debug, Serialize)]
pub struct Invalid {
    pub key: &'static str,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "params")]
    pub params: Option<(&'static str, Param)>,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Param {
    Number(i64),
    Text(&'static str),
}

fn params<S: serde::Serializer>(
    value: &Option<(&'static str, Param)>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(1))?;
    if let Some((name, param)) = value {
        map.serialize_entry(name, param)?;
    }
    map.end()
}

/// How a call fails: on purpose, on input its schema refuses (valibot's ValiError), on a
/// body that isn't JSON, or unexpectedly.
#[derive(Debug)]
pub enum Error {
    App(AppError),
    Invalid(Invalid),
    NotJson,
    Database(rusqlite::Error),
    Auth {
        status: u16,
        code: &'static str,
        message: &'static str,
    },
}

impl From<AppError> for Error {
    fn from(error: AppError) -> Self {
        Error::App(error)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Database(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// `throw new AppError(code, key)`.
pub fn refuse<T>(code: Code, key: Key) -> Result<T> {
    Err(Error::App(AppError { code, key }))
}
