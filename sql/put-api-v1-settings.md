# PUT /api/v1/settings

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 3 | 1 | 1 | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 3 of 3 requests:

```sql
select "time_zone", "week_start", "locale", "theme", "timer_layout", "show_summary", "compact_rows", "wide_timer", "timer_view", "calendar_weekend", "app_icon", "scene_collection", "scene_pin", "scene_background", "scene_strength", "surfaces", "scene_weather", "scene_intro", "scene_tagline", "duration_format", "date_format", "time_format", "copy_duration_pattern", "copy_duration_control", "country" from "user_settings" where "user_settings"."user_id" = ?
```
