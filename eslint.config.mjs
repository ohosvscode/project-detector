import antfu from '@antfu/eslint-config'

export default antfu({
  ignores: [
    'mock/**',
    'index.d.ts',
    '*.js',
  ],
})
