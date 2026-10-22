# SQL per route

From `app.jsonl`. Calls and statements are medians per request.

| Route | Requests | Calls | Statements | Transactions | Repeated statements |
| --- | --- | --- | --- | --- | --- |
| [DELETE /api/v1/api-keys/:id](delete-api-v1-api-keys-{id}.md) | 3 | 1 | 1 | 0 |  |
| [DELETE /api/v1/organizations/:organizationId/entries/:id](delete-api-v1-organizations-{organizationid}-entries-{id}.md) | 3 | 3 | 3 | 0 |  |
| [DELETE /api/v1/organizations/:organizationId/projects/:id](delete-api-v1-organizations-{organizationid}-projects-{id}.md) | 4 | 4.5 | 3.5 | 0.5 |  |
| [DELETE /api/v1/organizations/:organizationId/projects/:projectId/teams/:teamId](delete-api-v1-organizations-{organizationid}-projects-{projectid}-teams-{teamid}.md) | 1 | 5 | 5 | 0 |  |
| [DELETE /api/v1/organizations/:organizationId/teams/:teamId](delete-api-v1-organizations-{organizationid}-teams-{teamid}.md) | 3 | 3 | 3 | 0 |  |
| [DELETE /api/v1/organizations/:organizationId/teams/:teamId/members/:userId](delete-api-v1-organizations-{organizationid}-teams-{teamid}-members-{userid}.md) | 3 | 6 | 4 | 1 |  |
| [GET /api/auth/callback/:id](get-api-auth-callback-{id}.md) | 36 | 7 | 7 | 0 |  |
| [GET /api/auth/list-accounts](get-api-auth-list-accounts.md) | 15 | 3 | 3 | 0 |  |
| [GET /api/auth/passkey/generate-authenticate-options](get-api-auth-passkey-generate-authenticate-options.md) | 30 | 1 | 1 | 0 |  |
| [GET /api/auth/passkey/generate-register-options](get-api-auth-passkey-generate-register-options.md) | 18 | 2 | 2 | 0 |  |
| [GET /api/auth/passkey/list-user-passkeys](get-api-auth-passkey-list-user-passkeys.md) | 13 | 1 | 1 | 0 |  |
| [GET /api/v1/api-keys](get-api-v1-api-keys.md) | 2 | 0.5 | 0.5 | 0 |  |
| [GET /api/v1/availability](get-api-v1-availability.md) | 1 | 1 | 1 | 0 |  |
| [GET /api/v1/deployment](get-api-v1-deployment.md) | 14 | 0 | 0 | 0 |  |
| [GET /api/v1/dev-users](get-api-v1-dev-users.md) | 1 | 1 | 1 | 0 |  |
| [GET /api/v1/invitations/:id](get-api-v1-invitations-{id}.md) | 4 | 1 | 1 | 0 |  |
| [GET /api/v1/me](get-api-v1-me.md) | 5 | 3 | 3 | 0 |  |
| [GET /api/v1/organizations/:organizationId/entries](get-api-v1-organizations-{organizationid}-entries.md) | 9 | 3 | 3 | 0 |  |
| [GET /api/v1/organizations/:organizationId/entries/first-start](get-api-v1-organizations-{organizationid}-entries-first-start.md) | 1 | 3 | 3 | 0 |  |
| [GET /api/v1/organizations/:organizationId/invitations](get-api-v1-organizations-{organizationid}-invitations.md) | 2 | 3 | 3 | 0 |  |
| [GET /api/v1/organizations/:organizationId/members](get-api-v1-organizations-{organizationid}-members.md) | 9 | 4 | 4 | 0 |  |
| [GET /api/v1/organizations/:organizationId/projects](get-api-v1-organizations-{organizationid}-projects.md) | 10 | 4 | 4 | 0 |  |
| [GET /api/v1/organizations/:organizationId/teams](get-api-v1-organizations-{organizationid}-teams.md) | 7 | 4 | 4 | 0 |  |
| [GET /api/v1/session](get-api-v1-session.md) | 25 | 0 | 0 | 0 |  |
| [GET /api/v1/sign-in-methods](get-api-v1-sign-in-methods.md) | 14 | 0 | 0 | 0 |  |
| [GET /api/v1/timer](get-api-v1-timer.md) | 10 | 1.5 | 1.5 | 0 |  |
| [GET /sign-in](get-sign-in.md) | 13 | 0 | 0 | 0 |  |
| [PATCH /api/v1/organizations/:organizationId/entries/:id](patch-api-v1-organizations-{organizationid}-entries-{id}.md) | 3 | 3 | 3 | 0 |  |
| [PATCH /api/v1/organizations/:organizationId/issue-links](patch-api-v1-organizations-{organizationid}-issue-links.md) | 4 | 2.5 | 2.5 | 0 |  |
| [PATCH /api/v1/organizations/:organizationId/projects/:id](patch-api-v1-organizations-{organizationid}-projects-{id}.md) | 2 | 3.5 | 3.5 | 0 |  |
| [PATCH /api/v1/organizations/:organizationId/teams/:teamId](patch-api-v1-organizations-{organizationid}-teams-{teamid}.md) | 2 | 3 | 3 | 0 |  |
| [PATCH /api/v1/organizations/:organizationId/teams/:teamId/members/:userId](patch-api-v1-organizations-{organizationid}-teams-{teamid}-members-{userid}.md) | 2 | 3.5 | 3.5 | 0 |  |
| [PATCH /api/v1/settings](patch-api-v1-settings.md) | 9 | 1 | 1 | 0 |  |
| [POST /api/auth/callback/:id](post-api-auth-callback-{id}.md) | 3 | 0 | 0 | 0 |  |
| [POST /api/auth/link-social](post-api-auth-link-social.md) | 5 | 3 | 3 | 0 |  |
| [POST /api/auth/organization/cancel-invitation](post-api-auth-organization-cancel-invitation.md) | 9 | 0 | 0 | 0 | 1 |
| [POST /api/auth/organization/check-slug](post-api-auth-organization-check-slug.md) | 6 | 0 | 0 | 0 |  |
| [POST /api/auth/organization/create](post-api-auth-organization-create.md) | 24 | 5 | 5 | 0 | 1 |
| [POST /api/auth/organization/leave](post-api-auth-organization-leave.md) | 8 | 0 | 0 | 0 | 1 |
| [POST /api/auth/organization/remove-member](post-api-auth-organization-remove-member.md) | 14 | 5.5 | 5.5 | 0 | 1 |
| [POST /api/auth/organization/set-active](post-api-auth-organization-set-active.md) | 16 | 2 | 2 | 0 | 1 |
| [POST /api/auth/organization/update](post-api-auth-organization-update.md) | 17 | 4 | 4 | 0 | 1 |
| [POST /api/auth/organization/update-member-role](post-api-auth-organization-update-member-role.md) | 17 | 4 | 4 | 0 | 1 |
| [POST /api/auth/passkey/delete-passkey](post-api-auth-passkey-delete-passkey.md) | 16 | 1 | 1 | 0 |  |
| [POST /api/auth/passkey/verify-authentication](post-api-auth-passkey-verify-authentication.md) | 42 | 4 | 4 | 0 |  |
| [POST /api/auth/passkey/verify-registration](post-api-auth-passkey-verify-registration.md) | 23 | 3 | 3 | 0 |  |
| [POST /api/auth/sign-in/email](post-api-auth-sign-in-email.md) | 26 | 3 | 3 | 0 |  |
| [POST /api/auth/sign-in/social](post-api-auth-sign-in-social.md) | 45 | 1 | 1 | 0 |  |
| [POST /api/auth/sign-out](post-api-auth-sign-out.md) | 1 | 5 | 5 | 0 |  |
| [POST /api/auth/unlink-account](post-api-auth-unlink-account.md) | 7 | 3 | 3 | 0 |  |
| [POST /api/auth/update-user](post-api-auth-update-user.md) | 18 | 2 | 2 | 0 |  |
| [POST /api/v1/api-keys](post-api-v1-api-keys.md) | 4 | 2 | 2 | 0 |  |
| [POST /api/v1/invitations/:id/accept](post-api-v1-invitations-{id}-accept.md) | 5 | 17 | 13 | 2 | 3 |
| [POST /api/v1/organizations/:organizationId/entries](post-api-v1-organizations-{organizationid}-entries.md) | 2 | 4 | 4 | 0 |  |
| [POST /api/v1/organizations/:organizationId/invitations](post-api-v1-organizations-{organizationid}-invitations.md) | 7 | 14 | 14 | 0 | 3 |
| [POST /api/v1/organizations/:organizationId/projects](post-api-v1-organizations-{organizationid}-projects.md) | 4 | 3 | 3 | 0 |  |
| [POST /api/v1/organizations/:organizationId/projects/:id/archive](post-api-v1-organizations-{organizationid}-projects-{id}-archive.md) | 2 | 3.5 | 3.5 | 0 |  |
| [POST /api/v1/organizations/:organizationId/projects/:id/unarchive](post-api-v1-organizations-{organizationid}-projects-{id}-unarchive.md) | 1 | 4 | 4 | 0 |  |
| [POST /api/v1/organizations/:organizationId/report](post-api-v1-organizations-{organizationid}-report.md) | 10 | 6 | 6 | 0 |  |
| [POST /api/v1/organizations/:organizationId/report/breakdown](post-api-v1-organizations-{organizationid}-report-breakdown.md) | 1 | 5 | 5 | 0 |  |
| [POST /api/v1/organizations/:organizationId/report/entries](post-api-v1-organizations-{organizationid}-report-entries.md) | 3 | 5 | 5 | 0 | 1 |
| [POST /api/v1/organizations/:organizationId/report/entry-totals](post-api-v1-organizations-{organizationid}-report-entry-totals.md) | 1 | 5 | 5 | 0 |  |
| [POST /api/v1/organizations/:organizationId/report/export](post-api-v1-organizations-{organizationid}-report-export.md) | 2 | 7 | 7 | 0 | 2 |
| [POST /api/v1/organizations/:organizationId/teams](post-api-v1-organizations-{organizationid}-teams.md) | 3 | 6 | 4 | 1 |  |
| [POST /api/v1/organizations/:organizationId/timer/start](post-api-v1-organizations-{organizationid}-timer-start.md) | 11 | 2 | 2 | 0 |  |
| [POST /api/v1/timer/stop](post-api-v1-timer-stop.md) | 5 | 1 | 1 | 0 |  |
| [PUT /api/v1/organizations/:organizationId/projects/:projectId/teams/:teamId](put-api-v1-organizations-{organizationid}-projects-{projectid}-teams-{teamid}.md) | 2 | 4.5 | 4.5 | 0 |  |
| [PUT /api/v1/organizations/:organizationId/teams/:teamId/members/:userId](put-api-v1-organizations-{organizationid}-teams-{teamid}-members-{userid}.md) | 2 | 7 | 5 | 1 |  |
| [PUT /api/v1/settings](put-api-v1-settings.md) | 3 | 1 | 1 | 0 |  |

Calls outside any request: 0.
