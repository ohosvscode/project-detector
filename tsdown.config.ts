import path from 'node:path'
import { dts } from 'rolldown-plugin-dts'
import { defineConfig } from 'tsdown/config'
import { dependencies } from './package.json'
import { IndexExternalResolvePlugin } from './scripts/index-external-plugin'
import { createDtsResolvePlugin } from './scripts/resolve-plugin'

function createBasePlugins() {
  return [
    IndexExternalResolvePlugin(path.resolve('index'), '../index.js'),
  ]
}

function createDtsPlugins() {
  return dts()
    .filter(plugin => plugin.name !== 'rolldown-plugin-dts:resolver')
    .concat(
      createDtsResolvePlugin({
        tsconfig: path.resolve('tsconfig.json'),
        resolve: false,
      }),
    )
}

const esmOutputOptions = {
  advancedChunks: {
    groups: [
      // handle .d.ts files
      { test: /module-build-profile.*\.d\.[cm]?ts$/, name: 'module-build-profile.d' },
      { test: /project-build-profile.*\.d\.[cm]?ts$/, name: 'project-build-profile.d' },
    ],
  },
}

export default defineConfig([
  {
    entry: './src/node/index.ts',
    external: [/\.node$/, path.resolve('index.js')],
    dts: false,
    format: 'esm',
    outExtensions: () => ({ js: '.js' }),
    platform: 'node',
    shims: true,
    plugins: [...createBasePlugins(), ...createDtsPlugins()],
    outputOptions: esmOutputOptions,
  },
  {
    entry: './src/node/index.ts',
    external: [/\.node$/, path.resolve('index.js')],
    dts: false,
    format: 'cjs',
    outExtensions: () => ({ js: '.cjs' }),
    plugins: createBasePlugins(),
  },
  {
    entry: {
      bundled: './src/node/index.ts',
    },
    external: [/\.node$/, path.resolve('index.js')],
    noExternal: Object.keys(dependencies),
    dts: false,
    format: 'esm',
    outExtensions: () => ({ js: '.js' }),
    platform: 'node',
    shims: true,
    plugins: [...createBasePlugins(), ...createDtsPlugins()],
    outputOptions: esmOutputOptions,
  },
  {
    entry: {
      bundled: './src/node/index.ts',
    },
    external: [/\.node$/, path.resolve('index.js')],
    noExternal: Object.keys(dependencies),
    dts: false,
    format: 'cjs',
    outExtensions: () => ({ js: '.cjs' }),
    plugins: createBasePlugins(),
  },
])
