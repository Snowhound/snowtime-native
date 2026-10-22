# POST /api/v1/organizations/:organizationId/projects

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 4 | 3 (2–4) | 3 (2–4) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 4 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 1 of 4 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 3 of 4 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 3 of 4 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 2 of 4 requests:

```sql
select count(*) from "project" where (("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))
```

In 2 of 4 requests:

```sql
insert into "project" ("id", "organization_id", "name", "color", "archived_at", "created_at", "created_by", "updated_at", "updated_by", "sys_deleted") values (?, ?, ?, ?, null, (CAST(ROUND(unixepoch('subsec') * 1000) AS INTEGER)), ?, (CAST(ROUND(unixepoch('subsec') * 1000) AS INTEGER)), ?, 0) returning "id", "name", "color", "archived_at"
```
