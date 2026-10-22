# POST /api/auth/organization/leave

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 8 | 0 (0–10) | 0 (0–8) | 0 (0–1) | 0 (0–3) |

Per request: the median, and the range when requests differ.

## Statements

In 3 of 8 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 3 of 8 requests, **up to 2 times in one request**:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 3 of 8 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where (("member"."user_id" = ?) and ("member"."organization_id" = ?))
```

In 1 of 8 requests:

```sql
delete from "member" where "member"."id" = ?
```

In 1 of 8 requests:

```sql
update "session" set "active_organization_id" = ?, "updated_at" = ? where "session"."token" = ? returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```

In 1 of 8 requests:

```sql
update "time_entry" set "stopped_at" = min(max(?, "time_entry"."started_at" + 1), "time_entry"."started_at" + ?), "updated_at" = ?, "updated_by" = ? where (((("time_entry"."user_id" = ?) and (("time_entry"."stopped_at" is null)) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."organization_id" = ?)) returning "id", "organization_id", "user_id", "project_id", "description", "started_at", "stopped_at", "created_at", "created_by", "updated_at", "updated_by", "sys_deleted", "ticket"
```

In 1 of 8 requests:

```sql
delete from "team_member" where (("team_member"."user_id" = ?) and ("team_member"."team_id" in (select "id" from "team" where "team"."organization_id" = ?))) returning "team_id"
```

In 1 of 8 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where "member"."organization_id" = ? limit ?
```
