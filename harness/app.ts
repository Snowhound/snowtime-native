// app/perf/lib/app.ts with startApp's env option and OAuth preload, and freePort exported, as
// snowtime main has them from 54ae90f. Delete this file and import app/perf/lib/app once the
// pin reaches that commit.
import { createClient } from '@libsql/client'
import { type ChildProcess, spawn } from 'node:child_process'
import { cpSync, existsSync, mkdirSync, rmSync, writeFileSync } from 'node:fs'
import { createServer } from 'node:net'
import { join, resolve } from 'node:path'
import { CACHE, ROOT, SEED_NOW } from '../app/perf/lib/database'

export { buildApp, signInHeaders, signedInContext } from '../app/perf/lib/app'
export type { RunningApp } from '../app/perf/lib/app'

const BUILD = join(CACHE, 'build')
const OAUTH_PRELOAD = resolve(import.meta.dir, '../conformance/oauth-preload.ts')

export async function freePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const server = createServer()
    server.unref()
    server.on('error', reject)
    server.listen(0, '127.0.0.1', () => {
      const address = server.address()
      server.close(() => resolve(typeof address === 'object' && address ? address.port : 0))
    })
  })
}

export async function startApp({
  database,
  build = BUILD,
  scene = false,
  executable,
  env = {},
}: {
  database: string
  build?: string
  scene?: boolean
  executable?: string
  env?: Record<string, string>
}) {
  if (!executable && !existsSync(join(build, 'server/index.mjs'))) {
    throw new Error(`[perf] No build in ${build}`)
  }
  const port = await freePort()
  const copy = join(CACHE, `run-${port}.db`)
  cpSync(database, copy)
  const client = createClient({ url: `file:${copy}` })
  await client.execute(
    `update user_settings set scene_intro = 0${scene ? '' : ', scene_background = 0, scene_weather = 0'}`,
  )
  client.close()
  const url = `http://127.0.0.1:${port}`
  const cwd = join(CACHE, 'run')
  mkdirSync(cwd, { recursive: true })
  const log = join(CACHE, `server-${port}.log`)
  writeFileSync(log, '')
  const [command, ...args] = executable
    ? [executable]
    : [
        'bun',
        '--preload',
        join(ROOT, 'perf/lib/clock.ts'),
        ...(env.OAUTH_FAKE_PROVIDER ? ['--preload', OAUTH_PRELOAD] : []),
        join(build, 'server/index.mjs'),
      ]
  const server: ChildProcess = spawn(command, args, {
    cwd,
    stdio: ['ignore', 'pipe', 'pipe'],
    env: {
      PATH: process.env.PATH,
      HOME: process.env.HOME,
      NODE_ENV: 'development',
      PORT: String(port),
      HOST: '127.0.0.1',
      PERF_NOW: String(SEED_NOW.getTime()),
      TURSO_DATABASE_URL: `file:${copy}`,
      BETTER_AUTH_SECRET: 'perf-harness-secret-perf-harness-secret',
      BETTER_AUTH_URL: url,
      ...env,
    },
  })
  const output: string[] = []
  server.stdout?.on('data', (chunk) => output.push(String(chunk)))
  server.stderr?.on('data', (chunk) => output.push(String(chunk)))

  for (let waited = 0; ; waited += 100) {
    if (server.exitCode !== null) {
      throw new Error(`[perf] Server exited ${server.exitCode}:\n${output.join('')}`)
    }
    try {
      const response = await fetch(`${url}/sign-in`)
      if (response.ok) break
    } catch {}
    if (waited > 20_000) {
      server.kill()
      throw new Error(`[perf] Server didn't answer in 20 s:\n${output.join('')}`)
    }
    await Bun.sleep(100)
  }

  return {
    url,
    pid: server.pid!,
    stop: async () => {
      if (server.exitCode !== null) return
      const exited = new Promise((resolve) => server.once('exit', resolve))
      server.kill()
      await exited
      writeFileSync(log, output.join(''))
      rmSync(copy, { force: true })
    },
  }
}
