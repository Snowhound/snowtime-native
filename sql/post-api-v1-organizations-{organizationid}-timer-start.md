# POST /api/v1/organizations/:organizationId/timer/start

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 11 | 2 (0–10) | 2 (0–8) | 0 (0–1) | 0 (0–1) |

Per request: the median, and the range when requests differ.

## Statements

In 2 of 11 requests:

```sql
select "api_key"."id", "api_key"."reference_id", "user"."email", "api_key"."enabled", "api_key"."expires_at", "api_key"."permissions", "api_key"."last_request" from "api_key" inner join "user" on "user"."id" = "api_key"."reference_id" where "api_key"."key" = ?
```

In 1 of 11 requests:

```sql
update "api_key" set "last_request" = ? where "api_key"."id" = ?
```

In 8 of 11 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 8 of 11 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 5 of 11 requests:

```sql
select "id" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 5 of 11 requests:

```sql
select count(*) from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."user_id" = ?) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?))
```

In 5 of 11 requests:

```sql
update "time_entry" set "stopped_at" = min(max(?, "time_entry"."started_at" + 1), "time_entry"."started_at" + ?), "updated_at" = ?, "updated_by" = ? where (((("time_entry"."user_id" = ?) and (("time_entry"."stopped_at" is null)) and ("time_entry"."sys_deleted" = 0))) and (exists (select 1 from "member" where "member"."organization_id" = "time_entry"."organization_id" and "member"."user_id" = ?))) returning "id", "organization_id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at"
```

In 4 of 11 requests:

```sql
insert into "time_entry" ("id", "organization_id", "user_id", "project_id", "description", "started_at", "stopped_at", "created_at", "created_by", "updated_at", "updated_by", "sys_deleted", "ticket") values (?, ?, ?, ?, ?, ?, null, (CAST(ROUND(unixepoch('subsec') * 1000) AS INTEGER)), ?, (CAST(ROUND(unixepoch('subsec') * 1000) AS INTEGER)), ?, 0, ?) returning "id", "organization_id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at"
```

In 1 of 11 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 1 of 11 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 1 of 11 requests:

```sql
select "archived_at" from "project" where (("project"."id" = ?) and ((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))))
```
