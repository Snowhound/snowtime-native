export type Floors = {
  // The relative floor for render CPU and p50 latency (the profile's renderFloor).
  render?: number
  // The metric's measurement resolution, such as one /proc clock tick spread over the calls.
  resolution?: number
}

// How each side's two rounds become one value: their mean, or the lower round, which
// discounts a round that host interference slowed.
export type Statistic = 'mean' | 'lowest'

export function compareMetric(
  baseline: number[],
  current: number[],
  metric: string,
  floors: Floors = {},
  statistic: Statistic = 'mean',
) {
  if (
    baseline.length !== 2 ||
    current.length !== 2 ||
    [...baseline, ...current].some((n) => !Number.isFinite(n) || n < 0)
  )
    throw new Error('Each gate metric needs two finite, nonnegative observations')
  function combine(rounds: number[]) {
    return statistic === 'lowest' ? Math.min(...rounds) : (rounds[0] + rounds[1]) / 2
  }
  const a = combine(baseline)
  const b = combine(current)
  if (metric.endsWith('/cpu_ms') && a === 0)
    throw new Error('CPU is below /proc resolution; increase API repeats')
  const rss = metric.endsWith('/peak_rss_mb') || metric.endsWith('/peak_rss_mib')
  const serverTiming = metric.endsWith('/session_ms') || metric.endsWith('/db_ms')
  const render =
    metric.startsWith('render/') && (metric.endsWith('/cpu_ms') || metric.endsWith('/p50_ms'))
  const relative = metric.endsWith('/p95_ms') ? 0.1 : render ? (floors.render ?? 0.05) : 0.05
  const floor = Math.max(
    a * relative,
    rss ? 4 : 0,
    serverTiming ? 0.1 : 0,
    floors.resolution ?? 0,
  )
  const band = Math.max(Math.abs(baseline[0] - baseline[1]), floor)
  return {
    baseline: a,
    current: b,
    delta: b - a,
    deltaPercent: a ? (b / a - 1) * 100 : null,
    band,
    flagged: b - a > band + Number.EPSILON * Math.max(1, a),
  }
}
