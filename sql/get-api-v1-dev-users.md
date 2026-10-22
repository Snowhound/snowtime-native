# GET /api/v1/dev-users

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 1 | 1 | 1 | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 1 of 1 requests:

```sql
select "d0"."email" as "email" from "user" as "d0" where "d0"."email" in (?, …)
```
