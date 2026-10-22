# POST /api/auth/organization/set-active

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 16 | 2 (0–6) | 2 (0–6) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 11 of 16 requests, **up to 2 times in one request**:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 10 of 16 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 2 of 16 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."slug" = ?
```

In 5 of 16 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where (("member"."user_id" = ?) and ("member"."organization_id" = ?))
```

In 4 of 16 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."id" = ?
```

In 6 of 16 requests:

```sql
update "session" set "active_organization_id" = ?, "updated_at" = ? where "session"."token" = ? returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```
