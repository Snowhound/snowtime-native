# DELETE /api/v1/organizations/:organizationId/teams/:teamId/members/:userId

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 3 | 6 (5–7) | 4 (3–5) | 1 | 0 |

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
select "id" from "team" where (("team"."id" = ?) and ("team"."organization_id" = ?))
```

In 2 of 3 requests:

```sql
delete from "team_member" where (("team_member"."team_id" = ?) and ("team_member"."user_id" = ?)) returning "id"
```

In 1 of 3 requests:

```sql
update "team" set "member_count" = "team"."member_count" - 1 where "team"."id" = ?
```
