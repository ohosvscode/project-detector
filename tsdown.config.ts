import path from 'node:path'
import { defineConfig } from 'tsdown'

export default defineConfig({
  entry: './src/node/index.ts',
  external: [/\.node$/, path.resolve('index.js')],
  platform: 'node', // 明确指定平台为 node
})
