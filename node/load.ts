import { execSync } from 'node:child_process'
import { existsSync, readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { join } from 'node:path'
import process from 'node:process'

const { platform, arch } = process

function isMusl(): boolean {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    }
    catch {
      return true
    }
  }
  else {
    const report = process.report.getReport() as any
    const { glibcVersionRuntime } = report.header
    return !glibcVersionRuntime
  }
}

function getLoadPath(): string {
  const baseDir = join(__dirname, '..')
  let localPath: string | null = null
  let packageName: string | null = null

  switch (platform) {
    case 'android':
      switch (arch) {
        case 'arm64':
          localPath = join(baseDir, 'project-detector.android-arm64.node')
          packageName = '@arkts/project-detector-android-arm64'
          break
        default:
          throw new Error(`Unsupported architecture on Android: ${arch}`)
      }
      break

    case 'win32':
      switch (arch) {
        case 'x64':
          localPath = join(baseDir, 'project-detector.win32-x64-msvc.node')
          packageName = '@arkts/project-detector-win32-x64-msvc'
          break
        case 'ia32':
          localPath = join(baseDir, 'project-detector.win32-ia32-msvc.node')
          packageName = '@arkts/project-detector-win32-ia32-msvc'
          break
        case 'arm64':
          localPath = join(baseDir, 'project-detector.win32-arm64-msvc.node')
          packageName = '@arkts/project-detector-win32-arm64-msvc'
          break
        default:
          throw new Error(`Unsupported architecture on Windows: ${arch}`)
      }
      break

    case 'darwin': {
      // Try universal binary first
      const universalPath = join(baseDir, 'project-detector.darwin-universal.node')
      if (existsSync(universalPath)) {
        return universalPath
      }

      // Try loading from universal package
      try {
        return require.resolve('@arkts/project-detector-darwin-universal')
      }
      catch {}

      switch (arch) {
        case 'x64':
          localPath = join(baseDir, 'project-detector.darwin-x64.node')
          packageName = '@arkts/project-detector-darwin-x64'
          break
        case 'arm64':
          localPath = join(baseDir, 'project-detector.darwin-arm64.node')
          packageName = '@arkts/project-detector-darwin-arm64'
          break
        default:
          throw new Error(`Unsupported architecture on macOS: ${arch}`)
      }
      break
    }

    case 'freebsd':
      if (arch !== 'x64') {
        throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
      }
      localPath = join(baseDir, 'project-detector.freebsd-x64.node')
      packageName = '@arkts/project-detector-freebsd-x64'
      break

    case 'linux':
      switch (arch) {
        case 'x64':
          if (isMusl()) {
            localPath = join(baseDir, 'project-detector.linux-x64-musl.node')
            packageName = '@arkts/project-detector-linux-x64-musl'
          }
          else {
            localPath = join(baseDir, 'project-detector.linux-x64-gnu.node')
            packageName = '@arkts/project-detector-linux-x64-gnu'
          }
          break
        case 'arm64':
          if (isMusl()) {
            localPath = join(baseDir, 'project-detector.linux-arm64-musl.node')
            packageName = '@arkts/project-detector-linux-arm64-musl'
          }
          else {
            localPath = join(baseDir, 'project-detector.linux-arm64-gnu.node')
            packageName = '@arkts/project-detector-linux-arm64-gnu'
          }
          break
        case 'arm':
          localPath = join(baseDir, 'project-detector.linux-arm-gnueabihf.node')
          packageName = '@arkts/project-detector-linux-arm-gnueabihf'
          break
        default:
          throw new Error(`Unsupported architecture on Linux: ${arch}`)
      }
      break

    default:
      throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
  }

  // Try local file first
  if (localPath && existsSync(localPath)) {
    return localPath
  }

  // Try package
  if (packageName) {
    try {
      return require.resolve(packageName)
    }
    catch {
      throw new Error(
        `Failed to load native binding for ${platform}-${arch}. `
        + `Local file not found: ${localPath}, `
        + `Package not found: ${packageName}`,
      )
    }
  }

  throw new Error(`Failed to determine load path for ${platform}-${arch}`)
}

export function loadNativeBinding<T>(): T {
  const loadPath = getLoadPath()
  const require = createRequire(import.meta.url)
  return require(loadPath)
}
