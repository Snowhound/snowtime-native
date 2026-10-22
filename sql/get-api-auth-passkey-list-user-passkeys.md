# GET /api/auth/passkey/list-user-passkeys

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 13 | 1 (0–2) | 1 (0–2) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 13 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 1 of 13 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 11 of 13 requests:

```sql
select "id", "name", "public_key", "user_id", "credential_id", "counter", "device_type", "backed_up", "transports", "created_at", "aaguid" from "passkey" where "passkey"."user_id" = ? limit ?
```
