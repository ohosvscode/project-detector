import { describe, expect } from 'vitest'
import { createRequire } from 'node:module'
import { URI } from 'vscode-uri'
import path from 'node:path'

describe.sequential('sample', (it) => {
  const require = createRequire(import.meta.url)
  const { ProjectDetector, Project, Module }: typeof import('../dist') = require('../dist')

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
    const project2Modules = Module.findAll(project2)
    expect(project2Modules).toHaveLength(1)
    project2Module1 = project2Modules[0]
    expect(project2Module1.getProject() === project2).toBe(true)
  })
})