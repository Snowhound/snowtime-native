# GET /api/v1/invitations/:id

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 4 | 1 | 1 | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 4 of 4 requests:

```sql
select "invitation"."id", "invitation"."email", "invitation"."role", "invitation"."status", "invitation"."expires_at", "invitation"."organization_id", "organization"."name", "team"."name", "inviter"."name" from "invitation" inner join "organization" on "organization"."id" = "invitation"."organization_id" inner join "user" "inviter" on "inviter"."id" = "invitation"."inviter_id" left join "team" on "team"."id" = "invitation"."team_id" where "invitation"."id" = ?
```
