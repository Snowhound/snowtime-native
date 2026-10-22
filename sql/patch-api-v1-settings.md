# PATCH /api/v1/settings

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 9 | 1 (0–1) | 1 (0–1) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 2 of 9 requests:

```sql
update "user_settings" set "updated_at" = ?, "updated_by" = ?, "locale" = ? where "user_settings"."user_id" = ? returning "time_zone", "week_start", "locale", "theme", "timer_layout", "show_summary", "compact_rows", "wide_timer", "timer_view", "calendar_weekend", "app_icon", "scene_collection", "scene_pin", "scene_background", "scene_strength", "surfaces", "scene_weather", "scene_intro", "scene_tagline", "duration_format", "date_format", "time_format", "copy_duration_pattern", "copy_duration_control", "country"
```

In 2 of 9 requests:

```sql
update "user_settings" set "week_start" = ?, "updated_at" = ?, "updated_by" = ? where "user_settings"."user_id" = ? returning "time_zone", "week_start", "locale", "theme", "timer_layout", "show_summary", "compact_rows", "wide_timer", "timer_view", "calendar_weekend", "app_icon", "scene_collection", "scene_pin", "scene_background", "scene_strength", "surfaces", "scene_weather", "scene_intro", "scene_tagline", "duration_format", "date_format", "time_format", "copy_duration_pattern", "copy_duration_control", "country"
```

In 2 of 9 requests:

```sql
update "user_settings" set "updated_at" = ?, "updated_by" = ?, "copy_duration_pattern" = ?, "copy_duration_control" = ? where "user_settings"."user_id" = ? returning "time_zone", "week_start", "locale", "theme", "timer_layout", "show_summary", "compact_rows", "wide_timer", "timer_view", "calendar_weekend", "app_icon", "scene_collection", "scene_pin", "scene_background", "scene_strength", "surfaces", "scene_weather", "scene_intro", "scene_tagline", "duration_format", "date_format", "time_format", "copy_duration_pattern", "copy_duration_control", "country"
```
