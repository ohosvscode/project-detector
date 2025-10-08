import { defineConfig } from 'tsdown/config'

export default defineConfig({
  entry: 'node/index.ts',
  outDir: 'out',
  format: ['esm', 'cjs'],
})
