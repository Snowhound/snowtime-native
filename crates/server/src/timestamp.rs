//! A moment in milliseconds since the epoch, as the schema's timestamp_ms columns store it.
//! It travels as an ISO 8601 string, as `Date.toJSON` writes one and valibot's revived
//! `v.date()` reads one (src/lib/api/wire.ts).
use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub i64);

const DAY_MS: i64 = 86_400_000;

// Days since 1970-01-01 of a proleptic Gregorian date (Howard Hinnant's algorithm).
pub(crate) fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

pub(crate) fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(month <= 2), month, day)
}

pub(crate) fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 31,
    }
}

impl Timestamp {
    /// `Date.toISOString()`: 2026-09-30T07:30:00.000Z.
    pub fn to_iso(self) -> String {
        let days = self.0.div_euclid(DAY_MS);
        let ms = self.0.rem_euclid(DAY_MS);
        let (year, month, day) = civil_from_days(days);
        format!(
            "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
            ms / 3_600_000,
            ms / 60_000 % 60,
            ms / 1000 % 60,
            ms % 1000
        )
    }

    /// The time `new Date(text)` gives in the app's process, for the dates its transport
    /// revives: V8's `Date.parse` (the generated `inputs::js_date`, task 081.49), its legacy
    /// forms included, with times without a zone read in this process's time zone.
    pub fn parse(text: &str) -> Option<Timestamp> {
        crate::inputs::js_date::date_parse(text, local_to_utc).map(Timestamp)
    }
}

// A local time, as milliseconds since 1970 read in this process's time zone, in UTC.
// Node takes the offset from before a DST change for a time the change skips or repeats;
// mktime picks by the platform's rules, which matter only outside UTC.
fn local_to_utc(local: i64) -> i64 {
    let seconds = local.div_euclid(1000) as libc::time_t;
    // SAFETY: tm is a C record whose fields are all valid when zeroed.
    let mut fields: libc::tm = unsafe { std::mem::zeroed() };
    // SAFETY: gmtime_r and mktime borrow the values for the call only.
    let utc = unsafe {
        if libc::gmtime_r(&seconds, &mut fields).is_null() {
            return local;
        }
        fields.tm_isdst = -1;
        libc::mktime(&mut fields)
    };
    utc as i64 * 1000 + local.rem_euclid(1000)
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_iso())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        Timestamp::parse(&text).ok_or_else(|| serde::de::Error::custom("Invalid date"))
    }
}

impl ToSql for Timestamp {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

// The app writes integers, but a REAL in the column reads as Drizzle reads it: `new Date`
// truncates it toward zero. perf/stress/dataset.ts writes some.
impl FromSql for Timestamp {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Real(ms) => Ok(Timestamp(ms.trunc() as i64)),
            _ => i64::column_result(value).map(Timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn round_trips() {
        let seed = Timestamp::parse("2026-09-30T07:30:00.000Z").unwrap();
        assert_eq!(seed.0, 1_790_753_400_000);
        assert_eq!(seed.to_iso(), "2026-09-30T07:30:00.000Z");
        assert_eq!(Timestamp(0).to_iso(), "1970-01-01T00:00:00.000Z");
        assert_eq!(Timestamp(-1).to_iso(), "1969-12-31T23:59:59.999Z");
    }

    #[test]
    fn reads_what_date_reads() {
        assert_eq!(
            Timestamp::parse("2026-09-30").unwrap().to_iso(),
            "2026-09-30T00:00:00.000Z"
        );
        assert_eq!(
            Timestamp::parse("2026-09-30T10:30+03:00").unwrap().to_iso(),
            "2026-09-30T07:30:00.000Z"
        );
        assert_eq!(
            Timestamp::parse("2026-09-30T07:30:00.12345Z")
                .unwrap()
                .to_iso(),
            "2026-09-30T07:30:00.123Z"
        );
        // Date rolls February 30 over, and reads the legacy forms.
        assert_eq!(
            Timestamp::parse("2026-02-30T00:00:00Z").unwrap().to_iso(),
            "2026-03-02T00:00:00.000Z"
        );
        assert_eq!(
            Timestamp::parse("Oct 22 2026 10:00 GMT+2")
                .unwrap()
                .to_iso(),
            "2026-10-22T08:00:00.000Z"
        );
        assert!(Timestamp::parse("yesterday").is_none());
    }
}
