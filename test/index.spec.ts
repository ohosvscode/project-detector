import path from 'node:path'
import MagicString from 'magic-string'
import { describe, expect } from 'vitest'
import { ElementDirectory, ElementJsonFile, ElementJsonFileReference, Module, Product, Project, ProjectDetector, Resource, ResourceDirectory, Uri } from '../index.js'

describe.sequential('projectDetector', (it) => {
  const mockPath = path.resolve(__dirname, '..', 'mock')
  let projectDetector: ProjectDetector

  it.sequential('projectDetector.create', () => {
    projectDetector = ProjectDetector.create(Uri.file(mockPath).toString())
    expect(projectDetector.getWorkspaceFolder().fsPath).toBe(mockPath)
  })

  let harmonyProject1: Project
  let harmonyProject2: Project

  it.sequential('project.findAll', () => {
    const projects = Project.findAll(projectDetector)
    harmonyProject1 = projects.find(project => project.getUri().toString().includes('harmony-project-1'))!
    harmonyProject2 = projects.find(project => project.getUri().toString().includes('harmony-project-2'))!
    expect(harmonyProject1).toBeDefined()
    expect(harmonyProject2).toBeDefined()
  })

  let harmonyProject1Module: Module
  let harmonyProject2Module: Module

  it.sequential('module.findAll', () => {
    const harmonyProject1Modules = Module.findAll(harmonyProject1)
    expect(harmonyProject1Modules.length).toBe(1)
    harmonyProject1Module = harmonyProject1Modules[0]
    expect(harmonyProject1Module).toBeDefined()
    const harmonyProject2Modules = Module.findAll(harmonyProject2)
    expect(harmonyProject2Modules.length).toBe(1)
    harmonyProject2Module = harmonyProject2Modules[0]
    expect(harmonyProject2Module).toBeDefined()
  })

  let harmonyProject1MainProduct: Product
  let harmonyProject1OhosTestProduct: Product
  let harmonyProject2MainProduct: Product
  let harmonyProject2OhosTestProduct: Product

  it.sequential('product.findAll', () => {
    const harmonyProject1Products = Product.findAll(harmonyProject1Module)
    expect(harmonyProject1Products.length).toBe(2)
    harmonyProject1MainProduct = harmonyProject1Products.find(product => product.getName() === 'default')!
    expect(harmonyProject1MainProduct).toBeDefined()
    harmonyProject1OhosTestProduct = harmonyProject1Products.find(product => product.getName() === 'ohosTest')!
    expect(harmonyProject1OhosTestProduct).toBeDefined()

    const harmonyProject2Products = Product.findAll(harmonyProject2Module)
    expect(harmonyProject2Products.length).toBe(2)
    harmonyProject2MainProduct = harmonyProject2Products.find(product => product.getName() === 'default')!
    expect(harmonyProject2MainProduct).toBeDefined()
    harmonyProject2OhosTestProduct = harmonyProject2Products.find(product => product.getName() === 'ohosTest')!
    expect(harmonyProject2OhosTestProduct).toBeDefined()
  })

  let harmonyProject1MainResource: Resource

  it.sequential('resource.findAll', () => {
    const harmonyProject1Resources = Resource.findAll(harmonyProject1MainProduct)
    expect(harmonyProject1Resources.length).toBe(1)
    harmonyProject1MainResource = harmonyProject1Resources[0]
    expect(harmonyProject1MainResource).toBeDefined()
  })

  let harmonyProject1MainBaseResource: ResourceDirectory

  it.sequential('resourceDirectory.findAll', () => {
    const harmonyProject1MainResourceDirectories = ResourceDirectory.findAll(harmonyProject1MainResource)
    harmonyProject1MainBaseResource = harmonyProject1MainResourceDirectories.find(resourceDirectory => resourceDirectory.getUri().toString().includes('base'))!
    expect(harmonyProject1MainBaseResource).toBeDefined()
  })

  let stringJsonFile: ElementJsonFile

  it.sequential('elementDirectory & elementJsonFile.findAll', () => {
    const elementDirectory = ElementDirectory.from(harmonyProject1MainBaseResource)
    const elementJsonFiles = ElementJsonFile.findAll(elementDirectory)
    expect(elementJsonFiles.length).toBeGreaterThanOrEqual(1)
    stringJsonFile = elementJsonFiles.find(elementJsonFile => elementJsonFile.getUri().toString().includes('string.json'))!
    expect(stringJsonFile).toBeDefined()
  })

  it.sequential('ElementJsonFileReference.findAll', () => {
    const references = ElementJsonFileReference.findAll(stringJsonFile)
    expect(references.length).toBeGreaterThanOrEqual(1)
    const ms = new MagicString(stringJsonFile.getContent())
    for (const reference of references) {
      expect(ms.slice(reference.getNameStart(), reference.getNameEnd())).toBe(reference.getNameFullText())
      expect(ms.slice(reference.getValueStart(), reference.getValueEnd())).toBe(reference.getValueFullText())
      expect(reference.getNameFullText()).toBe(`"${reference.getNameText()}"`)
      expect(reference.getValueFullText()).toBe(`"${reference.getValueText()}"`)
      expect(reference.toEtsFormat()).toBe(`app.${reference.getElementType()}.${reference.getNameText()}`)
      expect(reference.toJsonFormat()).toBe(`$${reference.getElementType()}:${reference.getNameText()}`)
    }
  })
})
