/* eslint-disable no-console */
import { Bench } from 'tinybench'
import { ProjectDetector } from '../index.js'

const bench = new Bench()

bench.add('Native ProjectDetector.create', () => {
  ProjectDetector.create('')
})

bench.run()
  .then(() => console.table(bench.table()))
  .catch(console.error)
