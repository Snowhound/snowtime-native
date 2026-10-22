# POST /api/auth/update-user

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 18 | 2 (0–3) | 2 (0–3) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 13 of 18 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 13 of 18 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 1 of 18 requests:

```sql
update "user" set "name" = ?, "updated_at" = ? where "user"."id" = ? returning "id", "name", "email", "email_verified", "image", "created_at", "updated_at"
```

In 1 of 18 requests:

```sql
update "user" set "name" = ?, "image" = ?, "updated_at" = ? where "user"."id" = ? returning "id", "name", "email", "email_verified", "image", "created_at", "updated_at"
```

In 2 of 18 requests:

```sql
update "user" set "image" = ?, "updated_at" = ? where "user"."id" = ? returning "id", "name", "email", "email_verified", "image", "created_at", "updated_at"
```
