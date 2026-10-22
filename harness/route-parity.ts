// Starts the native server on the benchmark database and checks it serves every route the
// app's routers define, with the porting kit's route parity check (route-parity.json).
//   bun harness/route-parity.ts <binary> [kit checkout, by default harness/kit.sh's]
import { spawnSync } from 'node:child_process'
import { join, resolve } from 'node:path'
import { seededDatabase } from '../app/perf/lib/database'
import { startNative } from './native'

const [binary, given] = process.argv.slice(2)
if (!binary) throw new Error('usage: bun harness/route-parity.ts <binary> [kit checkout]')
const kit =
  given ?? spawnSync(join(import.meta.dir, 'kit.sh'), { encoding: 'utf8' }).stdout.trim()
const server = await startNative(resolve(binary), await seededDatabase())
let status = 1
try {
  const root = resolve(import.meta.dir, '..')
  status =
    spawnSync(
      'bun',
      [join(resolve(kit), 'tools/route-parity/cli.ts'), 'route-parity.json', '--url', server.url],
      { cwd: root, stdio: 'inherit' },
    ).status ?? 1
} finally {
  await server.stop()
}
process.exit(status)
