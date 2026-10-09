// Runs conformance tests against a native server: serves it a copy of the benchmark database
// with its clock at SEED_NOW and password sign-in on (app/conformance/server.ts), runs the
// tests with CONFORMANCE_URL, and stops it. The OAuth and login-domain files need their own
// setup, so each file runs against its own server. Without files, it runs every file: the
// app's, and the port's conformance/ folder, whose file replaces the app's of the same name.
//
//   bun harness/conformance.ts target/debug/snowtime-axum
//   bun harness/conformance.ts <binary> conformance/oauth.conformance.ts app/conformance/timer.conformance.ts

import { spawn, spawnSync } from 'node:child_process'
import { readdirSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { LOGIN_DOMAINS_ENV, loginDomainsDatabase } from '../conformance/login-domains-fixture'
import { oauthEnv, startOAuthProvider } from '../conformance/oauth-provider'
import { ROOT, seededDatabase } from '../app/perf/lib/database'
import { startNative } from './native'

const [binary, ...files] = process.argv.slice(2)
if (!binary) throw new Error('Usage: bun harness/conformance.ts <binary> [test files]')
if (files.length === 0) process.exit(runAll(binary))

function runAll(binary: string) {
  const isTest = (name: string) => name.endsWith('.conformance.ts')
  const port = resolve(import.meta.dir, '../conformance')
  const own = readdirSync(port).filter(isTest)
  const app = readdirSync(join(ROOT, 'conformance')).filter(
    (name) => isTest(name) && !own.includes(name),
  )
  const all = [
    ...app.map((name) => join(ROOT, 'conformance', name)),
    ...own.map((name) => join(port, name)),
  ].sort((a, b) => a.split('/').at(-1)!.localeCompare(b.split('/').at(-1)!))
  const failed = all.filter(
    (file) => spawnSync('bun', [import.meta.path, binary, file], { stdio: 'inherit' }).status !== 0,
  )
  for (const file of failed) console.error(`[conformance] failed: ${file}`)
  return failed.length > 0 ? 1 : 0
}

const provider = files.some((file) => file.includes('oauth.conformance'))
  ? startOAuthProvider()
  : undefined
const domains = files.some((file) => file.includes('login-domains.conformance'))
const server = await startNative(
  binary,
  domains ? await loginDomainsDatabase(await seededDatabase()) : await seededDatabase(),
  provider ? oauthEnv(provider.url) : domains ? LOGIN_DOMAINS_ENV : {},
)
let code = 1
try {
  const tests = files.map((file) => resolve(file))
  const cwd = resolve(import.meta.dir, '..')
  code = await new Promise<number>((resolve, reject) => {
    const child = spawn(
      'bun',
      [
        'test',
        ...tests,
        ...(process.env.CONFORMANCE_TEST_NAME_PATTERN
          ? ['-t', process.env.CONFORMANCE_TEST_NAME_PATTERN]
          : []),
      ],
      {
        cwd,
        stdio: 'inherit',
        env: {
          ...process.env,
          CONFORMANCE_URL: server.url,
          ...(provider ? { OAUTH_FAKE_PROVIDER: provider.url } : {}),
        },
      },
    )
    child.once('error', reject)
    child.once('exit', (status) => resolve(status ?? 1))
  })
} finally {
  await server.stop()
  await provider?.stop()
}
process.exit(code)
