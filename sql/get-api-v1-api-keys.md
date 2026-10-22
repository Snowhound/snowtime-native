# GET /api/v1/api-keys

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 2 | 0.5 (0–1) | 0.5 (0–1) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 2 requests:

```sql
select "id", "name", "permissions", "created_at", "expires_at", "last_request" from "api_key" where "api_key"."reference_id" = ? order by "api_key"."created_at" desc
```
