# GET /api/v1/session

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 25 | 0 (0–5) | 0 (0–5) | 0 | 0 |

Per request: the median, and the range when requests differ. 13 of them in process, from another request.

## Statements

In 6 of 25 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 6 of 25 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 9 of 25 requests:

```sql
select "organization"."id", "organization"."name", "organization"."slug", "organization"."issue_links", "member"."role" from "member" inner join "organization" on "organization"."id" = "member"."organization_id" where "member"."user_id" = ? order by "organization"."name" asc, "organization"."id" asc
```

In 9 of 25 requests:

```sql
select "time_zone", "week_start", "locale", "theme", "timer_layout", "show_summary", "compact_rows", "wide_timer", "timer_view", "calendar_weekend", "app_icon", "scene_collection", "scene_pin", "scene_background", "scene_strength", "surfaces", "scene_weather", "scene_intro", "scene_tagline", "duration_format", "date_format", "time_format", "copy_duration_pattern", "copy_duration_control", "country" from "user_settings" where "user_settings"."user_id" = ?
```

In 5 of 25 requests:

```sql
select "started_at", "stopped_at" from "time_entry" where (("time_entry"."organization_id" in (?)) and ("time_entry"."user_id" = ?) and ("time_entry"."sys_deleted" = 0) and ("time_entry"."started_at" >= ?) and ("time_entry"."started_at" < ?)) union all select "started_at", "stopped_at" from "time_entry" where (("time_entry"."user_id" = ?) and (("time_entry"."stopped_at" is null)) and ("time_entry"."sys_deleted" = 0) and ("time_entry"."started_at" < ?))
```

In 1 of 25 requests:

```sql
select "started_at", "stopped_at" from "time_entry" where (("time_entry"."organization_id" in (?, …)) and ("time_entry"."user_id" = ?) and ("time_entry"."sys_deleted" = 0) and ("time_entry"."started_at" >= ?) and ("time_entry"."started_at" < ?)) union all select "started_at", "stopped_at" from "time_entry" where (("time_entry"."user_id" = ?) and (("time_entry"."stopped_at" is null)) and ("time_entry"."sys_deleted" = 0) and ("time_entry"."started_at" < ?))
```

In 1 of 25 requests:

```sql
update "session" set "expires_at" = ?, "updated_at" = ? where "session"."token" = ? returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```

In 3 of 25 requests:

```sql
select "id" from "invitation" where ((lower("invitation"."email") = ?) and ("invitation"."status" = ?) and ("invitation"."expires_at" > ?)) order by "invitation"."expires_at" asc limit ?
```
