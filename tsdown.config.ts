import path from 'node:path'
import { defineConfig } from 'tsdown'

export default defineConfig({
  entry: './node/index.ts',
  external: [/\.node$/, path.resolve('index.js')],
})
