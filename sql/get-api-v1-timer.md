# GET /api/v1/timer

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 10 | 1.5 (0–3) | 1.5 (0–3) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 2 of 10 requests:

```sql
select "api_key"."id", "api_key"."reference_id", "user"."email", "api_key"."enabled", "api_key"."expires_at", "api_key"."permissions", "api_key"."last_request" from "api_key" inner join "user" on "user"."id" = "api_key"."reference_id" where "api_key"."key" = ?
```

In 8 of 10 requests:

```sql
select "d0"."id" as "id", "d0"."organization_id" as "organizationId", "d0"."user_id" as "userId", "d0"."project_id" as "projectId", "d0"."description" as "description", "d0"."ticket" as "ticket", "d0"."started_at" as "startedAt", "d0"."stopped_at" as "stoppedAt", (select json_object('id', "id", 'name', "name", 'color', "color") as "r" from (select "d1"."id" as "id", "d1"."name" as "name", "d1"."color" as "color" from "project" as "d1" where "d0"."project_id" = "d1"."id" limit ?) as "t") as "project" from "time_entry" as "d0" where (("d0"."user_id" = ?) and (("d0"."stopped_at" is null)) and ("d0"."sys_deleted" = 0) and (exists (select * from "organization" as "f0" where (("d0"."organization_id" = "f0"."id") and (exists (select * from "member" as "f1" where (("f0"."id" = "f1"."organization_id") and ("f1"."user_id" = ?)) limit 1))) limit 1))) limit ?
```

In 3 of 10 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 3 of 10 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```
