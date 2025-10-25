import type { Uri } from '../../index'
import type { ProjectDetector } from './project-detector'
import type { Resource } from './resource'
import { signal } from 'alien-signals'
import { ResfileDirectory as RustResfileDirectory } from '../../index'
import { DisposableSignal } from './types'

export interface ResfileDirectory extends RustResfileDirectory {
  getUnderlyingResfileDirectory(): RustResfileDirectory
}

export namespace ResfileDirectory {
  function fromRustResfileDirectory(resfileDirectory: RustResfileDirectory | null): ResfileDirectory | null {
    if (!resfileDirectory)
      return null
    return {
      getUri: () => resfileDirectory.getUri(),
      getResource: () => resfileDirectory.getResource(),
      findAll: () => resfileDirectory.findAll(),
      getUnderlyingResfileDirectory: () => resfileDirectory,
    }
  }

  export function from(resource: Resource): DisposableSignal<ResfileDirectory | null> {
    const resfileDirectory = signal<ResfileDirectory | null>(fromRustResfileDirectory(RustResfileDirectory.from(resource.getUnderlyingResource())))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (uri.isEqual(resource.getUri())) {
        switch (event) {
          case 'file-created':
          case 'file-changed':
            resfileDirectory(fromRustResfileDirectory(RustResfileDirectory.from(resource.getUnderlyingResource())))
            break
          case 'file-deleted':
            resfileDirectory(null)
            break
        }
        return
      }

      const resfileUris = resfileDirectory()?.findAll() ?? []
      const existingResfileIndex = resfileUris.findIndex(resfileUri => resfileUri.isEqual(uri))
      if (existingResfileIndex !== -1) {
        resfileDirectory(fromRustResfileDirectory(RustResfileDirectory.from(resource.getUnderlyingResource())))
      }
    }

    resource.getProduct().getModule().getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(resfileDirectory, () => resource.getProduct().getModule().getProject().getProjectDetector().off('*', handle))
  }
}
