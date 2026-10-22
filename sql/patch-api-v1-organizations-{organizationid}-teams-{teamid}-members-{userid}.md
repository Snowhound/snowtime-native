# PATCH /api/v1/organizations/:organizationId/teams/:teamId/members/:userId

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 2 | 3.5 (3–4) | 3.5 (3–4) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 2 of 2 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 2 of 2 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 2 of 2 requests:

```sql
select "id" from "team" where (("team"."id" = ?) and ("team"."organization_id" = ?))
```

In 1 of 2 requests:

```sql
update "team_member" set "role" = ? where (("team_member"."team_id" = ?) and ("team_member"."user_id" = ?)) returning "team_id", "user_id", "role"
```
