import { spawnSync } from 'node:child_process'
import fs from 'node:fs'
import process from 'node:process'

spawnSync('napi version', process.argv.slice(2), { stdio: 'inherit' })
const version = JSON.parse(fs.readFileSync('package.json', 'utf-8')).version
spawnSync(`pnpm build && git add --all && git commit -m "${version}"`, { stdio: 'inherit' })
