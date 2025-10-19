import type { FSWatcher } from 'chokidar'
import fs from 'node:fs'
import path from 'node:path'
import { afterAll, describe, expect } from 'vitest'
import { Uri } from '../index'
import { ElementDirectory, ElementJsonFile, ElementJsonFileReference, Module, Product, Project, ProjectDetector, Resource, ResourceDirectory, Watcher } from '../src/node'

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
    const mainProduct = products()[0]
    const parsedBuildProfile = harmonyProject1Module.getParsedBuildProfile()
    const currentTargetConfig = mainProduct.getCurrentTargetConfig()
    expect(currentTargetConfig).toBeDefined()
    const filteredTargets = parsedBuildProfile.targets?.filter(target => target.name !== mainProduct.getName()) ?? []
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
    harmonyProject1MainProduct = products().find(product => product.getName() === 'default')!
  })

  let harmonyProject1MainResource: Resource

  it.sequential('resource.findAll', async () => {
    const resources = Resource.findAll(harmonyProject1MainProduct)
    expect(resources().length).toBe(1)
    const resource = resources()[0]
    const targetName = resource.getProduct().getName()
    const buildProfileUri = resource.getProduct().getModule().getBuildProfileUri()
    const parsedBuildProfile = resource.getProduct().getModule().getParsedBuildProfile()
    fs.writeFileSync(
      buildProfileUri.fsPath,
      JSON.stringify({
        ...parsedBuildProfile,
        targets: (parsedBuildProfile.targets ?? []).filter(target => target.name !== targetName),
      }, null, 2),
    )
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(resources().length).toBe(0)
    fs.writeFileSync(
      buildProfileUri.fsPath,
      JSON.stringify({
        ...parsedBuildProfile,
        targets: [{ name: targetName }, ...(parsedBuildProfile.targets ?? []).filter(target => target.name !== targetName)],
      }, null, 2),
    )
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(resources().length).toBe(1)
    harmonyProject1MainResource = resources()[0]
  })

  let baseResourceDirectory: ResourceDirectory

  it.sequential('resourceDirectory.findAll', async () => {
    const resourceDirectories = ResourceDirectory.findAll(harmonyProject1MainResource)
    expect(resourceDirectories().length).toBe(3)
    const resourceDirectory = resourceDirectories().find(resourceDirectory => resourceDirectory.getQualifiers() === 'base')!
    expect(resourceDirectory).toBeDefined()
    const uri = resourceDirectory.getUri()
    fs.renameSync(uri.fsPath, path.resolve(Uri.dirname(uri).fsPath, 'base.bak'))
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(resourceDirectories().length).toBe(2)
    expect(resourceDirectories().find(resourceDirectory => resourceDirectory.getQualifiers() === 'base')).toBeUndefined()
    fs.renameSync(path.resolve(Uri.dirname(uri).fsPath, 'base.bak'), uri.fsPath)
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(resourceDirectories().length).toBe(3)
    expect(resourceDirectories().find(resourceDirectory => resourceDirectory.getQualifiers() === 'base')).toBeDefined()
    baseResourceDirectory = resourceDirectories().find(resourceDirectory => resourceDirectory.getQualifiers() === 'base')!
    expect(baseResourceDirectory).toBeDefined()
  })

  let baseElementDirectory: ElementDirectory

  it.sequential('elementDirectory.from', async () => {
    const elementDirectory = ElementDirectory.from(baseResourceDirectory)
    expect(elementDirectory()).toBeDefined()
    const elementDirectoryUri = elementDirectory()?.getUri()
    expect(elementDirectoryUri).toBeDefined()
    fs.renameSync(elementDirectoryUri!.fsPath, path.resolve(Uri.dirname(elementDirectoryUri!).fsPath, 'element.bak'))
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(elementDirectory()?.getUri()).toBeUndefined()
    fs.renameSync(path.resolve(Uri.dirname(elementDirectoryUri!).fsPath, 'element.bak'), elementDirectoryUri!.fsPath)
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(elementDirectory()?.getUri()).toBeDefined()
    baseElementDirectory = elementDirectory()!
  })

  let baseElementStringJsonFile: ElementJsonFile

  it.sequential('elementJsonFile.findAll', async () => {
    const elementJsonFiles = ElementJsonFile.findAll(baseElementDirectory)
    const currentElementJsonFileCount = elementJsonFiles().length
    expect(currentElementJsonFileCount).toBeGreaterThanOrEqual(1)
    const stringJsonFile = elementJsonFiles().find(elementJsonFile => Uri.basename(elementJsonFile.getUri()) === 'string.json')!
    expect(stringJsonFile).toBeDefined()
    fs.renameSync(stringJsonFile.getUri().fsPath, path.resolve(Uri.dirname(stringJsonFile.getUri()).fsPath, 'string.json.bak'))
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(elementJsonFiles().length).toBeLessThan(currentElementJsonFileCount)
    expect(elementJsonFiles().find(elementJsonFile => Uri.basename(elementJsonFile.getUri()) === 'string.json')).toBeUndefined()
    fs.renameSync(path.resolve(Uri.dirname(stringJsonFile.getUri()).fsPath, 'string.json.bak'), stringJsonFile.getUri().fsPath)
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(elementJsonFiles().length).toBeGreaterThanOrEqual(currentElementJsonFileCount)
    expect(elementJsonFiles().find(elementJsonFile => Uri.basename(elementJsonFile.getUri()) === 'string.json')).toBeDefined()
    baseElementStringJsonFile = elementJsonFiles().find(elementJsonFile => Uri.basename(elementJsonFile.getUri()) === 'string.json')!
    expect(stringJsonFile).toBeDefined()
  })

  it.sequential('ElementJsonFileReference.findAll', async () => {
    const elementJsonFileReferences = ElementJsonFileReference.findAll(baseElementStringJsonFile)
    const currentElementJsonFileReferenceCount = elementJsonFileReferences().length
    const parsedContent = baseElementStringJsonFile.parse()
    fs.writeFileSync(
      baseElementStringJsonFile.getUri().fsPath,
      JSON.stringify({
        string: [
          ...parsedContent?.string ?? [],
          {
            name: '6',
            value: '6',
          },
        ],
      }, null, 2),
    )
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(elementJsonFileReferences().length).toBeGreaterThan(currentElementJsonFileReferenceCount)
    expect(elementJsonFileReferences().find(elementJsonFileReference => elementJsonFileReference.getNameText() === '6')).toBeDefined()
    fs.writeFileSync(
      baseElementStringJsonFile.getUri().fsPath,
      JSON.stringify(parsedContent, null, 2),
    )
    await new Promise(resolve => setTimeout(resolve, 2000))
    expect(elementJsonFileReferences().length).toBe(currentElementJsonFileReferenceCount)
    expect(elementJsonFileReferences().find(elementJsonFileReference => elementJsonFileReference.getNameText() === '6')).toBeUndefined()
  })
})
