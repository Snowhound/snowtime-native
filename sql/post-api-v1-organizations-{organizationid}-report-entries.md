# POST /api/v1/organizations/:organizationId/report/entries

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 3 | 5 (5–6) | 5 (5–6) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 3 of 3 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 3 of 3 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 3 of 3 requests:

```sql
select "time_zone", "week_start" from "user_settings" where "user_settings"."user_id" = ?
```

In 3 of 3 requests:

```sql
select "team"."id", "team_member"."user_id" from "team" left join "team_member" on "team_member"."team_id" = "team"."id" where "team"."organization_id" = ?
```

In 2 of 3 requests, **up to 2 times in one request**:

```sql
with "probe" as (select "started_at" from "time_entry" where (((((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?))))) and (min( coalesce("time_entry"."stopped_at", min(?, "time_entry"."started_at" + ?)), ? ) > max("time_entry"."started_at", ?))) order by "time_entry"."started_at" desc limit ?), "day_window" as (select coalesce(max(cast(value as integer)), ?) as "window_from" from json_each(?) where cast(value as integer) <= ( select case when count(*) < ? then ? else min("probe"."started_at") end from "probe" )) select "time_entry"."id", "time_entry"."user_id", "time_entry"."project_id", "time_entry"."description", "time_entry"."ticket", "time_entry"."started_at", "time_entry"."stopped_at", "day_window"."window_from" from "day_window" cross join "time_entry" where (((((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > "day_window"."window_from" - ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > "day_window"."window_from"))))) and (min( coalesce("time_entry"."stopped_at", min(?, "time_entry"."started_at" + ?)), ? ) > max("time_entry"."started_at", ?)))
```

In 1 of 3 requests:

```sql
select "id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?))) and ("time_entry"."project_id" = ?))
```
