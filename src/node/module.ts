import type { Project } from './project'
import type { Signal } from './types'
import { signal } from 'alien-signals'
import { Module as RustModule } from '../../index'

export interface Module extends RustModule {
  getUnderlyingModule(): RustModule
}

export namespace Module {
  function boundRustModule(module: RustModule): Module {
    return {
      getBuildProfileContent: () => module.getBuildProfileContent(),
      getBuildProfileUri: () => module.getBuildProfileUri(),
      getParsedBuildProfile: () => module.getParsedBuildProfile(),
      getUri: () => module.getUri(),
      getUnderlyingModule: () => module,
      getModuleName: () => module.getModuleName(),
      getProject: () => module.getProject(),
    }
  }

  export function findAll(project: Project): Signal<Module[]> {
    const modules: Signal<Module[]> = signal(RustModule.findAll(project.getUnderlyingProject()).map(boundRustModule))
    return modules
  }
}
