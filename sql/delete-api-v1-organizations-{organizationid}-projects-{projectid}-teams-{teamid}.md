# DELETE /api/v1/organizations/:organizationId/projects/:projectId/teams/:teamId

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 1 | 5 | 5 | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 1 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 1 of 1 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 1 of 1 requests:

```sql
select "id", "name", "color", "archived_at" from "project" where (("project"."id" = ?) and ((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))))
```

In 1 of 1 requests:

```sql
select "id" from "team" where (("team"."id" = ?) and ("team"."organization_id" = ?))
```

In 1 of 1 requests:

```sql
delete from "project_team" where (("project_team"."project_id" = ?) and ("project_team"."team_id" = ?) and ("project_team"."organization_id" = ?))
```
