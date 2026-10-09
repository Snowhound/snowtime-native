// The machine a measurement ran on, for its evidence folder: lscpu and free on Linux, sysctl
// on macOS, where the measured servers run in Docker Desktop's Linux VM, whose size
// `docker info` records.
export function machineCommands(): [string, string[], string][] {
  const docker: [string, string[], string] = ['docker', ['info'], 'docker-info.txt']
  if (process.platform === 'darwin')
    return [
      ['sysctl', ['machdep.cpu.brand_string', 'hw.ncpu', 'hw.memsize'], 'cpu.txt'],
      docker,
    ]
  return [['lscpu', [], 'cpu.txt'], ['free', ['-b'], 'memory.txt'], docker]
}
