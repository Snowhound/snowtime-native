# GET /api/v1/organizations/:organizationId/teams

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 7 | 4 (0–6) | 4 (0–6) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 7 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 1 of 7 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 6 of 7 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 6 of 7 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 6 of 7 requests:

```sql
select "id", "name" from "team" where "team"."organization_id" = ? order by "team"."name" asc
```

In 6 of 7 requests:

```sql
select "team_member"."team_id", "team_member"."user_id", "team_member"."role" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where "team"."organization_id" = ?
```
