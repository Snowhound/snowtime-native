# POST /api/v1/organizations/:organizationId/report/export

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 2 | 7 (5–9) | 7 (5–9) | 0 | 0 |

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

In 2 of 2 requests, **up to 2 times in one request**:

```sql
select "time_zone", "week_start" from "user_settings" where "user_settings"."user_id" = ?
```

In 2 of 2 requests, **up to 2 times in one request**:

```sql
select "team"."id", "team_member"."user_id" from "team" left join "team_member" on "team_member"."team_id" = "team"."id" where "team"."organization_id" = ?
```

In 2 of 2 requests:

```sql
select "id", "user_id", "project_id", "description", "ticket", "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?))))
```

In 1 of 2 requests:

```sql
select "user_id", "project_id", "ticket", "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?))))
```

In 1 of 2 requests:

```sql
select "id", "name", "email" from "user" where (("user"."id" in ( with recursive ids(id) as ( select min(user_id) from time_entry where organization_id = ? union all select (select min(user_id) from time_entry where organization_id = ? and user_id > ids.id) from ids where id is not null ) select id from ids )) and ("user"."id" <> ?) and (not exists (select 1 from "member" where (("member"."user_id" = "user"."id") and ("member"."organization_id" = ?)))) and (exists (select 1 from "time_entry" where (("time_entry"."user_id" = "user"."id") and ((((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?)))))))))
```
