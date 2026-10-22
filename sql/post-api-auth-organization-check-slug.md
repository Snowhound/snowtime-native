# POST /api/auth/organization/check-slug

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 6 | 0 (0–3) | 0 (0–3) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 2 of 6 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 2 of 6 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 2 of 6 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."slug" = ?
```
