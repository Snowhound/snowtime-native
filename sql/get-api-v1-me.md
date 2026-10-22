# GET /api/v1/me

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 5 | 3 (1–4) | 3 (1–4) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 5 of 5 requests:

```sql
select "api_key"."id", "api_key"."reference_id", "user"."email", "api_key"."enabled", "api_key"."expires_at", "api_key"."permissions", "api_key"."last_request" from "api_key" inner join "user" on "user"."id" = "api_key"."reference_id" where "api_key"."key" = ?
```

In 2 of 5 requests:

```sql
update "api_key" set "last_request" = ? where "api_key"."id" = ?
```

In 3 of 5 requests:

```sql
select "organization"."id", "organization"."name", "organization"."slug", "organization"."issue_links", "member"."role" from "member" inner join "organization" on "organization"."id" = "member"."organization_id" where "member"."user_id" = ? order by "organization"."name" asc, "organization"."id" asc
```

In 3 of 5 requests:

```sql
select "id", "name", "email" from "user" where "user"."id" = ?
```
