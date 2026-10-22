# POST /api/v1/timer/stop

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 5 | 1 (1–2) | 1 (1–2) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 3 of 5 requests:

```sql
select "api_key"."id", "api_key"."reference_id", "user"."email", "api_key"."enabled", "api_key"."expires_at", "api_key"."permissions", "api_key"."last_request" from "api_key" inner join "user" on "user"."id" = "api_key"."reference_id" where "api_key"."key" = ?
```

In 4 of 5 requests:

```sql
update "time_entry" set "stopped_at" = min(max(?, "time_entry"."started_at" + 1), "time_entry"."started_at" + ?), "updated_at" = ?, "updated_by" = ? where (((("time_entry"."user_id" = ?) and (("time_entry"."stopped_at" is null)) and ("time_entry"."sys_deleted" = 0))) and (exists (select 1 from "member" where "member"."organization_id" = "time_entry"."organization_id" and "member"."user_id" = ?)) and ("time_entry"."id" = ?)) returning "id", "organization_id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at"
```
