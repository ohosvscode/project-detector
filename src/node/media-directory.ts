import type { Uri } from '../../index'
import type { ProjectDetector } from './project-detector'
import type { ResourceDirectory } from './resource-directory'
import { signal } from 'alien-signals'
import { MediaDirectory as RustMediaDirectory } from '../../index'
import { DisposableSignal } from './types'

export interface MediaDirectory extends RustMediaDirectory {
  getUnderlyingMediaDirectory(): RustMediaDirectory
}

export namespace MediaDirectory {
  function fromRustMediaDirectory(mediaDirectory: RustMediaDirectory | null): MediaDirectory | null {
    if (!mediaDirectory)
      return null
    return {
      getUri: () => mediaDirectory.getUri(),
      getResourceDirectory: () => mediaDirectory.getResourceDirectory(),
      findAll: () => mediaDirectory.findAll(),
      getUnderlyingMediaDirectory: () => mediaDirectory,
    }
  }

  export function from(resourceDirectory: ResourceDirectory): DisposableSignal<MediaDirectory | null> {
    const mediaDirectory = signal<MediaDirectory | null>(fromRustMediaDirectory(RustMediaDirectory.from(resourceDirectory.getUnderlyingResourceDirectory())))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (uri.isEqual(resourceDirectory.getUri())) {
        switch (event) {
          case 'file-created':
          case 'file-changed':
            mediaDirectory(fromRustMediaDirectory(RustMediaDirectory.from(resourceDirectory.getUnderlyingResourceDirectory())))
            break
          case 'file-deleted':
            mediaDirectory(null)
            break
        }
        return
      }

      const mediaFileUris = mediaDirectory()?.findAll() ?? []
      const existingMediaFileIndex = mediaFileUris.findIndex(mediaFileUri => mediaFileUri.isEqual(uri))
      if (existingMediaFileIndex !== -1) {
        mediaDirectory(fromRustMediaDirectory(RustMediaDirectory.from(resourceDirectory.getUnderlyingResourceDirectory())))
      }
    }

    resourceDirectory.getResource().getProduct().getModule().getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(mediaDirectory, () => resourceDirectory.getResource().getProduct().getModule().getProject().getProjectDetector().off('*', handle))
  }
}
