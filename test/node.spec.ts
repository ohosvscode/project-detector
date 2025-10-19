import type { FSWatcher } from 'chokidar'
import fs from 'node:fs'
import path from 'node:path'
import { afterAll, describe, expect } from 'vitest'
import { Uri } from '../index'
import { Module, Product, Project, ProjectDetector, Resource, Watcher } from '../src/node'

describe.sequential('projectDetector', (it) => {
  const mockPath = path.resolve(__dirname, '..', 'mock')

  let projectDetector: ProjectDetector
  let watcher: FSWatcher

  it.sequential('projectDetector.create', async () => {
    projectDetector = ProjectDetector.create(Uri.file(mockPath).toString())
    expect(projectDetector.getWorkspaceFolder().fsPath).toBe(mockPath)
    watcher = await Watcher.fromChokidar(projectDetector)
    watcher.on('all', (event, path) => console.warn(`[FILE EVENT] ${event} ${path}`))
  })

  afterAll(() => {
    watcher.close()
  })

  let harmonyProject1: Project

  it.sequential('project.findAll', async () => {
    const projects = Project.findAll(projectDetector)
    expect(projects().length).toBe(2)
    harmonyProject1 = projects().find(project => project.getUri().toString().includes('harmony-project-1'))!
    const harmonyProject2 = projects().find(project => project.getUri().toString().includes('harmony-project-2'))!
    expect(harmonyProject1).toBeDefined()
    expect(harmonyProject2).toBeDefined()

    // test the file event listener:
    // when the build-profile.json5 file is deleted, the project should be removed
    fs.renameSync(harmonyProject1.getBuildProfileUri().fsPath, path.resolve(harmonyProject1.getUri().fsPath, 'build-profile.json5.bak'))
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(projects().length).toBe(1)
    expect(projects().find(project => project.getUri().toString().includes('harmony-project-1'))).toBeUndefined()

    // when the build-profile.json5 file is created, the project should be added
    fs.renameSync(path.resolve(harmonyProject1.getUri().fsPath, 'build-profile.json5.bak'), harmonyProject1.getBuildProfileUri().fsPath)
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(projects().length).toBe(2)
  })

  let harmonyProject1Module: Module

  it.sequential('module.findAll', async () => {
    const modules = Module.findAll(harmonyProject1)
    expect(modules().length).toBe(1)
    const bakFilePath = path.resolve(modules()[0].getUri().fsPath, 'build-profile.json5.bak')
    const buildProfileFilePath = modules()[0].getBuildProfileUri().fsPath

    // when the build-profile.json5 file is deleted, the module should be removed
    fs.renameSync(buildProfileFilePath, bakFilePath)
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(modules().length).toBe(0)

    // when the build-profile.json5 file is created, the module should be added
    fs.renameSync(bakFilePath, buildProfileFilePath)
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(modules().length).toBe(1)

    harmonyProject1Module = modules()[0]
  })

  let harmonyProject1MainProduct: Product

  it.sequential('product.findAll', async () => {
    const products = Product.findAll(harmonyProject1Module)
    expect(products().length).toBeGreaterThanOrEqual(2)
    harmonyProject1MainProduct = products()[0]
    const parsedBuildProfile = harmonyProject1Module.getParsedBuildProfile()
    const currentTargetConfig = harmonyProject1MainProduct.getCurrentTargetConfig()
    expect(currentTargetConfig).toBeDefined()
    const filteredTargets = parsedBuildProfile.targets?.filter(target => target.name !== harmonyProject1MainProduct.getName()) ?? []
    expect(filteredTargets.length).toBe(1)
    fs.writeFileSync(
      harmonyProject1Module.getBuildProfileUri().fsPath,
      JSON.stringify({
        ...parsedBuildProfile,
        targets: filteredTargets,
      }, null, 2),
    )
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(products().length).toBe(1)
    fs.writeFileSync(
      harmonyProject1Module.getBuildProfileUri().fsPath,
      JSON.stringify({
        ...parsedBuildProfile,
        targets: parsedBuildProfile.targets,
      }, null, 2),
    )
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(products().length).toBe(2)
  })

  it.sequential('resource.findAll', async () => {
    const resources = Resource.findAll(harmonyProject1MainProduct)
    console.log(resources().map(resource => resource.getUri().toString()))
  })
})
