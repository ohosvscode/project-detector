import { describe, expect } from 'vitest'
import { createRequire } from 'node:module'
import { URI, Utils as UriUtils } from 'vscode-uri'
import path from 'node:path'
import fs from 'node:fs'
import MagicString from 'magic-string'

describe.sequential('sample', (it) => {
  const require = createRequire(import.meta.url)
  const { ProjectDetector, Project, Module, Product, Resource, ResourceGroup, ElementJsonFileNameReference }: typeof import('../dist') = require('../dist')

  let projectDetector: InstanceType<typeof ProjectDetector>

  it.sequential('should create a project detector', () => {
    const workspaceFolder = URI.file(path.resolve(__dirname, '..', 'mock'))
    projectDetector = ProjectDetector.create(workspaceFolder.toString())
    expect(projectDetector.getWorkspaceFolder()).toBe(workspaceFolder.toString())
  })

  let project1: InstanceType<typeof Project>
  let project2: InstanceType<typeof Project>

  it.sequential('should find all projects', () => {
    const projects = Project.findAll(projectDetector)
    expect(projects).toHaveLength(2)
    
    for (const project of projects) {
      const parsed = project.getParsedBuildProfileContent()
      const buildProfileContent = project.getBuildProfileContent()
      expect(project.getProjectDetector() === projectDetector).toBe(true)
      expect(JSON.parse(buildProfileContent)).toEqual(parsed)
    }

    project1 = projects[0]
    project2 = projects[1]
  })

  let project1Module1: InstanceType<typeof Module>
  let project2Module1: InstanceType<typeof Module>

  it.sequential('should find all modules', () => {
    const project1Modules = Module.findAll(project1)
    expect(project1Modules).toHaveLength(1)
    project1Module1 = project1Modules[0]
    expect(project1Module1.getProject() === project1).toBe(true)
    expect(JSON.parse(project1Module1.getBuildProfileContent())).toEqual(project1Module1.getParsedBuildProfileContent())
    const project2Modules = Module.findAll(project2)
    expect(project2Modules).toHaveLength(1)
    project2Module1 = project2Modules[0]
    expect(project2Module1.getProject() === project2).toBe(true)
    expect(JSON.parse(project2Module1.getBuildProfileContent())).toEqual(project2Module1.getParsedBuildProfileContent())
  })

  let project1Product1: InstanceType<typeof Product>
  let project1Product2: InstanceType<typeof Product>
  let resourceDirectory1: string
  let resourceDirectory2: string

  it('should find all products', () => {
    const project1Products = Product.findAll(project1Module1)
    expect(project1Products).toHaveLength(2)
    project1Product1 = project1Products[0]
    project1Product2 = project1Products[1]
    resourceDirectory1 = project1Product1.getResourceDirectories()[0]
    resourceDirectory2 = project1Product2.getResourceDirectories()[0]

    expect(resourceDirectory1).toBeDefined()
    expect(resourceDirectory2).toBeDefined()
  })

  let resource1: InstanceType<typeof Resource>[]
  let resource2: InstanceType<typeof Resource>[]
  let qualifiedDirectories1: import('../dist').ResourceQualifiedDirectory[]
  let qualifiedDirectories2: import('../dist').ResourceQualifiedDirectory[]

  it('should find all qualified resource directories', () => {
    resource1 = Resource.findAll(project1Product1)
    expect(resource1).toHaveLength(1)
    resource2 = Resource.findAll(project1Product2)
    expect(resource2).toHaveLength(1)
    qualifiedDirectories1 = resource1[0].getQualifiedDirectories()
    qualifiedDirectories2 = resource2[0].getQualifiedDirectories()

    expect(qualifiedDirectories1.length).toBeGreaterThanOrEqual(1)
    expect(qualifiedDirectories2.length).toBeGreaterThanOrEqual(1)
  })

  let elementJsonFiles: InstanceType<typeof import('../dist').ElementJsonFile>[]
  it('should find all resource groups', () => {
    const resourceGroups = ResourceGroup.findAll(resource1)
    const baseGroup = resourceGroups.find(group => group.isBase())
    expect(baseGroup).toBeDefined()
    elementJsonFiles = baseGroup!.getElementJsonFiles()
    expect(elementJsonFiles.length).toBeGreaterThanOrEqual(1)
  })

  it('should find all element json file name references', () => {
    const colorJsonFile = elementJsonFiles.find(file => UriUtils.basename(URI.parse(file.getUri())) === 'color.json')
    expect(colorJsonFile).toBeDefined()
    const nameReferences = ElementJsonFileNameReference.findAll(colorJsonFile!)
    expect(nameReferences.length).toBeGreaterThanOrEqual(1)
    const primaryColorReference = nameReferences.find(reference => reference.getText() === 'primary_color')
    expect(primaryColorReference).toBeDefined()
    const colorJsonText = fs.readFileSync(URI.parse(colorJsonFile!.getUri()).fsPath, 'utf-8')
    const ms = new MagicString(colorJsonText)
    expect(ms.slice(primaryColorReference!.getStart(), primaryColorReference!.getEnd())).toBe('primary_color')
  })
})