import path from 'node:path'
import { defineConfig } from 'tsdown'

export default defineConfig({
  entry: './src/node/index.ts',
  external: [/\.node$/, path.resolve('index.js')],
})
