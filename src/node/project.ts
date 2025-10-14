import type { Uri } from '../../index'
import type { ProjectDetector } from './project-detector'
import { signal } from 'alien-signals'
import { Project as RustProject } from '../../index'
import { DisposableSignal } from './types'

export interface Project extends RustProject {
  getUnderlyingProject(): RustProject
}

export namespace Project {
  function boundRustProject(project: RustProject): Project {
    return {
      getBuildProfileContent: () => project.getBuildProfileContent(),
      getBuildProfileUri: () => project.getBuildProfileUri(),
      getParsedBuildProfile: () => project.getParsedBuildProfile(),
      getUri: () => project.getUri(),
      getUnderlyingProject: () => project,
      getProjectDetector: () => project.getProjectDetector(),
    }
  }

  export function findAll(projectDetector: ProjectDetector): DisposableSignal<Project[]> {
    const underlyingProjectDetector = projectDetector.getUnderlyingProjectDetector()
    const projects = signal<Project[]>(RustProject.findAll(underlyingProjectDetector).map(boundRustProject))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (event === 'file-created' && uri.fsPath.endsWith('build-profile.json5')) {
        projects(RustProject.findAll(underlyingProjectDetector).map(boundRustProject))
        const newProject = RustProject.create(underlyingProjectDetector, uri.fsPath)
        if (newProject) {
          projects([...projects(), boundRustProject(newProject)])
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
          currentProjects[existingProjectIndex] = boundRustProject(RustProject.create(underlyingProjectDetector, uri.fsPath)!)
          projects(currentProjects)
        }
      }
    }

    projectDetector.on('*', handle)
    return DisposableSignal.create(projects, () => projectDetector.off('*', handle)) as DisposableSignal<Project[]>
  }
}
