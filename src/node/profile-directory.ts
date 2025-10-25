import type { Uri } from '../../index'
import type { ProjectDetector } from './project-detector'
import type { ResourceDirectory } from './resource-directory'
import { signal } from 'alien-signals'
import { ProfileDirectory as RustProfileDirectory } from '../../index'
import { DisposableSignal } from './types'

export interface ProfileDirectory extends RustProfileDirectory {
  getUnderlyingProfileDirectory(): RustProfileDirectory
}

export namespace ProfileDirectory {
  function fromRustProfileDirectory(profileDirectory: RustProfileDirectory | null): ProfileDirectory | null {
    if (!profileDirectory)
      return null
    return {
      getUri: () => profileDirectory.getUri(),
      getResourceDirectory: () => profileDirectory.getResourceDirectory(),
      findAll: () => profileDirectory.findAll(),
      getUnderlyingProfileDirectory: () => profileDirectory,
    }
  }

  export function from(resourceDirectory: ResourceDirectory): DisposableSignal<ProfileDirectory | null> {
    const profileDirectory = signal<ProfileDirectory | null>(fromRustProfileDirectory(RustProfileDirectory.from(resourceDirectory.getUnderlyingResourceDirectory())))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (uri.isEqual(resourceDirectory.getUri())) {
        switch (event) {
          case 'file-created':
          case 'file-changed':
            profileDirectory(fromRustProfileDirectory(RustProfileDirectory.from(resourceDirectory.getUnderlyingResourceDirectory())))
            break
          case 'file-deleted':
            profileDirectory(null)
            break
        }
        return
      }

      const profileUris = profileDirectory()?.findAll() ?? []
      const existingProfileIndex = profileUris.findIndex(profileUri => profileUri.isEqual(uri))
      if (existingProfileIndex !== -1) {
        profileDirectory(fromRustProfileDirectory(RustProfileDirectory.from(resourceDirectory.getUnderlyingResourceDirectory())))
      }
    }

    resourceDirectory.getResource().getProduct().getModule().getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(profileDirectory, () => resourceDirectory.getResource().getProduct().getModule().getProject().getProjectDetector().off('*', handle))
  }
}
