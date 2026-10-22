use crate::calendar::{WeekStart, Zone};
pub use crate::inputs::settings::*;
use crate::schemas::*;

pub const COLLECTIONS: &[&str] = &["mountains", "countryside", "coast"];
pub const IMAGES: &[&str] = &[
    "winter",
    "spring",
    "summer",
    "autumn",
    "land-january",
    "land-february",
    "land-march",
    "land-april",
    "land-may",
    "land-june",
    "land-july",
    "land-august",
    "land-september",
    "land-october",
    "land-november",
    "land-december",
    "coast-january",
    "coast-february",
    "coast-march",
    "coast-april",
    "coast-may",
    "coast-june",
    "coast-july",
    "coast-august",
    "coast-september",
    "coast-october",
    "coast-november",
    "coast-december",
];

// Hooks of the generated validators (crate::inputs::settings). ICU rejects tzdb's
// placeholder Factory zone, which jiff can read.
pub(crate) fn time_zone_unknown(name: &str) -> bool {
    !name.eq_ignore_ascii_case("Factory") && Zone::get(name).is_some()
}

// validDurationPattern (src/lib/duration-pattern-settings.ts): at most 40 UTF-16 units, with
// H, M, or S outside a backslash escape.
pub(crate) fn copy_duration_pattern(pattern: &str) -> bool {
    let mut chars = pattern.chars();
    let mut field = false;
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                chars.next();
            }
            'H' | 'M' | 'S' => field = true,
            _ => {}
        }
    }
    field && pattern.encode_utf16().count() <= 40
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn field_checks_keep_valibots_order_and_boolean_revival() {
        let result = decode::<UpdateSettingsInput>(
            json!({"country": "FI", "locale": "bad", "timeZone": "Mars/Olympus"}),
        );
        assert!(
            matches!(result, Err(Error::Invalid(invalid)) if invalid.message == "Unknown time zone.")
        );
        let patch: UpdateSettingsInput = decode(json!({"showSummary": "false", "sceneWeather": "true", "scenePin": null, "country": null})).unwrap();
        assert_eq!(patch.show_summary, Some(false));
        assert_eq!(patch.scene_weather, Some(true));
        assert_eq!(patch.scene_pin, Patch::Null);
        assert_eq!(patch.country, Patch::Null);
        assert!(decode::<UpdateSettingsInput>(json!({"showSummary": null})).is_err());
    }

    #[test]
    fn copy_patterns_need_a_field_outside_escapes_in_40_utf16_units() {
        let long = format!("H{}", " ".repeat(39));
        for pattern in ["H:MM:SS", "Hh Mm Ss", "S", "\\HH", "H\\", long.as_str()] {
            assert!(decode::<UpdateSettingsInput>(json!({"copyDurationPattern": pattern})).is_ok());
        }
        let too_long = format!("H{}", " ".repeat(40));
        // 41 UTF-16 units in 21 chars.
        let astral = format!("H{}", "😀".repeat(20));
        for pattern in ["", "hms", "\\H", too_long.as_str(), astral.as_str()] {
            let result = decode::<UpdateSettingsInput>(json!({"copyDurationPattern": pattern}));
            assert!(
                matches!(result, Err(Error::Invalid(invalid)) if invalid.message == "Use H, M, or S, in at most 40 characters."),
                "{pattern}"
            );
        }
        for control in [json!("icon"), json!(""), json!(null)] {
            assert!(
                decode::<UpdateSettingsInput>(json!({"copyDurationControl": control})).is_err()
            );
        }
    }

    #[test]
    fn zones_keep_aliases_and_refuse_offsets_and_the_placeholder_zone() {
        for name in [
            "UTC",
            "utc",
            "US/Eastern",
            "Etc/GMT+2",
            "America/Argentina/Buenos_Aires",
        ] {
            assert!(decode::<CreateSettingsInput>(json!({"timeZone": name})).is_ok());
        }
        for name in [
            "+02:00",
            "",
            "Factory",
            "factory",
            "Z",
            "Mars/Olympus",
            "UTC\n",
        ] {
            assert!(decode::<CreateSettingsInput>(json!({"timeZone": name})).is_err());
        }
    }
}

// The user's settings as the API sends them. The text values were checked when written.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub time_zone: Zone,
    pub week_start: WeekStart,
    pub locale: String,
    pub theme: String,
    pub timer_layout: String,
    pub show_summary: bool,
    pub compact_rows: bool,
    pub wide_timer: bool,
    pub timer_view: String,
    pub calendar_weekend: bool,
    pub app_icon: String,
    pub scene_collection: String,
    pub scene_pin: Option<String>,
    pub scene_background: bool,
    pub scene_strength: String,
    pub surfaces: String,
    pub scene_weather: bool,
    pub scene_intro: bool,
    pub scene_tagline: bool,
    pub duration_format: String,
    pub date_format: String,
    pub time_format: String,
    pub copy_duration_pattern: String,
    pub copy_duration_control: String,
    pub country: Option<String>,
}
