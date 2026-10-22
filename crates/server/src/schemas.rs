//! Input decoding and tri-state patches. The input structs and their validators are
//! generated from the app's valibot schemas (crate::inputs, schema-gen.json); domain
//! outputs and the hooks the generated code calls live in their own schemas.rs modules.
use serde_json::Value;

pub(crate) use crate::calendar::Day;
pub(crate) use crate::errors::{Error, Invalid, Param, Result};
pub use crate::inputs::runtime::{Patch, Schema, js_whitespace};
pub(crate) use crate::timestamp::Timestamp;
pub(crate) use serde::{Deserialize, Serialize};

// The longest an entry runs (MAX_ENTRY_HOURS).
pub const MAX_ENTRY_HOURS: i64 = 24;
pub const MAX_ENTRY_MS: i64 = MAX_ENTRY_HOURS * 3_600_000;
// Longest range listEntries returns.
pub(crate) const MAX_LIST_DAYS: i64 = 93;

/// A JSON value, validated against the input's schema as the app's valibot schema does,
/// and decoded. A refusal answers the first issue's key, parameter, and English message;
/// valibot's own checks carry no key and answer validation_invalid.
pub fn decode<T: Schema>(value: Value) -> Result<T> {
    crate::inputs::runtime::parse(value).map_err(|issue| {
        debug_assert!(issue.params.len() <= 1, "one parameter at most: {issue:?}");
        Error::Invalid(Invalid {
            key: issue.key.unwrap_or("validation_invalid"),
            params: issue.params.first().map(|&(name, param)| {
                let param = match param {
                    crate::inputs::runtime::Param::Number(n) => Param::Number(n),
                    crate::inputs::runtime::Param::Text(t) => Param::Text(t),
                };
                (name, param)
            }),
            message: issue.message,
        })
    })
}

#[derive(serde::Deserialize)]
pub struct Empty {}
impl Schema for Empty {
    fn check(_: &mut Value, _: &mut crate::inputs::runtime::Ctx) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::schemas::*;
    use crate::projects::schemas::*;
    use crate::timer::schemas::*;
    use serde_json::json;

    fn body(error: Error) -> String {
        let Error::Invalid(invalid) = error else {
            panic!("a validation refusal")
        };
        String::from_utf8(crate::wire::invalid_failure(invalid).body).unwrap()
    }

    fn refusal<T: Schema>(input: Value) -> String {
        body(decode::<T>(input).err().expect("a refusal"))
    }

    // validationRefusal in the app's src/server/http.server.ts (task 095).
    #[test]
    fn refusals_answer_key_params_and_english_message_in_order() {
        let id = "0192f3a4-5b6c-7d8e-9f01-23456789abcd";
        assert_eq!(
            refusal::<StartTimerInput>(json!({ "id": id, "description": "x".repeat(501) })),
            r#"{"error":{"key":"validation_too_long","params":{"max":500},"message":"Use at most 500 characters."}}"#
        );
        assert_eq!(
            refusal::<StopTimerInput>(json!({ "id": "not-a-uuid" })),
            r#"{"error":{"key":"validation_invalid_id","message":"Invalid id."}}"#
        );
        assert_eq!(
            refusal::<StopTimerInput>(json!({})),
            r#"{"error":{"key":"validation_invalid","message":"Invalid key: Expected \"id\" but received undefined"}}"#
        );
    }

    #[test]
    fn checks_ids_and_tickets() {
        let stop = |id: &str| decode::<StopTimerInput>(json!({ "id": id })).is_ok();
        assert!(stop("0192f3a4-5b6c-7d8e-9f01-23456789abcd"));
        assert!(stop("0192F3A4-5B6C-7D8E-BF01-23456789ABCD"));
        assert!(!stop("0192f3a4-5b6c-4d8e-9f01-23456789abcd"));
        assert!(!stop("not-a-uuid"));
        let id = "0192f3a4-5b6c-7d8e-9f01-23456789abcd";
        let ticket =
            |key: &str| decode::<StartTimerInput>(json!({ "id": id, "ticket": key })).is_ok();
        assert!(ticket("LUM-12"));
        assert!(ticket("A1B2C3D4E5-1234567"));
        assert!(!ticket("L-12"));
        assert!(!ticket("LUM-012"));
        assert!(!ticket("lum-12"));
    }

    #[test]
    fn decodes_with_defaults_and_trimming() {
        let input: StartTimerInput = decode(json!({
            "organizationId": "o",
            "id": "0192f3a4-5b6c-7d8e-9f01-23456789abcd",
            "description": "  Planning  ",
        }))
        .unwrap();
        assert_eq!(input.description, "Planning");
        assert_eq!(input.ticket, Patch::Absent);
        let update: UpdateEntryInput = decode(json!({
            "id": "0192f3a4-5b6c-7d8e-9f01-23456789abcd",
            "projectId": null,
        }))
        .unwrap();
        assert_eq!(update.project_id, Patch::Null);
        assert_eq!(update.ticket, Patch::Absent);
        assert!(decode::<StopTimerInput>(json!({ "id": 3 })).is_err());
    }

    #[test]
    fn reads_query_booleans() {
        let archived = |input| decode::<ListProjectsInput>(input).map(|i| i.include_archived);
        assert_eq!(
            archived(json!({ "includeArchived": "true" })).ok(),
            Some(true)
        );
        assert_eq!(
            archived(json!({ "includeArchived": "false" })).ok(),
            Some(false)
        );
        assert_eq!(archived(json!({ "organizationId": "o" })).ok(), Some(false));
        let Err(Error::Invalid(invalid)) = archived(json!({ "includeArchived": "yes" })) else {
            panic!("a flag that isn't true or false fails");
        };
        assert_eq!(
            invalid.message,
            r#"Invalid type: Expected boolean but received "yes""#
        );
    }
}
