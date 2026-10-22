# DELETE /api/v1/organizations/:organizationId/teams/:teamId

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 3 | 3 | 3 | 0 | 0 |

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
delete from "team" where (("team"."id" = ?) and ("team"."organization_id" = ?)) returning "id"
```
