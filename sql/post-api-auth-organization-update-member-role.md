# POST /api/auth/organization/update-member-role

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 17 | 4 (0–9) | 4 (0–9) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 11 of 17 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 12 of 17 requests, **up to 4 times in one request**:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 11 of 17 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where (("member"."user_id" = ?) and ("member"."organization_id" = ?))
```

In 3 of 17 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where "member"."organization_id" = ? limit ?
```

In 3 of 17 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."id" = ?
```

In 3 of 17 requests:

```sql
update "member" set "role" = ? where "member"."id" = ? returning "id", "organization_id", "user_id", "role", "created_at"
```

In 6 of 17 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where "member"."id" = ?
```
