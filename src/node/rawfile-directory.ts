import type { Uri } from '../../index'
import type { ProjectDetector } from './project-detector'
import type { Resource } from './resource'
import { signal } from 'alien-signals'
import { RawfileDirectory as RustRawfileDirectory } from '../../index'
import { DisposableSignal } from './types'

export interface RawfileDirectory extends RustRawfileDirectory {
  getUnderlyingRawfileDirectory(): RustRawfileDirectory
}

export namespace RawfileDirectory {
  function fromRustRawfileDirectory(rawfileDirectory: RustRawfileDirectory | null): RawfileDirectory | null {
    if (!rawfileDirectory)
      return null
    return {
      getUri: () => rawfileDirectory.getUri(),
      getResource: () => rawfileDirectory.getResource(),
      findAll: () => rawfileDirectory.findAll(),
      getUnderlyingRawfileDirectory: () => rawfileDirectory,
    }
  }

  export function from(resource: Resource): DisposableSignal<RawfileDirectory | null> {
    const rawfileDirectory = signal<RawfileDirectory | null>(fromRustRawfileDirectory(RustRawfileDirectory.from(resource)))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (uri.isEqual(resource.getUri())) {
        switch (event) {
          case 'file-created':
          case 'file-changed':
            rawfileDirectory(fromRustRawfileDirectory(RustRawfileDirectory.from(resource)))
            break
          case 'file-deleted':
            rawfileDirectory(null)
            break
        }
        return
      }

      const rawfileUris = rawfileDirectory()?.findAll() ?? []
      const existingRawfileIndex = rawfileUris.findIndex(rawfileUri => rawfileUri.isEqual(uri))
      if (existingRawfileIndex !== -1) {
        rawfileDirectory(fromRustRawfileDirectory(RustRawfileDirectory.from(resource)))
      }
    }

    resource.getProduct().getModule().getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(rawfileDirectory, () => resource.getProduct().getModule().getProject().getProjectDetector().off('*', handle))
  }
}
