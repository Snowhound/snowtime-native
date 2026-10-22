# POST /api/auth/organization/cancel-invitation

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 9 | 0 (0–7) | 0 (0–7) | 0 | 0 |

Per request: the median, and the range when requests differ.

## Statements

In 4 of 9 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 4 of 9 requests, **up to 2 times in one request**:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 4 of 9 requests:

```sql
select "id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at" from "invitation" where "invitation"."id" = ?
```

In 3 of 9 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where (("member"."user_id" = ?) and ("member"."organization_id" = ?))
```

In 2 of 9 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."id" = ?
```

In 2 of 9 requests:

```sql
update "invitation" set "status" = ? where (("invitation"."id" = ?) and ("invitation"."id" in (select "id" from "invitation" where "invitation"."id" = ? limit ?))) returning "id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at"
```
