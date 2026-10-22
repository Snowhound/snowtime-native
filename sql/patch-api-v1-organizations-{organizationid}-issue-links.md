# PATCH /api/v1/organizations/:organizationId/issue-links

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 4 | 2.5 (2–3) | 2.5 (2–3) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 4 of 4 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 4 of 4 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 2 of 4 requests:

```sql
update "organization" set "issue_links" = ? where "organization"."id" = ? returning "id", "issue_links"
```
