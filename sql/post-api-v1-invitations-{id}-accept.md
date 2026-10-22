# POST /api/v1/invitations/:id/accept

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 5 | 17 (2–18) | 13 (2–14) | 2 (0–2) | 3 (0–3) |

Per request: the median, and the range when requests differ.

## Statements

In 2 of 5 requests, **up to 2 times in one request**:

```sql
select "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at" from "session" where "session"."token" = ?
```

In 2 of 5 requests, **up to 2 times in one request**:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 5 of 5 requests:

```sql
select "d0"."id" as "id", "d0"."organization_id" as "organizationId", "d0"."email" as "email", "d0"."role" as "role", "d0"."team_id" as "teamId", "d0"."status" as "status", "d0"."expires_at" as "expiresAt", "d0"."inviter_id" as "inviterId", "d0"."created_at" as "createdAt" from "invitation" as "d0" where "d0"."id" = ? limit ?
```

In 3 of 5 requests:

```sql
select "d0"."id" as "id", "d0"."organization_id" as "organizationId", "d0"."user_id" as "userId", "d0"."role" as "role", "d0"."created_at" as "createdAt" from "member" as "d0" where (("d0"."organization_id" = ?) and ("d0"."user_id" = ?)) limit ?
```

In 5 of 5 requests:

```sql
select "id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at" from "invitation" where "invitation"."id" = ?
```

In 3 of 5 requests:

```sql
select count(*) from "member" where "member"."organization_id" = ?
```

In 3 of 5 requests:

```sql
select "id", "name", "slug", "logo", "metadata", "created_at", "issue_links" from "organization" where "organization"."id" = ?
```

In 3 of 5 requests:

```sql
update "invitation" set "status" = ? where (((("invitation"."id" = ?) and ("invitation"."status" = ?))) and ("invitation"."id" in (select "id" from "invitation" where (("invitation"."id" = ?) and ("invitation"."status" = ?)) limit ?))) returning "id", "organization_id", "email", "role", "team_id", "status", "expires_at", "inviter_id", "created_at"
```

In 3 of 5 requests:

```sql
insert into "member" ("id", "organization_id", "user_id", "role", "created_at") values (?, …) returning "id", "organization_id", "user_id", "role", "created_at"
```

In 3 of 5 requests:

```sql
update "session" set "active_organization_id" = ?, "updated_at" = ? where "session"."token" = ? returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```

In 3 of 5 requests:

```sql
select "d0"."id" as "id", "d0"."organization_id" as "organizationId", "d0"."email" as "email", "d0"."role" as "role", "d0"."team_id" as "teamId", "d0"."status" as "status", "d0"."expires_at" as "expiresAt", "d0"."inviter_id" as "inviterId", "d0"."created_at" as "createdAt" from "invitation" as "d0" where (("d0"."id" = ?) and ("d0"."status" = ?)) limit ?
```

In 3 of 5 requests:

```sql
select "team_id" from "invitation" where (("invitation"."id" = ?) and ("invitation"."status" = ?))
```

In 1 of 5 requests, **up to 2 times in one request**:

```sql
insert into "team_member" ("id", "team_id", "user_id", "role", "membership_key", "created_at") values (?, ?, ?, ?, null, ?) on conflict do nothing returning "id"
```

In 1 of 5 requests:

```sql
update "team" set "member_count" = "team"."member_count" + 1 where "team"."id" = ?
```
