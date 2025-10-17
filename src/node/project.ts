import type { ProjectDetector } from './project-detector'
import { signal } from 'alien-signals'
import { Project as RustProject } from '../../index'
import { createProjectModuleHandler } from './common'
import { DisposableSignal } from './types'

/**
 * {@linkcode Project} represents a `hvigor` project.
 *
 * Hvigor is a build task orchestration tool based on TypeScript, which mainly
 * provides task management mechanisms, including task registration orchestration,
 * project model management, configuration management, and provides specific
 * processes and configurable settings for building and testing applications.
 *
 * @see https://developer.huawei.com/consumer/en/doc/harmonyos-guides/ide-hvigor
 *
 * ---
 *
 * {@linkcode Project} 代表一个`hvigor`工程。
 *
 * 编译构建工具 Hvigor 是一款基于TypeScript实现的构建任务编排工具，主要提供任务
 * 管理机制，包括任务注册编排、工程模型管理、配置管理等关键能力，提供专用于构建
 * 和测试应用的流程和可配置设置。
 *
 * @see https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/ide-hvigor
 */
export interface Project extends RustProject {
  getUnderlyingProject(): RustProject
  getProjectDetector(): ProjectDetector
}

export namespace Project {
  function fromRustProject(project: RustProject, projectDetector: ProjectDetector): Project {
    return {
      getBuildProfileContent: () => project.getBuildProfileContent(),
      getBuildProfileUri: () => project.getBuildProfileUri(),
      getParsedBuildProfile: () => project.getParsedBuildProfile(),
      getUri: () => project.getUri(),
      getUnderlyingProject: () => project,
      getProjectDetector: () => projectDetector,
    }
  }

  /**
   * Find all projects in the current project detector.
   *
   * @param projectDetector - The project detector to find projects in.
   * @returns A disposable signal that emits an array of projects. When the project file is `created`/`changed`/`deleted`, the signal will be updated.
   * If you want to stop listening the file events, you can call the {@linkcode DisposableSignal.dispose} method.
   */
  export function findAll(projectDetector: ProjectDetector): DisposableSignal<Project[]> {
    const underlyingProjectDetector = projectDetector.getUnderlyingProjectDetector()
    const projects = signal<Project[]>(RustProject.findAll(underlyingProjectDetector).map(project => fromRustProject(project, projectDetector)))
    const handler = createProjectModuleHandler(
      projects,
      uri => RustProject.create(underlyingProjectDetector, uri.fsPath),
      project => fromRustProject(project, projectDetector),
    )
    projectDetector.on('*', handler)
    return DisposableSignal.fromSignal(projects, () => projectDetector.off('*', handler))
  }
}
