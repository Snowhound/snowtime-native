# POST /api/v1/organizations/:organizationId/report/entry-totals

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 1 | 5 | 5 | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 1 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 1 of 1 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 1 of 1 requests:

```sql
select "time_zone", "week_start" from "user_settings" where "user_settings"."user_id" = ?
```

In 1 of 1 requests:

```sql
select "team"."id", "team_member"."user_id" from "team" left join "team_member" on "team_member"."team_id" = "team"."id" where "team"."organization_id" = ?
```

In 1 of 1 requests:

```sql
select "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?))) and ("time_entry"."project_id" = ?))
```
