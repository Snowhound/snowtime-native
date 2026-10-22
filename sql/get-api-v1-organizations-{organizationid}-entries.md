# GET /api/v1/organizations/:organizationId/entries

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 9 | 3 (2–5) | 3 (2–5) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 9 requests:

```sql
select "api_key"."id", "api_key"."reference_id", "user"."email", "api_key"."enabled", "api_key"."expires_at", "api_key"."permissions", "api_key"."last_request" from "api_key" inner join "user" on "user"."id" = "api_key"."reference_id" where "api_key"."key" = ?
```

In 9 of 9 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 9 of 9 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 1 of 9 requests:

```sql
select "id", "organization_id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."user_id" in (?)) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?)))) order by "time_entry"."started_at" desc
```

In 1 of 9 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 1 of 9 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 6 of 9 requests:

```sql
select "id", "organization_id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?)))) order by "time_entry"."started_at" desc
```
