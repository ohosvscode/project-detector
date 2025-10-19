import path from 'node:path'
import { dts } from 'rolldown-plugin-dts'
import { defineConfig } from 'tsdown/config'
import { createDtsResolvePlugin } from './scripts/resolve-plugin'

export default defineConfig({
  entry: './src/node/index.ts',
  external: [/\.node$/, path.resolve('index.js')],
  dts: false,
  plugins: [
    dts()
      .filter(plugin => plugin.name !== 'rolldown-plugin-dts:resolver')
      .concat(
        createDtsResolvePlugin({
          tsconfig: path.resolve('tsconfig.json'),
          resolve: false,
        }),
      ),
  ],
  outputOptions: {
    advancedChunks: {
      groups: [
        // handle .d.ts files
        { test: /module-build-profile.*\.d\.[cm]?ts$/, name: 'module-build-profile.d' },
        { test: /project-build-profile.*\.d\.[cm]?ts$/, name: 'project-build-profile.d' },
      ],
    },
  },
})
