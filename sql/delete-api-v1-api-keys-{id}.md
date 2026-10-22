# DELETE /api/v1/api-keys/:id

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 3 | 1 (0–1) | 1 (0–1) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 2 of 3 requests:

```sql
delete from "api_key" where (("api_key"."id" = ?) and ("api_key"."reference_id" = ?)) returning "id"
```
