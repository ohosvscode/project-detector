import type { Uri } from '../../index'
import type { ProjectDetector } from './project-detector'
import type { ResourceDirectory } from './resource-directory'
import path from 'node:path'
import { signal } from 'alien-signals'
import { ElementDirectory as RustElementDirectory } from '../../index'
import { DisposableSignal } from './types'

export interface ElementDirectory extends RustElementDirectory {
  getResourceDirectory(): ResourceDirectory
  getUnderlyingElementDirectory(): RustElementDirectory
}

export namespace ElementDirectory {
  function fromRustElementDirectory(elementDirectory: RustElementDirectory | null, resourceDirectory: ResourceDirectory): ElementDirectory | null {
    if (!elementDirectory)
      return null

    return {
      getResourceDirectory: () => resourceDirectory,
      getUnderlyingElementDirectory: () => elementDirectory,
      getUri: () => elementDirectory.getUri(),
    }
  }

  export function from(resourceDirectory: ResourceDirectory): DisposableSignal<ElementDirectory | null> {
    const elementDirectory = signal<ElementDirectory | null>(fromRustElementDirectory(RustElementDirectory.from(resourceDirectory.getUnderlyingResourceDirectory()), resourceDirectory))

    const handle = (_event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (resourceDirectory.getResource().getProduct().getModule().getBuildProfileUri().isEqual(uri) || resourceDirectory.getResource().getProduct().getModule().getProject().getBuildProfileUri().isEqual(uri)) {
        elementDirectory(fromRustElementDirectory(RustElementDirectory.from(resourceDirectory.getUnderlyingResourceDirectory()), resourceDirectory))
        return
      }

      if (uri.fsPath.split(resourceDirectory.getUri().fsPath)[1]?.split(path.sep).filter(Boolean).length !== 1) {
        elementDirectory(fromRustElementDirectory(RustElementDirectory.from(resourceDirectory.getUnderlyingResourceDirectory()), resourceDirectory))
      }
    }

    resourceDirectory.getResource().getProduct().getModule().getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(elementDirectory, () => resourceDirectory.getResource().getProduct().getModule().getProject().getProjectDetector().off('*', handle))
  }
}
