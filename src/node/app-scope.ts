import type { Uri } from '../../index'
import type { Project } from './project'
import type { ProjectDetector } from './project-detector'
import { signal } from 'alien-signals'
import { AppScope as RustAppScope } from '../../index'
import { DisposableSignal } from './types'

export interface AppScope extends RustAppScope {}

export namespace AppScope {
  export function from(project: Project): DisposableSignal<AppScope | null> {
    const appScope = signal<AppScope | null>(RustAppScope.from(project.getUnderlyingProject()))

    const handle = (_event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (project.getBuildProfileUri().isEqual(uri) || project.getUri().isEqual(uri)) {
        appScope(RustAppScope.from(project.getUnderlyingProject()))
      }
      else if (uri.fsPath.endsWith('app.json5')) {
        appScope(RustAppScope.from(project.getUnderlyingProject()))
      }
    }

    project.getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(appScope, () => project.getProjectDetector().off('*', handle))
  }
}
