import type { Uri } from '../../index'
import type { ModuleLevelBuildProfile } from './interfaces/module-build-profile'
import type { Module } from './module'
import type { ProjectDetector } from './project-detector'
import { signal } from 'alien-signals'
import { Product as RustProduct } from '../../index'
import { DisposableSignal } from './types'

/**
 * Single {@linkcode Module} contain multiple products.
 *
 * Usually, application vendors
 * will customize the same application for different versions based on different
 * deployment environments, different target audiences, different runtime environments, etc.,
 * such as domestic version, international version, normal version, VIP version,
 * free version, paid version, etc. For the above scenarios supports the use of a small
 * amount of code configuration to instantiate different versions of differences,
 * and achieve the high reuse of source code and resource files through different target
 * product versions during the compilation and build process.
 *
 * @see https://developer.huawei.com/consumer/en/doc/harmonyos-guides/ide-customized-multi-targets-and-products-guides#section1011341611469
 *
 * ---
 * 一个 {@linkcode Module} 包含多个product。
 *
 * 通常情况下，应用厂商会根据不同的部署环境，不同的目标人群，不同的运行环境等，
 * 将同一个应用定制为不同的版本，如国内版、国际版、普通版、VIP版、免费版、付费版等。
 * 针对以上场景支持通过少量的代码配置以实例化不同的差异版本，在编译构建过程中实现
 * 一个应用构建出不同的目标产物版本，从而实现源代码、资源文件等的高效复用。
 *
 * @see https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/ide-customized-multi-targets-and-products-guides#section1011341611469
 */
export interface Product extends RustProduct {
  getCurrentTargetConfig(): ModuleLevelBuildProfile.Target
  getModule(): Module
  getUnderlyingProduct(): RustProduct
}

export namespace Product {
  function fromRustProduct(product: RustProduct, module: Module): Product {
    return {
      getModule: () => module,
      getName: () => product.getName(),
      getCurrentTargetConfig: () => product.getCurrentTargetConfig(),
      getSourceDirectories: () => product.getSourceDirectories(),
      getResourceDirectories: () => product.getResourceDirectories(),
      getUnderlyingProduct: () => product,
    }
  }

  export function findAll(module: Module): DisposableSignal<Product[]> {
    const products = signal<Product[]>(RustProduct.findAll(module.getUnderlyingModule()).map(product => fromRustProduct(product, module)))
    const handle = (event: keyof ProjectDetector.EventMap, uri: Uri) => {
      switch (event) {
        case 'file-changed': {
          if (module.getBuildProfileUri().isEqual(uri)) {
            products(RustProduct.findAll(module.getUnderlyingModule()).map(product => fromRustProduct(product, module)))
          }
          break
        }
        case 'file-created': {
          if (module.getBuildProfileUri().isEqual(uri)) {
            products(RustProduct.findAll(module.getUnderlyingModule()).map(product => fromRustProduct(product, module)))
          }
          break
        }
        case 'file-deleted': {
          if (module.getBuildProfileUri().isEqual(uri)) {
            products([])
          }
          break
        }
        default: {
          break
        }
      }
    }

    module.getProject().getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(products, () => module.getProject().getProjectDetector().off('*', handle))
  }
}
