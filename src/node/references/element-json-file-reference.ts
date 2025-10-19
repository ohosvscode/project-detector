import type { Uri } from '../../../index'
import type { ElementJsonFile } from '../files/element-json-file'
import type { ProjectDetector } from '../project-detector'
import { signal } from 'alien-signals'
import { ElementJsonFileReference as RustElementJsonFileReference } from '../../../index'
import { DisposableSignal } from '../types'

export interface ElementJsonFileReference extends RustElementJsonFileReference {
  getElementJsonFile(): ElementJsonFile
  getUnderlyingElementJsonFileReference(): RustElementJsonFileReference
}

export namespace ElementJsonFileReference {
  function fromRustElementJsonFileReference(elementJsonFileReference: RustElementJsonFileReference, elementJsonFile: ElementJsonFile): ElementJsonFileReference {
    return {
      getElementJsonFile: () => elementJsonFile,
      getUnderlyingElementJsonFileReference: () => elementJsonFileReference,
      getElementType: () => elementJsonFileReference.getElementType(),
      getNameStart: () => elementJsonFileReference.getNameStart(),
      getNameEnd: () => elementJsonFileReference.getNameEnd(),
      getNameText: () => elementJsonFileReference.getNameText(),
      getNameFullText: () => elementJsonFileReference.getNameFullText(),
      getValueStart: () => elementJsonFileReference.getValueStart(),
      getValueEnd: () => elementJsonFileReference.getValueEnd(),
      getValueText: () => elementJsonFileReference.getValueText(),
      getValueFullText: () => elementJsonFileReference.getValueFullText(),
      getFullElementType: () => elementJsonFileReference.getFullElementType(),
      toEtsFormat: () => elementJsonFileReference.toEtsFormat(),
      toJsonFormat: () => elementJsonFileReference.toJsonFormat(),
    }
  }

  export function findAll(elementJsonFile: ElementJsonFile): DisposableSignal<ElementJsonFileReference[]> {
    const elementJsonFileReferences = signal<ElementJsonFileReference[]>(RustElementJsonFileReference.findAll(elementJsonFile.getUnderlyingElementJsonFile()).map(elementJsonFileReference => fromRustElementJsonFileReference(elementJsonFileReference, elementJsonFile)))

    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      if (elementJsonFile.getElementDirectory().getResourceDirectory().getResource().getProduct().getModule().getBuildProfileUri().isEqual(uri) || elementJsonFile.getElementDirectory().getResourceDirectory().getResource().getProduct().getModule().getProject().getBuildProfileUri().isEqual(uri)) {
        elementJsonFileReferences(RustElementJsonFileReference.findAll(elementJsonFile.getUnderlyingElementJsonFile()).map(elementJsonFileReference => fromRustElementJsonFileReference(elementJsonFileReference, elementJsonFile)))
      }

      if (!uri.isEqual(elementJsonFile.getUri()))
        return

      switch (event) {
        case 'file-created': {
          elementJsonFileReferences([])
          break
        }
        case 'file-deleted': {
          elementJsonFileReferences([])
          break
        }
        case 'file-changed': {
          elementJsonFileReferences(RustElementJsonFileReference.findAll(elementJsonFile.getUnderlyingElementJsonFile()).map(elementJsonFileReference => fromRustElementJsonFileReference(elementJsonFileReference, elementJsonFile)))
          break
        }
      }
    }

    elementJsonFile.getElementDirectory()
      .getResourceDirectory()
      .getResource()
      .getProduct()
      .getModule()
      .getProject()
      .getProjectDetector()
      .on('*', handle)

    return DisposableSignal.fromSignal(elementJsonFileReferences, () => elementJsonFile
      .getElementDirectory()
      .getResourceDirectory()
      .getResource()
      .getProduct()
      .getModule()
      .getProject()
      .getProjectDetector()
      .off('*', handle))
  }
}
