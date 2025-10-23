import antfu from '@antfu/eslint-config'

export default antfu({
  ignores: [
    'mock/**',
    'index.d.ts',
    '*.js',
    '*.cjs',
    '*.mjs',
  ],

  rules: {
    'ts/no-namespace': 'off',
    'ts/method-signature-style': ['error', 'method'],
  },
})
