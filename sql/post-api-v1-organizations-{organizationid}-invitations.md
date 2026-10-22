# POST /api/v1/organizations/:organizationId/invitations

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 7 | 14 (2–16) | 14 (2–15) | 0 (0–1) | 0 (0–1) |

Per request: the median, and the range when requests differ.

## Statements

In 3 of 7 requests, **up to 2 times in one request**:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 6 of 7 requests, **up to 3 times in one request**:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 7 of 7 requests:

```sql
select "role" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 7 of 7 requests:

```sql
select "team_member"."team_id" from "team_member" inner join "team" on "team"."id" = "team_member"."team_id" where (("team_member"."user_id" = ?) and ("team_member"."role" = ?) and ("team"."organization_id" = ?))
```

In 6 of 7 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where (("member"."user_id" = ?) and ("member"."organization_id" = ?))
```

In 6 of 7 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."email" = ?
```

In 5 of 7 requests:

```sql
select "id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at" from "invitation" where (("invitation"."email" = ?) and ("invitation"."organization_id" = ?) and ("invitation"."status" = ?)) limit ?
```

In 5 of 7 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."id" = ?
```

In 5 of 7 requests:

```sql
select count(*) from "invitation" where (("invitation"."organization_id" = ?) and ("invitation"."status" = ?) and ("invitation"."expires_at" > ?))
```

In 5 of 7 requests:

```sql
select "id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at" from "invitation" where (("invitation"."organization_id" = ?) and ("invitation"."status" = ?)) limit ?
```

In 5 of 7 requests:

```sql
insert into "invitation" ("id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at") values (?, ?, ?, ?, null, ?, ?, ?, ?) returning "id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at"
```

In 4 of 7 requests:

```sql
select "id", "organization_id", "user_id", "role", "created_at" from "member" where (("member"."organization_id" = ?) and ("member"."user_id" = ?))
```

In 1 of 7 requests, **up to 2 times in one request**:

```sql
select "id" from "team" where (("team"."id" = ?) and ("team"."organization_id" = ?))
```

In 1 of 7 requests:

```sql
update "invitation" set "team_id" = ? where (("invitation"."id" = ?) and ("invitation"."organization_id" = ?))
```
