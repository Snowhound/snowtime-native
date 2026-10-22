# POST /api/auth/passkey/delete-passkey

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 16 | 1 (0–2) | 1 (0–2) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 10 of 16 requests:

```sql
select "id", "name", "public_key", "user_id", "credential_id", "counter", "device_type", "backed_up", "transports", "created_at", "aaguid" from "passkey" where "passkey"."id" = ?
```

In 3 of 16 requests:

```sql
delete from "passkey" where "passkey"."id" = ?
```
