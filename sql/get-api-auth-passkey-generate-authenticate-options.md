# GET /api/auth/passkey/generate-authenticate-options

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 30 | 1 (1–2) | 1 (1–2) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 3 of 30 requests:

```sql
select "id", "name", "public_key", "user_id", "credential_id", "counter", "device_type", "backed_up", "transports", "created_at", "aaguid" from "passkey" where "passkey"."user_id" = ? limit ?
```

In 30 of 30 requests:

```sql
insert into "verification" ("id", "identifier", "value", "expires_at", "created_at", "updated_at") values (?, …) returning "id", "identifier", "value", "expires_at", "created_at", "updated_at"
```
