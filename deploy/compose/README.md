# Deploy with Docker Compose

`compose.yml` runs the native server on one machine with Docker Compose, with Caddy in front
for HTTPS. Litestream can stream the database to S3-compatible storage, such as Hetzner
Object Storage, Cloudflare R2, or MinIO. The **Compose deploy** workflow
(`.github/workflows/compose-deploy.yml`) builds the images, pushes them to GHCR, and deploys
them over SSH. You can also build the images yourself and run Compose by hand.

| Service      | Image                   | Does                                                     |
| ------------ | ----------------------- | -------------------------------------------------------- |
| `app`        | `snowtime-native`       | Serves pages and the API on port 3000; migrates on start |
| `caddy`      | `snowtime-native-caddy` | HTTPS on 80 and 443, the same build's public files       |
| `litestream` | `litestream/litestream` | Streams the database to a bucket; only with backups on   |

The app's [own Compose deployment](https://github.com/Snowhound/snowtime/blob/main/docs/deployment/compose.md)
runs the TypeScript server the same way. Both use the Compose project `snowtime`, the same
service names, and the same database schema, so either one can replace the other on a
server and keep its data ([Switch from the TypeScript stack](#switch-from-the-typescript-stack)).

## Requirements

- A Linux server, x64 or Arm64, with Docker Engine and the Compose plugin. The app's
  [server requirements](https://github.com/Snowhound/snowtime/blob/main/docs/deployment/self-hosted.md#server-requirements)
  apply.
- A domain whose DNS points at the server, with inbound TCP 80 and 443, and UDP 443 for
  HTTP/3. Caddy gets the certificate from Let's Encrypt.
- At least one OAuth app for the domain
  ([Register the OAuth apps](https://github.com/Snowhound/snowtime/blob/main/docs/deployment/README.md#register-the-oauth-apps)).
  Outside demo mode, people sign in with OAuth or a passkey; there is no password sign-up.
- For backups, an S3-compatible bucket and an access key for it: endpoint URL, region,
  bucket name, key ID, and secret. Put the bucket in another location than the server.

The workflow builds x64 images only. On an Arm64 server, build the images yourself
([Build the images yourself](#build-the-images-yourself)).

## 1. Write the settings

On the server, create the deploy directory, such as `~/snowtime`, and in it `.env` with
`chmod 600`:

```dotenv
APP_HOST=time.example.com
RELEASE=
BETTER_AUTH_SECRET=<openssl rand -hex 32>
GITHUB_CLIENT_ID=...
GITHUB_CLIENT_SECRET=...
# Turns on the litestream service; leave it out to run without backups.
COMPOSE_PROFILES=backup
```

Keep `BETTER_AUTH_SECRET` across deploys; changing it signs everyone out. The other
variables the app reads are listed in
[Set the environment variables](https://github.com/Snowhound/snowtime/blob/main/docs/deployment/README.md#set-the-environment-variables).
Compose fixes the database URL, the public URL (`https://$APP_HOST`), migrations on
start, and the client IP header. Leave out `TURSO_AUTH_TOKEN` and the Upstash variables:
the database is a file in the `app_data` volume, and rate-limit counts stay in the
process.

| Optional variable | Default             | Sets                                               |
| ----------------- | ------------------- | -------------------------------------------------- |
| `IMAGE_REGISTRY`  | `ghcr.io/snowhound` | Where the images come from; the workflow sets it   |
| `DATABASE_VOLUME` | `snowtime_data`     | The database volume's name                         |
| `DEMO_MODE`       | unset               | `true` allows seeded password sign-in and no OAuth |

With backups on, create `litestream.env` beside it, also `chmod 600`. Only the
`litestream` service reads it, so the app never sees the bucket's keys:

```dotenv
LITESTREAM_ENDPOINT=https://fsn1.your-objectstorage.com
LITESTREAM_REGION=fsn1
LITESTREAM_BUCKET=<bucket>
LITESTREAM_ACCESS_KEY_ID=...
LITESTREAM_SECRET_ACCESS_KEY=...
```

Caddy trusts the client address in `CF-Connecting-IP` only from Cloudflare's ranges and
sets the header for the app itself, so the setup works with or without Cloudflare in
front.

## 2. Set up the deploy workflow

Skip this step to build and copy the images yourself.

Let your SSH user run Docker without sudo (`sudo usermod -aG docker "$USER"`, then log in
again); membership in the `docker` group is equivalent to root. On your own machine, make
a key for the workflow and authorize it on the server:

```bash
ssh-keygen -t ed25519 -N '' -C snowtime-compose-deploy -f snowtime-deploy
ssh-copy-id -i snowtime-deploy.pub <user>@<server>
ssh-keyscan -t ed25519 <server>
```

In your fork's **Settings > Secrets and variables > Actions**, add these repository
secrets, then delete the local key files:

| Secret                    | Value                                                 |
| ------------------------- | ----------------------------------------------------- |
| `COMPOSE_DEPLOY_HOST`     | The server's IP address or SSH hostname               |
| `COMPOSE_DEPLOY_USER`     | Your SSH user                                         |
| `COMPOSE_DEPLOY_PATH`     | The deploy directory, such as `/home/<user>/snowtime` |
| `COMPOSE_SSH_PRIVATE_KEY` | The contents of `snowtime-deploy`                     |
| `COMPOSE_SSH_KNOWN_HOSTS` | The `ssh-keyscan` line                                |

GitHub's runners connect from changing addresses, so the server's firewall must allow SSH
from anywhere; keep password login off. Anyone who can run workflows in the repository
can read these secrets.

## 3. Deploy

In the **Actions** tab, open **Compose deploy**, choose **Run workflow**, and pick the
branch. From a terminal:

```bash
gh workflow run compose-deploy.yml --ref main -f server=restart
```

The workflow builds the render bundle, then the server and Caddy images, tagged with the
commit's short ID, and pushes them to `ghcr.io/<owner>`. It copies `compose.yml` and
`litestream.yml` to the deploy directory, sets `RELEASE` and `IMAGE_REGISTRY` in `.env`,
and pulls the images with the job's token. With **restart**, it stops the app, starts the
stack, and fails unless the app turns healthy. **pull only** stops before that, so you
choose when the new images start. The first build takes longer, while the Rust build
has no cache.

GHCR keeps new packages private. To let a server pull without a token, or others pull
your images, make the packages public under the owner's **Packages**.

The first start creates and migrates an empty database. Open `https://<APP_HOST>` and
sign in with an OAuth provider.

## Build the images yourself

From the repository root, after building the render bundle as in the main
[README](../../README.md#server-rendering):

```bash
docker build -f deploy/Dockerfile --target app -t <registry>/snowtime-native:<release> .
docker build -f deploy/Dockerfile --target caddy -t <registry>/snowtime-native-caddy:<release> .
```

Push them to a registry the server can reach, or copy them with
`docker save | ssh <server> docker load`. Copy `compose.yml` and `litestream.yml` to the
deploy directory, set `RELEASE` and `IMAGE_REGISTRY` in `.env`, and start the stack:

```bash
docker compose up -d --wait
docker compose ps
```

## Backups

With `COMPOSE_PROFILES=backup`, Litestream copies each change to the bucket within a
second and writes a full snapshot a day, kept for a week (`litestream.yml`). A restore can
go back to any moment in that week. Run one Litestream per bucket path: two servers
writing to the same path can make it impossible to restore.

List what the bucket holds:

```bash
docker compose exec litestream litestream ltx /data/snowtime.db
```

### Restore

On a fresh server, or after losing the volume, restore before the app first starts:

1. Stop the old server's stack, if it still runs, so only one server writes to the bucket.
2. Write `.env` and `litestream.env` as in step 1, and copy `compose.yml`,
   `litestream.yml`, and the images as in step 3 with **pull only**.
3. Create the volume and restore into it. The app's container creates the volume with
   the ownership Litestream needs:

   ```bash
   docker compose create app
   docker compose run --rm --no-deps litestream restore -if-replica-exists /data/snowtime.db
   docker compose up -d --wait
   ```

4. Point the DNS record at the new server.

To look at an earlier state without touching the database, restore to another file in the
volume, then copy it out:

```bash
docker compose run --rm --no-deps litestream restore -timestamp 2026-10-11T08:00:00Z -o /data/then.db /data/snowtime.db
```

## Update and roll back

Run the workflow again on the branch to deploy its latest commit. Each deploy sets a new
`RELEASE`, so earlier images stay on the server and in GHCR.

To roll back, set `RELEASE` in `.env` to an earlier commit's short ID and run
`docker compose up -d --no-build --force-recreate --wait`. A rollback doesn't undo schema
changes, so roll back only to a release whose migrations the database already has.

Don't run a second app against the volume, and don't use `docker compose down -v`: it
deletes the database and Caddy's certificates.

## Switch from the TypeScript stack

A server that runs the app's Compose deployment can switch to this one and back. Both
stacks name their project `snowtime`, so starting one replaces the other's containers.
Both use the same database volume and Caddy's certificate volumes. Only one runs at a
time.

1. Stop the TypeScript stack's automatic deploys: set the app repository's variable
   `COMPOSE_AUTO_DEPLOY` to `false`. Otherwise its next deploy switches back.
2. Check that this repository's pin has the same migrations as the running app. The pin's
   `app/drizzle` must hold every migration the database has; the server refuses to start
   on a migration it doesn't know.
3. Create a second deploy directory, such as `~/snowtime-native`, with a copy of the
   first one's `.env`. Keep `DATABASE_VOLUME`; it names the shared volume.
4. Back up the volume:

   ```bash
   docker run --rm -v <DATABASE_VOLUME>:/data -v "$PWD":/backup debian:trixie-slim \
     tar czf /backup/snowtime-data.tgz -C /data .
   ```

5. Deploy with the workflow and **restart**, which replaces the running stack. To choose
   the moment yourself, deploy with **pull only**, then run
   `docker compose up -d --no-build --force-recreate --wait` in the new directory.

To switch back, run the same command in the TypeScript stack's directory. Its `.env`
still names its last release.
