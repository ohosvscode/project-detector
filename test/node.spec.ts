import fs from 'node:fs'
import path from 'node:path'
import { describe, expect } from 'vitest'
import { Uri } from '../index'
import { Chokidar, Project, ProjectDetector } from '../src/node'

describe.sequential('projectDetector', (it) => {
  const mockPath = path.resolve(__dirname, '..', 'mock')

  let projectDetector: ProjectDetector

  it.sequential('projectDetector.create', async () => {
    projectDetector = ProjectDetector.create(Uri.file(mockPath).toString())
    expect(projectDetector.getWorkspaceFolder().fsPath).toBe(mockPath)
    await Chokidar.create(projectDetector)
  })

  it.sequential('project.findAll', async () => {
    const projects = Project.findAll(projectDetector)
    expect(projects().length).toBe(2)
    const harmonyProject1 = projects().find(project => project.getUri().toString().includes('harmony-project-1'))!
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
})
