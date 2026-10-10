// Named settings for the gate. A run compares baseline and current on one machine in one
// session, so a profile sets only the core, the render count, and the render floor; any
// machine can run any profile, and each platform has a default.
export type Profile = {
  // The CPU the gate container is pinned to.
  cpuset: string
  renders: number
  // The relative floor for render CPU and p50 latency.
  renderFloor: number
}

export const PROFILES: Record<string, Profile> = {
  ryzen: { cpuset: '0', renders: 25, renderFloor: 0.05 },
  // Docker Desktop's CPUs are macOS threads that move between performance and efficiency
  // cores, so render CPU and p50 vary 6-10% between identical runs. The VM's last CPU
  // varied less than CPU 0, which also carries more of the VM's own work.
  mac: { cpuset: '9', renders: 35, renderFloor: 0.1 },
}

export function defaultProfile() {
  return process.platform === 'darwin' ? 'mac' : 'ryzen'
}
