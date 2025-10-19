import type { Plugin, ResolvedId } from 'rolldown'
import type { resolveOptions } from 'rolldown-plugin-dts'
import path from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'
import { createResolver } from 'dts-resolver'
import { RE_CSS, RE_DTS, RE_JS, RE_JSON, RE_NODE_MODULES, RE_TS, RE_VUE } from 'rolldown-plugin-dts'
import { ResolverFactory } from 'rolldown/experimental'
import { globalLogger } from 'tsdown'

function filename_to_dts(id: string): string {
  return id
    .replace(RE_VUE, '.vue.ts')
    .replace(RE_TS, '.d.$1ts')
    .replace(RE_JS, '.d.$1ts')
    .replace(RE_JSON, '.d.ts')
}

function isSourceFile(id: string) {
  return RE_TS.test(id) || RE_VUE.test(id) || RE_JSON.test(id)
}

const exclude: Record<string, string> = {
  [path.resolve(fileURLToPath(import.meta.url), '..', '..', 'index.d.ts')]: '../index',
  [path.resolve(fileURLToPath(import.meta.url), '..', '..', 'index.js')]: '../index',
}

for (const [filePath, replacement] of Object.entries(exclude)) {
  globalLogger.info(`exclude emit dts files: ${path.relative(process.cwd(), filePath)} -> ${replacement}`)
}

export function createDtsResolvePlugin({
  tsconfig,
  resolve,
}: Pick<ReturnType<typeof resolveOptions>, 'tsconfig' | 'resolve'>): Plugin {
  const baseDtsResolver = createResolver({
    tsconfig,
    resolveNodeModules: !!resolve,
    ResolverFactory,
  })

  return {
    name: 'rolldown-plugin-dts:resolver',

    resolveId: {
      order: 'pre',
      async handler(id, importer, options) {
        // Guard: Only operate on imports inside .d.ts files
        if (!importer || !RE_DTS.test(importer)) {
          return
        }

        const external = { id, external: true, moduleSideEffects: false }

        // Guard: Externalize non-code imports
        if (RE_CSS.test(id)) {
          return external
        }

        // Get Rolldown's resolution first for fallback and policy checks
        const rolldownResolution = await this.resolve(id, importer, options)
        const dtsResolution = resolveDtsPath(id, importer, rolldownResolution)

        // If resolution failed, error or externalize
        if (!dtsResolution) {
          const isFileImport = isFilePath(id)

          // Auto-export unresolvable packages
          return isFileImport ? null : external
        }

        // Externalize excluded files
        const normalizedResolution = path.normalize(dtsResolution)
        if (exclude[normalizedResolution] !== undefined) {
          return { id: exclude[normalizedResolution], external: true, moduleSideEffects: false }
        }

        // Externalize non-bundled node_modules dependencies
        if (
          // request resolved to inside node_modules
          RE_NODE_MODULES.test(dtsResolution)
          // User doesn't want to bundle this module
          && !shouldBundleNodeModule(id)
          // The importer is not in node_modules, or if it is, the module is marked as external by Rolldown
          && (!RE_NODE_MODULES.test(importer) || rolldownResolution?.external)
        ) {
          return external
        }

        // The path is either a declaration file or a source file that needs redirection.
        if (RE_DTS.test(dtsResolution)) {
          // It's already a .d.ts file, we're done
          return dtsResolution
        }

        if (isSourceFile(dtsResolution)) {
          // It's a .ts/.vue source file, so we load it to ensure its .d.ts is generated,
          // then redirect the import to the future .d.ts path
          await this.load({ id: dtsResolution })
          return filename_to_dts(dtsResolution)
        }
      },
    },
  }

  function shouldBundleNodeModule(id: string) {
    if (typeof resolve === 'boolean')
      return resolve
    return resolve.some(pattern =>
      typeof pattern === 'string' ? id === pattern : pattern.test(id),
    )
  }

  function resolveDtsPath(
    id: string,
    importer: string,
    rolldownResolution: ResolvedId | null,
  ): string | null {
    let dtsPath = baseDtsResolver(id, importer)
    if (dtsPath) {
      dtsPath = path.normalize(dtsPath)
    }

    if (!dtsPath || !isSourceFile(dtsPath)) {
      if (
        rolldownResolution
        && isFilePath(rolldownResolution.id)
        && isSourceFile(rolldownResolution.id)
        && !rolldownResolution.external
      ) {
        return rolldownResolution.id
      }
      return null
    }

    return dtsPath
  }
}

function isFilePath(id: string) {
  return id.startsWith('.') || path.isAbsolute(id)
}
