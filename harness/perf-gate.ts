import { spawnSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import { cpSync, existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { join, relative, resolve } from 'node:path'
import { parseArgs } from 'node:util'
import { signInHeaders } from '../app/perf/lib/app'
import { CACHE, ROOT, seededDatabase } from '../app/perf/lib/database'
import { checkOtherSessions, reserveStack } from '../app/perf/stress/lock'
import { droppedActions } from '../app/perf/stress/metrics'
import { LOCAL_SECRET } from '../app/perf/stress/stack'
import { machineCommands } from './gate/machine'
import { compareMetric } from './gate/metrics'
import { defaultProfile, PROFILES } from './gate/profiles'
import { processUsage, startNative } from './native'
import { nativeHotTimings } from './timings'

const { values } = parseArgs({
  options: {
    freeze: { type: 'boolean', default: false },
    'build-current': { type: 'boolean', default: false },
    inside: { type: 'boolean', default: false },
    identical: { type: 'boolean', default: false },
    calibrate: { type: 'boolean', default: false },
    help: { type: 'boolean', default: false },
    out: { type: 'string' },
    profile: { type: 'string', default: defaultProfile() },
    renders: { type: 'string' },
    repeats: { type: 'string', default: '1000' },
    rate: { type: 'string', default: '30' },
  },
})
if (values.help) {
  console.log(
    `perf-gate.ts --freeze | --build-current | --calibrate | [--identical] [--profile=${Object.keys(PROFILES).join('|')} --renders=N --repeats=1000 --rate=30]`,
  )
  process.exit(0)
}
const profile = PROFILES[values.profile]
if (!profile) throw new Error(`Unknown profile ${values.profile}`)
const renders = values.renders ?? String(profile.renders)
for (const value of [renders, values.repeats, values.rate])
  if (!Number.isInteger(Number(value)) || Number(value) < 1)
    throw new Error('Counts and rate must be positive integers')
const PORT = resolve(import.meta.dir, '..')
const gate = join(CACHE, 'native-gate')
const MODULES_VOLUME = 'snowtime-perf-gate-node-modules'
// Calibration stops here rather than retrying while the noise keeps flagging metrics.
const MAXIMUM_ATTEMPTS = 6
const out = values.out ?? join(gate, 'runs', new Date().toISOString().replaceAll(':', '-'))
mkdirSync(out, { recursive: true })
const commands: unknown[] = []
// Commands run in this repository; app commands pass cwd: ROOT, the app/ submodule.
function command(
  program: string,
  args: string[],
  log: string,
  env: Record<string, string> = {},
  cwd = PORT,
) {
  commands.push({ program, args, env, cwd, started: new Date().toISOString() })
  writeFileSync(join(out, 'commands.json'), JSON.stringify(commands, null, 2))
  const result = spawnSync(program, args, {
    cwd,
    encoding: 'utf8',
    maxBuffer: 128 * 1024 * 1024,
    env: { ...process.env, ...env },
  })
  writeFileSync(join(out, log), result.stdout + result.stderr)
  if (result.status !== 0) throw new Error(`${program} exited ${result.status}; see ${log}`)
  return result.stdout
}
// The images' build context can't reach the better-auth-rs fork beside this repository,
// which Cargo.toml patches in by path until it is pushed (task 081.39), so the builds get
// its committed tree, exported into the run's evidence.
function forkContext() {
  if (!readFileSync(join(PORT, 'Cargo.toml'), 'utf8').includes('path = "../better-auth-rs"')) {
    return []
  }
  const tree = join(out, 'better-auth-rs')
  mkdirSync(tree)
  command('git', ['-C', '../better-auth-rs', 'rev-parse', 'HEAD'], 'fork-commit.txt')
  command(
    'sh',
    ['-c', 'git -C ../better-auth-rs archive HEAD | tar -x -C "$1"', 'sh', tree],
    'fork-export.log',
  )
  return ['--build-context', `better-auth-rs=${tree}`]
}
function json(path: string) {
  return JSON.parse(readFileSync(path, 'utf8'))
}
function hash(path: string) {
  return createHash('sha256').update(readFileSync(path)).digest('hex')
}
function seal(directory: string, files: string[]) {
  return Object.fromEntries(
    files.map((file) => [
      file,
      { sha256: hash(join(directory, file)), bytes: statSync(join(directory, file)).size },
    ]),
  )
}
function snapshot(name: string) {
  const dest = join(gate, name)
  if (name === 'baseline' && existsSync(dest))
    throw new Error('Baseline already exists; keep it unchanged')
  mkdirSync(dest, { recursive: true })
  for (const [image, source, file] of [
    ['snowtime-native:bench', '/app/snowtime', 'server'],
    ['snowtime-kit-render:bench', '/usr/local/bin/render-bench', 'render-bench'],
  ]) {
    const container = command('docker', ['create', image], `${name}-${file}-create.txt`).trim()
    try {
      command(
        'docker',
        ['cp', `${container}:${source}`, join(dest, file)],
        `${name}-${file}-copy.txt`,
      )
    } finally {
      command('docker', ['rm', container], `${name}-${file}-remove.txt`)
    }
  }
  cpSync(
    join(PORT, 'adapter/dist/render.shared.js'),
    join(dest, 'render.shared.js'),
  )
  writeFileSync(
    join(dest, 'artifact.json'),
    JSON.stringify(
      {
        commit: command('git', ['rev-parse', 'HEAD'], `${name}-commit.txt`).trim(),
        diff: command('git', ['diff', '--submodule=diff', 'HEAD'], `${name}-diff.patch`),
        images: JSON.parse(
          command(
            'docker',
            ['image', 'inspect', 'snowtime-native:bench', 'snowtime-kit-render:bench'],
            `${name}-images.json`,
          ),
        ),
        files: seal(dest, ['server', 'render-bench', 'render.shared.js']),
      },
      null,
      2,
    ),
  )
}
async function replay(server: { url: string; pid: number }, round: string) {
  const inputs = join(gate, 'inputs')
  const summaryPath = join(out, `${round}-load-summary.json`)
  const ownCpu = process.cpuUsage()
  const before = processUsage(server.pid)
  const began = performance.now()
  const generator = Bun.spawn(
    [
      'k6',
      'run',
      '--quiet',
      '--summary-export',
      summaryPath,
      join(ROOT, 'perf/stress/scenario.js'),
    ],
    {
      env: {
        ...process.env,
        ORIGIN: server.url,
        SECRET: LOCAL_SECRET,
        RECORDING: join(inputs, 'api-slice.json'),
        USERS: join(inputs, 'M.users.json'),
        PLAN: JSON.stringify([
          {
            name: 'gate',
            action: 'gate-slice',
            users: 0,
            perHour: Number(values.rate) * 3600,
            seconds: 10,
            start: 1,
          },
        ]),
        PREALLOCATED_VUS: '50',
        MAX_VUS: '50',
      },
      stdout: 'pipe',
      stderr: 'pipe',
    },
  )
  const stdout = new Response(generator.stdout).text()
  const stderr = new Response(generator.stderr).text()
  let done = false
  const exited = generator.exited.then((code) => {
    done = true
    return code
  })
  const samples: unknown[] = []
  let previous = processUsage(generator.pid)
  let sampled = performance.now()
  let maximumCpu = 0
  let peakSwap = 0
  let maximumMemory = 0
  for (;;) {
    if (done) break
    await Bun.sleep(250)
    if (done) break
    const now = performance.now()
    const usage = processUsage(generator.pid)
    const serverUsage = processUsage(server.pid)
    const clientUsage = processUsage(process.pid)
    if (now - began >= 1250)
      maximumCpu = Math.max(maximumCpu, (usage.cpuMs - previous.cpuMs) / (now - sampled))
    peakSwap = Math.max(peakSwap, usage.swapKiB, serverUsage.swapKiB, clientUsage.swapKiB)
    maximumMemory = Math.max(maximumMemory, usage.rssMiB + serverUsage.rssMiB + clientUsage.rssMiB)
    samples.push({
      t: now,
      startup: now - began < 1250,
      generator: usage,
      server: serverUsage,
      client: clientUsage,
    })
    previous = usage
    sampled = now
  }
  const code = await exited
  const elapsed = (performance.now() - began) / 1000
  const after = processUsage(server.pid)
  writeFileSync(join(out, `${round}-generator.log`), (await stdout) + (await stderr))
  writeFileSync(join(out, `${round}-load-samples.json`), JSON.stringify(samples))
  // --summary-export writes each metric's statistics flat; droppedActions and the checks
  // below read handleSummary's shape, which nests them under `values`.
  const summary = {
    metrics: Object.fromEntries(
      Object.entries(json(summaryPath).metrics as Record<string, Record<string, number>>).map(
        ([name, values]) => [name, { values }],
      ),
    ),
  }
  const requests = summary.metrics.http_reqs?.values?.count ?? 0
  const errors = Object.entries(summary.metrics).some(
    ([name, metric]) =>
      name.startsWith('bench_errors') &&
      (metric as { values: { count?: number } }).values.count! > 0,
  )
  const clientCpu = process.cpuUsage(ownCpu)
  const actionP95 = summary.metrics['bench_action_duration{step:gate}']?.values?.['p(95)']
  const neededVUs = (actionP95 / 1000) * Number(values.rate)
  writeFileSync(
    join(out, `${round}-generator-validity.json`),
    JSON.stringify(
      {
        maximumCpu,
        maximumMemoryMiB: maximumMemory,
        estimatedVUsAtP95: neededVUs,
        allocatedVUs: 50,
        elapsed,
        peakSwapKiB: peakSwap,
      },
      null,
      2,
    ),
  )
  if (
    code !== 0 ||
    !requests ||
    droppedActions(summary.metrics, 'gate') ||
    errors ||
    maximumCpu > 0.7 ||
    maximumMemory > 1536 ||
    !Number.isFinite(neededVUs) ||
    neededVUs > 35 ||
    (clientCpu.user + clientCpu.system) / 1e6 / elapsed > 0.2 ||
    peakSwap ||
    after.swapKiB ||
    elapsed > 13
  )
    throw new Error(
      `${round}: invalid generator/replay (exit=${code}, requests=${requests}, max generator CPU=${maximumCpu}, elapsed=${elapsed})`,
    )
  return {
    metrics: { cpu_ms: (after.cpuMs - before.cpuMs) / requests, peak_rss_mib: after.peakMiB },
    cpuResolution: after.tickMs / requests,
  }
}
async function inside() {
  const inputs = join(gate, 'inputs')
  const all: Record<string, Record<string, number>> = {}
  // The coarsest /proc resolution each CPU metric had in any round, doubled: each round's
  // reading can be off by a tick, so two rounds can differ by two.
  const resolutions: Record<string, number> = {}
  function resolution(metric: string, value: number) {
    resolutions[metric] = Math.max(resolutions[metric] ?? 0, 2 * value)
  }
  for (const [index, name] of ['baseline', 'current', 'baseline', 'current'].entries()) {
    const round = `${index + 1}-${name}`
    const artifact = join(gate, values.identical ? 'baseline' : name)
    const manifest = json(join(artifact, 'artifact.json'))
    for (const file of Object.keys(manifest.files))
      if (hash(join(artifact, file)) !== manifest.files[file].sha256)
        throw new Error(`Changed retained ${name} artifact ${file}`)
    const metrics: Record<string, number> = {}
    for (const page of ['timer', 'week', 'month', 'year']) {
      const result = JSON.parse(
        command(
          join(artifact, 'render-bench'),
          [
            join(inputs, `${page}.json`),
            join(inputs, 'answers.json'),
            renders,
            '1',
            '1',
            join(out, `${round}-${page}.html`),
          ],
          `${round}-${page}.json`,
          {
            RENDER_HEAP_MB: '128',
            RENDER_SEMI_MB: '32',
            RENDER_BENCH_SKIP_FINAL_IDLE: '1',
          },
        ),
      )
      for (const metric of ['cpu_ms', 'p50_ms', 'p95_ms', 'peak_rss_mb'])
        metrics[`render/${page}/${metric}`] = result[metric]
    }
    const env = {
      RATE_LIMIT: 'off',
      EDGE_ACCESS_LOG: 'off',
      DB_READ_CONNECTIONS: '0',
      RENDERERS: '1',
    }
    const api = await startNative(join(artifact, 'server'), join(inputs, 'api.db'), env)
    try {
      const result = await nativeHotTimings(
        api,
        await signInHeaders(api, 'admin'),
        Number(values.repeats),
      )
      writeFileSync(join(out, `${round}-api.json`), JSON.stringify(result, null, 2))
      for (const [call, times] of Object.entries(result)) {
        for (const [metric, value] of Object.entries(times))
          metrics[`api/${call}/${metric}`] = value
        resolution(`api/${call}/cpu_ms`, processUsage(process.pid).tickMs / Number(values.repeats))
      }
    } finally {
      await api.stop()
    }
    const load = await startNative(join(artifact, 'server'), join(inputs, 'M.db'), {
      ...env,
      BETTER_AUTH_SECRET: LOCAL_SECRET,
    })
    try {
      const result = await replay(load, round)
      for (const [metric, value] of Object.entries(result.metrics))
        metrics[`load/${metric}`] = value
      resolution('load/cpu_ms', result.cpuResolution)
    } finally {
      await load.stop()
    }
    metrics['static/binary_bytes'] = statSync(join(artifact, 'server')).size
    metrics['static/render_bundle_bytes'] = statSync(join(artifact, 'render.shared.js')).size
    all[round] = metrics
    writeFileSync(join(out, 'rounds.json'), JSON.stringify(all, null, 2))
  }
  const comparison = Object.fromEntries(
    Object.keys(all['1-baseline']).map((metric) => [
      metric,
      compareMetric(
        [all['1-baseline'][metric], all['3-baseline'][metric]],
        [all['2-current'][metric], all['4-current'][metric]],
        metric,
        { render: profile.renderFloor, resolution: resolutions[metric] },
        profile.statistic,
      ),
    ]),
  )
  writeFileSync(join(out, 'comparison.json'), JSON.stringify(comparison, null, 2))
  const rows = [
    '| Metric | Baseline | Current | Delta | Noise band | Gate |',
    '| --- | ---: | ---: | ---: | ---: | --- |',
    ...Object.entries(comparison).map(
      ([metric, r]) =>
        `| ${metric} | ${r.baseline.toFixed(3)} | ${r.current.toFixed(3)} | ${r.deltaPercent === null ? r.delta.toFixed(3) : `${r.deltaPercent.toFixed(1)}%`} | ${r.band.toFixed(3)} | ${r.flagged ? 'REGRESSION' : 'pass'} |`,
    ),
  ]
  writeFileSync(join(out, 'table.md'), rows.join('\n') + '\n')
  console.log(rows.join('\n'))
  if (Object.values(comparison).some((r) => r.flagged)) process.exitCode = 1
}
async function main() {
  if (values.inside) {
    await inside()
    return
  }
  if (values.calibrate) {
    const attempts: { folder: string; exitCode: number | null; flagged: string[] }[] = []
    let streak = 0
    function recordCalibration(status: string) {
      writeFileSync(
        join(out, 'calibration.json'),
        JSON.stringify(
          {
            status,
            identical: true,
            profile: values.profile,
            requiredConsecutivePasses: 2,
            maximumAttempts: MAXIMUM_ATTEMPTS,
            consecutivePasses: streak,
            attempts,
          },
          null,
          2,
        ),
      )
    }
    recordCalibration('running')
    while (streak < 2) {
      if (attempts.length === MAXIMUM_ATTEMPTS) {
        recordCalibration('failed')
        console.error(
          `No two consecutive passes in ${MAXIMUM_ATTEMPTS} attempts; the noise exceeds this profile's floors. Evidence: ${out}`,
        )
        process.exitCode = 1
        return
      }
      const folder = join(out, `attempt-${attempts.length + 1}`)
      const args = [
        'harness/perf-gate.ts',
        '--identical',
        `--out=${folder}`,
        `--profile=${values.profile}`,
        `--renders=${renders}`,
        `--repeats=${values.repeats}`,
        `--rate=${values.rate}`,
      ]
      const result = spawnSync('bun', args, { cwd: PORT, stdio: 'inherit' })
      const comparison = existsSync(join(folder, 'comparison.json'))
        ? json(join(folder, 'comparison.json'))
        : {}
      const flagged = Object.entries(comparison)
        .filter(([, result]) => (result as { flagged: boolean }).flagged)
        .map(([metric]) => metric)
      attempts.push({ folder: relative(PORT, folder), exitCode: result.status, flagged })
      streak = result.status === 0 ? streak + 1 : 0
      recordCalibration(streak === 2 ? 'passed' : 'running')
      if (result.status !== 0 && result.status !== 1) {
        recordCalibration('invalid')
        throw new Error(
          'Invalid calibration attempt; inspect its excluded evidence before rerunning',
        )
      }
    }
    console.log(`Two consecutive identical comparisons passed. Evidence: ${out}`)
    return
  }
  checkOtherSessions()
  const release = reserveStack()
  try {
    if (values.freeze) {
      const prepared = json(join(CACHE, 'kit-baseline/prepared.json'))
      if (prepared.commit !== command('git', ['rev-parse', 'HEAD'], 'freeze-commit.txt').trim())
        throw new Error('Run kit-baseline.ts --prepare at this commit before freezing')
      snapshot('baseline')
      const inputs = join(gate, 'inputs')
      mkdirSync(inputs, { recursive: true })
      for (const page of ['timer', 'week', 'month', 'year', 'answers'])
        cpSync(
          join(PORT, `crates/render/results/${page}.json`),
          join(inputs, `${page}.json`),
        )
      cpSync(await seededDatabase(), join(inputs, 'api.db'))
      cpSync(prepared.paths.database, join(inputs, 'M.db'))
      cpSync(prepared.paths.users, join(inputs, 'M.users.json'))
      command(
        'bun',
        [
          'harness/api-recording.ts',
          prepared.recording,
          join(inputs, 'api-slice.json'),
          '--api-only',
          'getAppSession',
          'getRunningTimer',
          'listEntries',
          'getReport',
        ],
        'api-slice.log',
      )
      const slice = json(join(inputs, 'api-slice.json'))
      const requests = [...slice.actions.return, ...slice.actions['reports-week']].filter(
        (request: { path: string }) => request.path.startsWith('/api/v1/'),
      )
      slice.actions = {
        'gate-slice': [
          ...new Map(
            requests.map((request: unknown) => [JSON.stringify(request), request]),
          ).values(),
        ],
      }
      if (!slice.actions['gate-slice'].length)
        throw new Error('Recording has no return/week API slice')
      for (const endpoint of ['/session', '/timer', '/entries', '/report'])
        if (
          !slice.actions['gate-slice'].some((request: { path: string }) =>
            request.path.split('?')[0].endsWith(endpoint),
          )
        )
          throw new Error(`Recording is missing hot call ${endpoint}`)
      writeFileSync(join(inputs, 'api-slice.json'), JSON.stringify(slice, null, 2))
      writeFileSync(
        join(inputs, 'hashes.json'),
        JSON.stringify(
          seal(inputs, [
            'api.db',
            'M.db',
            'M.users.json',
            'api-slice.json',
            'timer.json',
            'week.json',
            'month.json',
            'year.json',
            'answers.json',
          ]),
          null,
          2,
        ),
      )
      command(
        'docker',
        ['build', '-f', 'harness/gate/Dockerfile', '-t', 'snowtime-perf-gate:runtime', '.'],
        'runtime-build.log',
      )
      snapshot('current')
    } else if (values['build-current']) {
      command('bun', ['--bun', 'run', 'build'], 'app-build.log', {}, ROOT)
      command('bun', ['adapter/build.ts'], 'bundle-build.log')
      const fork = forkContext()
      command(
        'docker',
        [
          'build',
          ...fork,
          '-f',
          'deploy/Dockerfile',
          '--build-arg',
          'CARGO_FEATURES=bench',
          '-t',
          'snowtime-native:bench',
          '.',
        ],
        'server-build.log',
      )
      command(
        'docker',
        [
          'build',
          ...fork,
          '-f',
          'adapter/bench/Dockerfile',
          '-t',
          'snowtime-kit-render:bench',
          '.',
        ],
        'render-build.log',
      )
      snapshot('current')
    } else {
      command('git', ['rev-parse', 'HEAD'], 'runner-commit.txt')
      command('git', ['diff', '--submodule=diff', 'HEAD'], 'runner-diff.patch')
      const current = json(
        join(gate, values.identical ? 'baseline/artifact.json' : 'current/artifact.json'),
      )
      if (
        !values.identical &&
        (current.commit !== command('git', ['rev-parse', 'HEAD'], 'current-commit.txt').trim() ||
          current.diff !==
            command('git', ['diff', '--submodule=diff', 'HEAD'], 'current-diff.patch'))
      )
        throw new Error(
          'Current checkout changed since artifact capture; run --build-current first',
        )
      cpSync(join(gate, 'baseline/artifact.json'), join(out, 'baseline-artifact.json'))
      cpSync(
        join(gate, values.identical ? 'baseline/artifact.json' : 'current/artifact.json'),
        join(out, 'current-artifact.json'),
      )
      writeFileSync(
        join(out, 'comparison-mode.json'),
        JSON.stringify({ identical: values.identical }),
      )
      cpSync(join(gate, 'inputs/hashes.json'), join(out, 'input-hashes.json'))
      for (const [file, h] of Object.entries(json(join(gate, 'inputs/hashes.json'))))
        if (hash(join(gate, 'inputs', file)) !== (h as { sha256: string }).sha256)
          throw new Error(`Changed gate input ${file}`)
      command('uname', ['-a'], 'machine.txt')
      for (const [program, args, log] of machineCommands()) command(program, args, log)
      command('docker', ['image', 'inspect', 'snowtime-perf-gate:runtime'], 'runtime-image.json')
      const running = command('docker', ['ps', '--format', '{{.Names}}'], 'running-containers.txt')
        .trim()
        .split('\n')
        .filter((name) => /^snowtime-bench-(app|caddy|sampler|litestream)-1$/.test(name))
      if (running.length) command('docker', ['stop', ...running], 'stop-bench-stack.log')
      // The app's packages for Linux, over the host's, which may be macOS builds. They're
      // installed before the measured container starts, since its script imports them.
      // Without scripts: the app's prepare hook would install git hooks in the host's checkout.
      const modules = ['-v', `${PORT}:/repo`, '-v', `${MODULES_VOLUME}:/repo/app/node_modules`]
      command(
        'docker',
        [
          'run',
          '--rm',
          ...modules,
          '--workdir=/repo/app',
          '--entrypoint=bun',
          'snowtime-perf-gate:runtime',
          'install',
          '--frozen-lockfile',
          '--ignore-scripts',
        ],
        'modules-install.log',
      )
      const started = Date.now()
      console.log('[gate] Starting retained baseline/current A/B/A/B; no builds during this run')
      commands.push({
        program: 'docker',
        limits: { cpus: 1, cpuset: profile.cpuset, memory: '2g', swap: 'disabled' },
        profile: values.profile,
        renders,
        repeats: values.repeats,
        rate: values.rate,
        started: new Date().toISOString(),
      })
      writeFileSync(join(out, 'commands.json'), JSON.stringify(commands, null, 2))
      const result = spawnSync(
        'docker',
        [
          'run',
          '--rm',
          '--cpus=1',
          `--cpuset-cpus=${profile.cpuset}`,
          '--memory=2g',
          '--memory-swap=2g',
          ...modules,
          'snowtime-perf-gate:runtime',
          `--out=/repo/${relative(PORT, out)}`,
          `--profile=${values.profile}`,
          `--renders=${renders}`,
          `--repeats=${values.repeats}`,
          `--rate=${values.rate}`,
          ...(values.identical ? ['--identical'] : []),
        ],
        { cwd: PORT, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 },
      )
      writeFileSync(join(out, 'gate.log'), result.stdout + result.stderr)
      writeFileSync(join(out, 'elapsed-seconds.txt'), `${(Date.now() - started) / 1000}\n`)
      // Bun exits 1 on an uncaught error too, so only a finished comparison makes 1 a regression.
      const compared =
        result.status === 0 ||
        (result.status === 1 && existsSync(join(out, 'comparison.json')))
      if (!compared && !existsSync(join(out, 'excluded.txt')))
        writeFileSync(
          join(out, 'excluded.txt'),
          `Gate container exited ${result.status}; see gate.log\n`,
        )
      console.log(result.stdout)
      if (result.stderr) console.error(result.stderr)
      process.exitCode = compared ? result.status! : 2
    }
  } finally {
    release()
  }
  console.log(`Evidence: ${out}`)
}
try {
  await main()
} catch (error) {
  writeFileSync(
    join(out, 'excluded.txt'),
    `${error instanceof Error ? error.stack : String(error)}\n`,
  )
  console.error(error)
  console.error(`Excluded evidence: ${out}`)
  process.exitCode = 2
}
