import child_process from 'node:child_process'
import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    environment: 'node',
  },
  plugins: [
    {
      name: 'rebuild',
      watchChange(id, { event }) {
        try {
          if (id.includes('target') || id.includes('node_modules') || id.includes('dist') || id.includes('coverage'))
            return
          console.log(`${event}`, id)
          child_process.execSync('pnpm build')
        }
        catch (error) {
          console.error(error)
          console.error(`Failed to rebuild.`)
        }
      },
    },
  ],
})
