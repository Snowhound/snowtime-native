# POST /api/v1/organizations/:organizationId/report

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 10 | 6 (2–6) | 6 (2–6) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 10 of 10 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 10 of 10 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 9 of 10 requests:

```sql
select "time_zone", "week_start" from "user_settings" where "user_settings"."user_id" = ?
```

In 7 of 10 requests:

```sql
select "team"."id", "team_member"."user_id" from "team" left join "team_member" on "team_member"."team_id" = "team"."id" where "team"."organization_id" = ?
```

In 5 of 10 requests:

```sql
select "user_id", "project_id", "ticket", "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?))))
```

In 5 of 10 requests:

```sql
select "id", "name", "email" from "user" where (("user"."id" in ( with recursive ids(id) as ( select min(user_id) from time_entry where organization_id = ? union all select (select min(user_id) from time_entry where organization_id = ? and user_id > ids.id) from ids where id is not null ) select id from ids )) and ("user"."id" <> ?) and (not exists (select 1 from "member" where (("member"."user_id" = "user"."id") and ("member"."organization_id" = ?)))) and (exists (select 1 from "time_entry" where (("time_entry"."user_id" = "user"."id") and ((((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?)))))))))
```

In 2 of 10 requests:

```sql
select "id" from "team" where (("team"."id" = ?) and ("team"."organization_id" = ?))
```

In 1 of 10 requests:

```sql
select "user_id", "project_id", "ticket", "started_at", "stopped_at" from "time_entry" where (((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."project_id" = ?) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?))))
```

In 1 of 10 requests:

```sql
select "id", "name", "email" from "user" where (("user"."id" in ( with recursive ids(id) as ( select min(user_id) from time_entry where organization_id = ? union all select (select min(user_id) from time_entry where organization_id = ? and user_id > ids.id) from ids where id is not null ) select id from ids )) and ("user"."id" <> ?) and (not exists (select 1 from "member" where (("member"."user_id" = "user"."id") and ("member"."organization_id" = ?)))) and (exists (select 1 from "time_entry" where (("time_entry"."user_id" = "user"."id") and ((((("time_entry"."organization_id" = ?) and ("time_entry"."sys_deleted" = 0))) and ("time_entry"."project_id" = ?) and ("time_entry"."started_at" > ?) and ("time_entry"."started_at" < ?) and (((("time_entry"."stopped_at" is null)) or ("time_entry"."stopped_at" > ?)))))))))
```
