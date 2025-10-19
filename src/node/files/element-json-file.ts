import type { Uri } from '../../../index'
import type { ElementDirectory } from '../element-directory'
import type { ProjectDetector } from '../project-detector'
import { signal } from 'alien-signals'
import { ElementJsonFile as RustElementJsonFile } from '../../../index'
import { DisposableSignal } from '../types'

export interface ElementJsonFile extends RustElementJsonFile {
  getElementDirectory(): ElementDirectory
  getUnderlyingElementJsonFile(): RustElementJsonFile
}

export namespace ElementJsonFile {
  function fromRustElementJsonFile(elementJsonFile: RustElementJsonFile, elementDirectory: ElementDirectory): ElementJsonFile {
    return {
      getElementDirectory: () => elementDirectory,
      getUnderlyingElementJsonFile: () => elementJsonFile,
      getUri: () => elementJsonFile.getUri(),
      getContent: () => elementJsonFile.getContent(),
      parse: () => elementJsonFile.parse(),
    }
  }

  export function findAll(elementDirectory: ElementDirectory): DisposableSignal<ElementJsonFile[]> {
    const elementJsonFiles = signal<ElementJsonFile[]>(RustElementJsonFile.findAll(elementDirectory.getUnderlyingElementDirectory()).map(elementJsonFile => fromRustElementJsonFile(elementJsonFile, elementDirectory)))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (elementDirectory.getResourceDirectory().getResource().getProduct().getModule().getBuildProfileUri().isEqual(uri) || elementDirectory.getResourceDirectory().getResource().getProduct().getModule().getProject().getBuildProfileUri().isEqual(uri)) {
        elementJsonFiles(RustElementJsonFile.findAll(elementDirectory.getUnderlyingElementDirectory()).map(elementJsonFile => fromRustElementJsonFile(elementJsonFile, elementDirectory)))
        return
      }

      switch (event) {
        case 'file-created': {
          const elementJsonFile = RustElementJsonFile.create(elementDirectory.getUnderlyingElementDirectory(), uri.fsPath)
          console.warn(`[ELEMENT JSON FILE] ${event} ${uri.fsPath} ${elementJsonFile ? 'created' : 'not created'}`)
          if (elementJsonFile) {
            elementJsonFiles([...elementJsonFiles(), fromRustElementJsonFile(elementJsonFile, elementDirectory)].filter(Boolean))
          }
          break
        }
        case 'file-deleted': {
          elementJsonFiles(elementJsonFiles().filter(elementJsonFile => !elementJsonFile.getUri().isEqual(uri)))
          break
        }
        case 'file-changed': {
          const files = elementJsonFiles()
          const index = files.findIndex(file => file.getUri().isEqual(uri))
          RustElementJsonFile.reload(files[index].getUnderlyingElementJsonFile())
          break
        }
      }
    }

    elementDirectory.getResourceDirectory().getResource().getProduct().getModule().getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(elementJsonFiles, () => elementDirectory.getResourceDirectory().getResource().getProduct().getModule().getProject().getProjectDetector().off('*', handle))
  }
}
