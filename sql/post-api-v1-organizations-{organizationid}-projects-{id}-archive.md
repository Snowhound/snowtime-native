# POST /api/v1/organizations/:organizationId/projects/:id/archive

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
select "id", "name", "color", "archived_at" from "project" where (("project"."id" = ?) and ((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))))
```

In 1 of 2 requests:

```sql
update "project" set "archived_at" = ?, "updated_at" = ?, "updated_by" = ? where (("project"."id" = ?) and ((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0)))) returning "id", "name", "color", "archived_at"
```
