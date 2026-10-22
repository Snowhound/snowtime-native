# POST /api/auth/sign-in/social

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 45 | 1 (0–1) | 1 (0–1) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 33 of 45 requests:

```sql
insert into "verification" ("id", "identifier", "value", "expires_at", "created_at", "updated_at") values (?, …) returning "id", "identifier", "value", "expires_at", "created_at", "updated_at"
```
