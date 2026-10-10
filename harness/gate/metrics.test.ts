import { expect, test } from 'bun:test'
import { compareMetric } from './metrics'

test('CPU has a five percent floor even when the baseline rounds agree', () => {
  expect(compareMetric([100, 100], [105, 105], 'api/timer/cpu_ms').flagged).toBe(false)
  expect(compareMetric([100, 100], [106, 106], 'api/timer/cpu_ms').flagged).toBe(true)
})
test('the two baseline rounds define a wider observed noise band', () => {
  expect(compareMetric([90, 110], [120, 120], 'api/timer/cpu_ms').flagged).toBe(false)
  expect(compareMetric([90, 110], [121, 121], 'api/timer/cpu_ms').flagged).toBe(true)
})
test('sizes have a five percent floor and quantized zero CPU is invalid', () => {
  expect(compareMetric([100, 100], [101, 101], 'static/binary_bytes').flagged).toBe(false)
  expect(compareMetric([100, 100], [90, 90], 'static/binary_bytes').flagged).toBe(false)
  expect(() => compareMetric([0, 0], [1, 1], 'api/timer/cpu_ms')).toThrow('resolution')
})

test('latency and Server-Timing floors apply in their native units', () => {
  for (const [metric, floor] of [
    ['render/week/p95_ms', 10],
    ['render/week/p50_ms', 5],
    ['api/session/session_ms', 5],
    ['api/session/db_ms', 5],
  ] as const) {
    expect(compareMetric([100, 100], [100 + floor, 100 + floor], metric).flagged).toBe(false)
    expect(compareMetric([100, 100], [101 + floor, 101 + floor], metric).flagged).toBe(true)
  }
})
test('RSS uses the larger of five percent and four MiB', () => {
  for (const metric of ['render/week/peak_rss_mb', 'load/peak_rss_mib']) {
    expect(compareMetric([40, 40], [44, 44], metric).flagged).toBe(false)
    expect(compareMetric([40, 40], [45, 45], metric).flagged).toBe(true)
    expect(compareMetric([200, 200], [210, 210], metric).flagged).toBe(false)
    expect(compareMetric([200, 200], [211, 211], metric).flagged).toBe(true)
  }
})

test('Server-Timing tolerates one resolution step above zero', () => {
  for (const metric of ['api/session/session_ms', 'api/session/db_ms']) {
    expect(compareMetric([0, 0], [0.1, 0.1], metric).flagged).toBe(false)
    expect(compareMetric([0, 0], [0.2, 0.2], metric).flagged).toBe(true)
    expect(compareMetric([1, 1], [1.1, 1.1], metric).flagged).toBe(false)
    expect(compareMetric([10, 10], [10.5, 10.5], metric).flagged).toBe(false)
    expect(compareMetric([10, 10], [10.6, 10.6], metric).flagged).toBe(true)
  }
})

test('a profile widens the floor for render CPU and p50 only', () => {
  for (const metric of ['render/week/cpu_ms', 'render/week/p50_ms']) {
    expect(compareMetric([100, 100], [110, 110], metric, { render: 0.1 }).flagged).toBe(false)
    expect(compareMetric([100, 100], [111, 111], metric, { render: 0.1 }).flagged).toBe(true)
  }
  expect(compareMetric([100, 100], [106, 106], 'api/timer/cpu_ms', { render: 0.1 }).flagged).toBe(
    true,
  )
})
test('CPU counted in clock ticks has a floor of its resolution', () => {
  // One 10 ms tick over 1,000 calls.
  const floors = { resolution: 0.01 }
  expect(compareMetric([0.04, 0.04], [0.05, 0.05], 'api/timer/cpu_ms', floors).flagged).toBe(false)
  expect(compareMetric([0.04, 0.04], [0.051, 0.051], 'api/timer/cpu_ms', floors).flagged).toBe(true)
})
test('the lowest-round statistic ignores one slowed round on either side', () => {
  expect(compareMetric([0.22, 0.23], [0.33, 0.23], 'api/timer/cpu_ms', {}, 'lowest').flagged).toBe(
    false,
  )
  expect(compareMetric([0.22, 0.23], [0.33, 0.23], 'api/timer/cpu_ms').flagged).toBe(true)
  expect(compareMetric([100, 101], [120, 115], 'render/week/cpu_ms', {}, 'lowest').flagged).toBe(
    true,
  )
})
