# GET /api/auth/callback/:id

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 36 | 7 (0–10) | 7 (0–10) | 0 | 2 (0–5) |

Per request: the median, and the range when requests differ.

## Statements

In 35 of 36 requests:

```sql
select "id", "identifier", "value", "expires_at", "created_at", "updated_at" from "verification" where "verification"."identifier" = ? order by "verification"."created_at" desc limit ?
```

In 35 of 36 requests:

```sql
select "id", "identifier", "value", "expires_at", "created_at", "updated_at" from "verification" where "verification"."expires_at" < ? limit ?
```

In 35 of 36 requests:

```sql
delete from "verification" where "verification"."expires_at" < ?
```

In 31 of 36 requests:

```sql
select "id", "identifier", "value", "expires_at", "created_at", "updated_at" from "verification" where "verification"."identifier" = ? limit ?
```

In 31 of 36 requests:

```sql
delete from "verification" where "verification"."identifier" = ?
```

In 20 of 36 requests:

```sql
select "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at" from "account" where (("account"."provider_id" = ?) and ("account"."account_id" = ?)) limit ?
```

In 13 of 36 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."email" = ?
```

In 2 of 36 requests:

```sql
select "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at" from "account" where "account"."user_id" = ? limit ?
```

In 9 of 36 requests:

```sql
insert into "account" ("id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at") values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, null, ?, ?) returning "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at"
```

In 15 of 36 requests:

```sql
insert into "session" ("id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at") values (?, ?, ?, ?, ?, ?, null, null, ?, ?) returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```

In 5 of 36 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```

In 5 of 36 requests:

```sql
update "account" set "provider_id" = ?, "access_token" = ?, "refresh_token" = ?, "id_token" = ?, "access_token_expires_at" = ?, "refresh_token_expires_at" = ?, "updated_at" = ? where "account"."id" = ? returning "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at"
```

In 1 of 36 requests:

```sql
update "account" set "provider_id" = ?, "access_token" = ?, "refresh_token" = ?, "id_token" = ?, "access_token_expires_at" = ?, "refresh_token_expires_at" = ?, "scope" = ?, "updated_at" = ? where "account"."id" = ? returning "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at"
```

In 8 of 36 requests:

```sql
insert into "user" ("id", "name", "email", "email_verified", "image", "created_at", "updated_at") values (?, …) returning "id", "name", "email", "email_verified", "image", "created_at", "updated_at"
```

In 1 of 36 requests:

```sql
insert into "account" ("id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at") values (?, ?, ?, ?, ?, ?, ?, null, null, ?, null, ?, ?) returning "id", "user_id", "account_id", "provider_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at"
```
