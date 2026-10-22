# POST /api/auth/sign-out

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 1 | 5 | 5 | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 1 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 1 of 1 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 1 of 1 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ? limit ?
```

In 1 of 1 requests:

```sql
delete from "session" where "session"."token" = ?
```

In 1 of 1 requests:

```sql
select "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at" from "account" where "account"."user_id" = ? limit ?
```
