import path from 'node:path'
import { dts } from 'rolldown-plugin-dts'
import { defineConfig } from 'tsdown/config'
import { dependencies } from './package.json'
import { IndexExternalResolvePlugin } from './scripts/index-external-plugin'
import { createDtsResolvePlugin } from './scripts/resolve-plugin'

export default defineConfig([
  {
    entry: './src/node/index.ts',
    external: [/\.node$/, path.resolve('index.js')],
    dts: false,
    format: 'esm',
    plugins: [
      IndexExternalResolvePlugin(path.resolve('index'), '../index.js'),
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
  },
  {
    entry: {
      bundled: './src/node/index.ts',
    },
    external: [/\.node$/, path.resolve('index.js')],
    noExternal: Object.keys(dependencies),
    dts: false,
    format: 'esm',
    plugins: [
      IndexExternalResolvePlugin(path.resolve('index'), '../index.js'),
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
  },
])
