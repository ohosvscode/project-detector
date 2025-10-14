import type { Emitter } from 'mitt'
import type { Uri } from '../../index'
import mitt from 'mitt'
import { ProjectDetector as RustProjectDetector } from '../../index'

export interface ProjectDetector extends Emitter<ProjectDetector.EventMap> {
  getWorkspaceFolder(): Uri
  getProjectDetector(): RustProjectDetector
}

export namespace ProjectDetector {

  // eslint-disable-next-line ts/consistent-type-definitions
  export type EventMap = {
    'file-changed': Uri
    'file-deleted': Uri
    'file-created': Uri
  }

  export function create(workspaceFolder: string): ProjectDetector {
    const projectDetector = RustProjectDetector.create(workspaceFolder)
    const emitter = mitt<ProjectDetector.EventMap>()

    return {
      all: emitter.all,
      on: emitter.on,
      off: emitter.off,
      emit: emitter.emit,
      getWorkspaceFolder: () => projectDetector.getWorkspaceFolder(),
      getProjectDetector: () => projectDetector,
    }
  }
}
