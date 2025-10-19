import type { Product } from './product'
import { signal } from 'alien-signals'
import { Resource as RustResource } from '../../index'
import { DisposableSignal } from './types'

export interface Resource extends RustResource {
  getProduct(): Product
  getUnderlyingResource(): RustResource
}

export namespace Resource {
  function fromRustResource(resource: RustResource, product: Product): Resource {
    return {
      getProduct: () => product,
      getUri: () => resource.getUri(),
      getQualifiedDirectories: () => resource.getQualifiedDirectories(),
      getUnderlyingResource: () => resource,
    }
  }

  export function findAll(product: Product): DisposableSignal<Resource[]> {
    const resources = signal<Resource[]>(RustResource.findAll(product.getUnderlyingProduct()).map(resource => fromRustResource(resource, product)))

    return DisposableSignal.fromSignal(resources)
  }
}
