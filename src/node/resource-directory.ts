import type { Uri } from '../../index'
import type { ProjectDetector } from './project-detector'
import type { Resource } from './resource'
import path from 'node:path'
import { signal } from 'alien-signals'
import { ResourceDirectory as RustResourceDirectory } from '../../index'
import { DisposableSignal } from './types'

export interface ResourceDirectory extends RustResourceDirectory {
  getResource(): Resource
  getUnderlyingResourceDirectory(): RustResourceDirectory
}

export namespace ResourceDirectory {
  function fromRustResourceDirectory(resourceDirectory: RustResourceDirectory, resource: Resource): ResourceDirectory {
    return {
      getUri: () => resourceDirectory.getUri(),
      getQualifiers: () => resourceDirectory.getQualifiers(),
      getResource: () => resource,
      getUnderlyingResourceDirectory: () => resourceDirectory,
    }
  }

  export function findAll(resource: Resource): DisposableSignal<ResourceDirectory[]> {
    const resourceDirectories = signal<ResourceDirectory[]>(RustResourceDirectory.findAll(resource.getUnderlyingResource()).map(resourceDirectory => fromRustResourceDirectory(resourceDirectory, resource)))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (resource.getProduct().getModule().getBuildProfileUri().isEqual(uri) || resource.getProduct().getModule().getProject().getBuildProfileUri().isEqual(uri)) {
        resourceDirectories(RustResourceDirectory.findAll(resource.getUnderlyingResource()).map(resourceDirectory => fromRustResourceDirectory(resourceDirectory, resource)))
        return
      }

      // When the resources/* has been changed, we need to reload the all resource directories
      if (uri.fsPath.split(resource.getUri().fsPath)[1]?.split(path.sep).filter(Boolean).length !== 1)
        return

      switch (event) {
        case 'file-created': {
          const resourceDirectory = RustResourceDirectory.create(resource.getUnderlyingResource(), uri.fsPath)
          if (resourceDirectory) {
            resourceDirectories([...resourceDirectories(), fromRustResourceDirectory(resourceDirectory, resource)])
          }
          break
        }
        case 'file-deleted': {
          resourceDirectories(resourceDirectories().filter(resourceDirectory => !resourceDirectory.getUri().isEqual(uri)))
          break
        }
        case 'file-changed': {
          resourceDirectories(RustResourceDirectory.findAll(resource.getUnderlyingResource()).map(resourceDirectory => fromRustResourceDirectory(resourceDirectory, resource)))
          break
        }
      }
    }

    resource.getProduct().getModule().getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(resourceDirectories, () => resource.getProduct().getModule().getProject().getProjectDetector().off('*', handle))
  }
}
