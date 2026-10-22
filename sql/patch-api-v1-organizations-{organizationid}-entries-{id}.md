# PATCH /api/v1/organizations/:organizationId/entries/:id

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 3 | 3 (3–4) | 3 (3–4) | 0 | 0 |

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
select "id", "organization_id", "user_id", "project_id", "description", "started_at", "stopped_at", "created_at", "created_by", "updated_at", "updated_by", "sys_deleted", "ticket" from "time_entry" where (("time_entry"."id" = ?) and ((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))))
```

In 1 of 3 requests:

```sql
update "time_entry" set "description" = ?, "updated_at" = ?, "updated_by" = ? where (("time_entry"."id" = ?) and ((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0)))) returning "id", "organization_id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at"
```
