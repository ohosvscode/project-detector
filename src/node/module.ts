import type { ModuleLevelBuildProfile } from './interfaces/module-build-profile'
import type { Project } from './project'
import { signal } from 'alien-signals'
import { Module as RustModule } from '../../index'
import { createProjectModuleHandler } from './common'
import { DisposableSignal } from './types'

/**
 * Single {@linkcode Workspace} contains multiple modules.
 *
 * As a basic functional unit of apps/atomic services, a module contains source
 * code, resource files, third-party libraries, and configuration files.
 *
 * It must contain the `build-profile.json5` and `oh-package.json5` files at the
 * project level, so the current module implementation provides
 * {@linkcode getProjectBuildProfile} and {@linkcode getProjectOhPackage} methods
 * to get their related information.
 *
 * @see https://developer.huawei.com/consumer/en/doc/harmonyos-guides/ide-hvigor-multi-module
 *
 * ---
 * 一个工作空间包含多个模块。
 *
 * 作为应用/原子服务的最小功能单元，模块包含源代码、资源文件、第三方库和配置文件。
 *
 * 它首先必须包含有工程级的`build-profile.json5` 和`oh-package.json5`文件，因
 * 此在当前模块实现中提供{@linkcode getProjectBuildProfile} 和 {@linkcode getProjectOhPackage}
 * 方法来获取它们的相关信息。
 *
 *
 * @see https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/ide-hvigor-multi-module
 */
export interface Module extends RustModule {
  getUnderlyingModule(): RustModule
  getParsedBuildProfile(): ModuleLevelBuildProfile
  getProject(): Project
}

export namespace Module {
  function fromRustModule(module: RustModule, project: Project): Module {
    return {
      getBuildProfileContent: () => module.getBuildProfileContent(),
      getBuildProfileUri: () => module.getBuildProfileUri(),
      getParsedBuildProfile: () => module.getParsedBuildProfile(),
      getUri: () => module.getUri(),
      getUnderlyingModule: () => module,
      getModuleName: () => module.getModuleName(),
      getProject: () => project,
    }
  }

  /**
   * Find all modules in the current project.
   *
   * @param project - The project to find modules in.
   * @returns A disposable signal that emits an array of modules. When the module file is `created`/`changed`/`deleted`, the signal will be updated.
   * If you want to stop listening the file events, you can call the {@linkcode DisposableSignal.dispose} method.
   */
  export function findAll(project: Project): DisposableSignal<Module[]> {
    const modules = signal(RustModule.findAll(project.getUnderlyingProject()).map(module => fromRustModule(module, project)))
    const handle = createProjectModuleHandler(
      modules,
      uri => RustModule.create(project.getUnderlyingProject(), uri.fsPath),
      module => fromRustModule(module, project),
      module => RustModule.reload(module.getUnderlyingModule()),
    )
    project.getProjectDetector().on('*', handle)
    return DisposableSignal.fromSignal(modules, () => project.getProjectDetector().off('*', handle))
  }
}
