# POST /api/auth/organization/create

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 24 | 5 (0–15) | 5 (0–15) | 0 | 0 (0–3) |

Per request: the median, and the range when requests differ.

## Statements

In 17 of 24 requests:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 17 of 24 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 17 of 24 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where "member"."user_id" = ? limit ?
```

In 17 of 24 requests, **up to 10 times in one request**:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."id" = ?
```

In 15 of 24 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."slug" = ?
```

In 1 of 24 requests:

```sql
insert into "organization" ("id", "name", "slug", "logo", "metadata", "created_at", "issue_links") values (?, ?, ?, null, ?, ?, null) returning "id", "name", "slug", "logo", "metadata", "created_at", "issue_links"
```

In 9 of 24 requests:

```sql
insert into "member" ("id", "organization_id", "user_id", "role", "created_at") values (?, …) returning "id", "organization_id", "user_id", "role", "created_at"
```

In 1 of 24 requests:

```sql
update "session" set "active_organization_id" = ?, "updated_at" = ? where "session"."token" = ? returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```

In 8 of 24 requests:

```sql
insert into "organization" ("id", "name", "slug", "logo", "metadata", "created_at", "issue_links") values (?, ?, ?, null, null, ?, null) returning "id", "name", "slug", "logo", "metadata", "created_at", "issue_links"
```
