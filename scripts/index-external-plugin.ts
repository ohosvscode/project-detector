import path from 'node:path'

export function IndexExternalResolvePlugin(targetPath: string, replacePath: string): import('rolldown').Plugin {
  const INDEX_ID = crypto.randomUUID()
  const ROOT_INDEX_ID = `\0${INDEX_ID}`

  return {
    name: `index-resolve`,
    resolveId: {
      order: 'post',
      handler(source, importer) {
        if (path.isAbsolute(source))
          return null
        if (!importer)
          return null

        const resolved = path.resolve(path.dirname(importer), source)
        if (resolved === targetPath)
          return { id: ROOT_INDEX_ID, external: true }
        return null
      },
    },

    renderChunk: {
      order: 'post',
      handler(code, chunkInfo) {
        if (chunkInfo.fileName.endsWith('.js') || chunkInfo.fileName.endsWith('.mjs') || chunkInfo.fileName.endsWith('.cjs'))
          return code.replaceAll(ROOT_INDEX_ID, replacePath).replaceAll(INDEX_ID, replacePath)
      },
    },
  }
}
