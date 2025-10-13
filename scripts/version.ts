import { execSync } from 'node:child_process'
import fs from 'node:fs'
import { NapiCli } from '@napi-rs/cli'

new NapiCli().version({}).then(() => {
  const version = JSON.parse(fs.readFileSync('package.json', 'utf-8')).version
  execSync(`pnpm build && git add --all && git commit -m "${version}"`)
}).catch(console.error)
