import type { ProjectDetector } from './project-detector'
import type { Signal } from './types'
import { signal } from 'alien-signals'
import { Project as RustProject } from '../../index'

export interface Project extends RustProject {
}

export namespace Project {
  export function findAll(projectDetector: ProjectDetector): Signal<Project[]> {
    const projects: Signal<Project[]> = signal(RustProject.findAll(projectDetector.getProjectDetector()))

    projectDetector.on('*', (event, uri) => {
      if (event === 'file-created' && uri.fsPath.endsWith('build-profile.json5')) {
        projects(RustProject.findAll(projectDetector.getProjectDetector()))
        const newProject = RustProject.create(projectDetector.getProjectDetector(), uri.fsPath)
        if (newProject) {
          projects([...projects(), newProject])
        }
      }
      else if (event === 'file-deleted' && uri.fsPath.endsWith('build-profile.json5')) {
        const existingProjectIndex = projects().findIndex(project => project.getUri().isEqual(uri) || project.getBuildProfileUri().isEqual(uri))
        if (existingProjectIndex !== -1) {
          projects(projects().filter((_, index) => index !== existingProjectIndex))
        }
      }
      else if (event === 'file-changed' && uri.fsPath.endsWith('build-profile.json5')) {
        const existingProjectIndex = projects().findIndex(project => project.getUri().isEqual(uri) || project.getBuildProfileUri().isEqual(uri))
        if (existingProjectIndex !== -1) {
          const currentProjects = projects()
          currentProjects[existingProjectIndex] = RustProject.create(projectDetector.getProjectDetector(), uri.fsPath)!
          projects(currentProjects)
        }
      }
    })

    return projects
  }
}
