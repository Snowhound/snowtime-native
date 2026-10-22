# POST /api/auth/link-social

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 5 | 3 (0–3) | 3 (0–3) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 4 of 5 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 4 of 5 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 4 of 5 requests:

```sql
insert into "verification" ("id", "identifier", "value", "expires_at", "created_at", "updated_at") values (?, …) returning "id", "identifier", "value", "expires_at", "created_at", "updated_at"
```
