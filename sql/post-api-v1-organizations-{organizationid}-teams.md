# POST /api/v1/organizations/:organizationId/teams

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 3 | 6 (2–6) | 4 (2–4) | 1 (0–1) | 0 |

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

In 2 of 3 requests:

```sql
select count(*) from "team" where "team"."organization_id" = ?
```

In 2 of 3 requests:

```sql
insert into "team" ("id", "organization_id", "name", "member_count", "created_at", "updated_at") values (?, ?, ?, ?, ?, null) returning "id", "name"
```
