# POST /api/auth/organization/update

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 17 | 4 (0–5) | 4 (0–5) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 12 of 17 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 12 of 17 requests, **up to 2 times in one request**:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 11 of 17 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where (("member"."user_id" = ?) and ("member"."organization_id" = ?))
```

In 2 of 17 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."slug" = ?
```

In 1 of 17 requests:

```sql
update "organization" set "name" = ?, "logo" = ?, "metadata" = ? where "organization"."id" = ? returning "id", "name", "slug", "logo", "metadata", "created_at", "issue_links"
```

In 1 of 17 requests:

```sql
update "organization" set "metadata" = ? where "organization"."id" = ? returning "id", "name", "slug", "logo", "metadata", "created_at", "issue_links"
```

In 1 of 17 requests:

```sql
update "organization" set "logo" = ?, "metadata" = ? where "organization"."id" = ? returning "id", "name", "slug", "logo", "metadata", "created_at", "issue_links"
```

In 2 of 17 requests:

```sql
update "organization" set "name" = ? where "organization"."id" = ? returning "id", "name", "slug", "logo", "metadata", "created_at", "issue_links"
```
