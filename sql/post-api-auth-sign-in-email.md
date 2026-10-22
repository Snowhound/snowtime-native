# POST /api/auth/sign-in/email

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 26 | 3 (2–4) | 3 (2–4) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 25 of 26 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."email" = ?
```

In 25 of 26 requests:

```sql
select "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at" from "account" where "account"."user_id" = ? limit ?
```

In 23 of 26 requests:

```sql
insert into "session" ("id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at") values (?, ?, ?, ?, ?, ?, null, null, ?, ?) returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```

In 1 of 26 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 1 of 26 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 2 of 26 requests:

```sql
select "d0"."email" as "email" from "user" as "d0" where "d0"."id" = ? limit ?
```
