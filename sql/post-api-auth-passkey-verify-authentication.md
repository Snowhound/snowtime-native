# POST /api/auth/passkey/verify-authentication

| Requests | Calls | Statements | Transactions | Writes outside a transaction |
| --- | --- | --- | --- | --- |
| 42 | 4 (0–7) | 4 (0–7) | 0 | 2 (0–4) |

Per request: the median, and the range when requests differ.

## Statements

In 36 of 42 requests:

```sql
select "id", "identifier", "value", "expires_at", "created_at", "updated_at" from "verification" where "verification"."identifier" = ? order by "verification"."created_at" desc limit ?
```

In 24 of 42 requests:

```sql
delete from "verification" where "verification"."id" in (select "id" from "verification" where "verification"."id" = ? limit ?) returning "id", "identifier", "value", "expires_at", "created_at", "updated_at"
```

In 24 of 42 requests:

```sql
delete from "verification" where "verification"."identifier" = ?
```

In 24 of 42 requests:

```sql
select "id", "name", "public_key", "user_id", "credential_id", "counter", "device_type", "backed_up", "transports", "created_at", "aaguid" from "passkey" where "passkey"."credential_id" = ?
```

In 6 of 42 requests:

```sql
update "passkey" set "counter" = ? where "passkey"."id" = ? returning "id", "name", "public_key", "user_id", "credential_id", "counter", "device_type", "backed_up", "transports", "created_at", "aaguid"
```

In 6 of 42 requests:

```sql
insert into "session" ("id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at") values (?, ?, ?, ?, ?, ?, null, null, ?, ?) returning "id", "user_id", "token", "expires_at", "ip_address", "user_agent", "active_organization_id", "active_team_id", "created_at", "updated_at"
```

In 6 of 42 requests:

```sql
select "id", "name", "email", "email_verified", "image", "created_at", "updated_at" from "user" where "user"."id" = ?
```
