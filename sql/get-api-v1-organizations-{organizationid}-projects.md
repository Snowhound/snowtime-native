# GET /api/v1/organizations/:organizationId/projects

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 10 | 4 (4–5) | 4 (4–5) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 10 requests:

```sql
select "api_key"."id", "api_key"."reference_id", "user"."email", "api_key"."enabled", "api_key"."expires_at", "api_key"."permissions", "api_key"."last_request" from "api_key" inner join "user" on "user"."id" = "api_key"."reference_id" where "api_key"."key" = ?
```

In 10 of 10 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 10 of 10 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 1 of 10 requests:

```sql
select "id", "name", "color", "archived_at", exists (select "id" from "time_entry" where (("time_entry"."project_id" = "project"."id") and ((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))))) from "project" where (((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))) and (((not exists (select "project_id" from "project_team" where "project_team"."project_id" = "project"."id")) or (exists (select "project_team"."project_id" from "project_team" inner join "team_member" on "team_member"."team_id" = "project_team"."team_id" where (("project_team"."project_id" = "project"."id") and ("team_member"."user_id" = ?)))))) and (("project"."archived_at" is null))) order by "project"."name" asc
```

In 10 of 10 requests:

```sql
select "project_id", "team_id" from "project_team" where "project_team"."organization_id" = ?
```

In 4 of 10 requests:

```sql
select "id", "name", "color", "archived_at", exists (select "id" from "time_entry" where (("time_entry"."project_id" = "project"."id") and ((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))))) from "project" where (("project"."organization_id" = ?) and ("project"."sys_deleted" = 0)) order by "project"."name" asc
```

In 5 of 10 requests:

```sql
select "id", "name", "color", "archived_at", exists (select "id" from "time_entry" where (("time_entry"."project_id" = "project"."id") and ((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))))) from "project" where (((("project"."organization_id" = ?) and ("project"."sys_deleted" = 0))) and (("project"."archived_at" is null))) order by "project"."name" asc
```
