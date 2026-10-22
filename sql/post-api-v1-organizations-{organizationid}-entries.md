# POST /api/v1/organizations/:organizationId/entries

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 2 | 4 | 4 | 0 | 0 |

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

In 1 of 2 requests:

```sql
select "archived_at" from "project" where (("project"."id" = ?) and ((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))))
```

In 2 of 2 requests:

```sql
select count(*) from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."user_id" = ?) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?))
```

In 1 of 2 requests:

```sql
insert into "time_entry" ("id", "organization_id", "user_id", "project_id", "description", "started_at", "stopped_at", "created_at", "created_by", "updated_at", "updated_by", "sys_deleted", "ticket") values (?, ?, ?, ?, ?, ?, ?, (CAST(ROUND(unixepoch('subsec') * 1000) AS INTEGER)), ?, (CAST(ROUND(unixepoch('subsec') * 1000) AS INTEGER)), ?, 0, ?) returning "id", "organization_id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at"
```
