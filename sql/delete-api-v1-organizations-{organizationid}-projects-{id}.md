# DELETE /api/v1/organizations/:organizationId/projects/:id

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 4 | 4.5 (3–8) | 3.5 (3–6) | 0.5 (0–1) | 0 |

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

In 4 of 4 requests:

```sql
select "id", "name", "color", "archived_at" from "project" where (("project"."id" = ?) and ((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))))
```

In 2 of 4 requests:

```sql
select "id" from "time_entry" where (("time_entry"."project_id" = ?) and ((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0)))) limit ?
```

In 1 of 4 requests:

```sql
delete from "project_team" where (("project_team"."project_id" = ?) and ("project_team"."organization_id" = ?))
```

In 1 of 4 requests:

```sql
update "project" set "updated_at" = ?, "updated_by" = ?, "sys_deleted" = ? where (("project"."id" = ?) and ((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0)))) returning "id"
```
