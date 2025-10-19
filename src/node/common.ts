import type { ProjectDetector } from './project-detector'
import type { Signal } from './types'
import { Uri } from '../../index'

export interface CompatibleObject {
  getUri(): Uri
  getBuildProfileUri(): Uri
}

/** @internal */
export function createProjectModuleHandler<InputCompatibleObject extends CompatibleObject, OutputCompatibleObject extends CompatibleObject>(
  signal: Signal<OutputCompatibleObject[]>,
  rawCreator: (uri: Uri) => InputCompatibleObject | null | undefined,
  toTsSideObject: (object: InputCompatibleObject) => OutputCompatibleObject,
  reload: (object: OutputCompatibleObject) => void,
) {
  return (event: keyof ProjectDetector.EventMap, uri: Uri) => {
    if (event === 'file-created' && uri.fsPath.endsWith('build-profile.json5')) {
      const projects = signal()
      const existingProjectIndex = projects.findIndex(project => project.getUri().isEqual(uri) || project.getBuildProfileUri().isEqual(uri))
      if (existingProjectIndex === -1) {
        const newProject = rawCreator(Uri.dirname(uri))
        if (newProject) {
          signal([...projects, toTsSideObject(newProject)] as OutputCompatibleObject[])
        }
      }
      else {
        projects[existingProjectIndex] = toTsSideObject(rawCreator(uri)!) as OutputCompatibleObject
        signal(projects)
      }
    }
    else if (event === 'file-deleted' && uri.fsPath.endsWith('build-profile.json5')) {
      const projects = signal()
      const existingProjectIndex = projects.findIndex(project => project.getUri().isEqual(uri) || project.getBuildProfileUri().isEqual(uri))
      if (existingProjectIndex !== -1) {
        signal(projects.filter((_, index) => index !== existingProjectIndex))
      }
    }
    else if (event === 'file-changed' && uri.fsPath.endsWith('build-profile.json5')) {
      const projects = signal()
      const existingProjectIndex = projects.findIndex(project => project.getUri().isEqual(uri) || project.getBuildProfileUri().isEqual(uri))
      if (existingProjectIndex !== -1) {
        reload(projects[existingProjectIndex])
      }
    }
  }
}
