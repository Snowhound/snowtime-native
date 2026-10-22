# POST /api/v1/api-keys

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 4 | 2 (0–3) | 2 (0–3) | 0 | 0 (0–2) |

Per request: the median, and the range when requests differ.

## Statements

In 3 of 4 requests:

```sql
select count(*) from "api_key" where "api_key"."reference_id" = ?
```

In 1 of 4 requests:

```sql
delete from "api_key" where (("api_key"."expires_at" < ?) and (("api_key"."expires_at" is not null)))
```

In 3 of 4 requests:

```sql
insert into "api_key" ("id", "config_id", "name", "start", "reference_id", "prefix", "key", "refill_interval", "refill_amount", "last_refill_at", "enabled", "rate_limit_enabled", "rate_limit_time_window", "rate_limit_max", "request_count", "remaining", "last_request", "expires_at", "created_at", "updated_at", "permissions", "metadata") values (?, …) returning "id", "config_id", "name", "start", "reference_id", "prefix", "key", "refill_interval", "refill_amount", "last_refill_at", "enabled", "rate_limit_enabled", "rate_limit_time_window", "rate_limit_max", "request_count", "remaining", "last_request", "expires_at", "created_at", "updated_at", "permissions", "metadata"
```
