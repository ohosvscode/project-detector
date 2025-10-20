import type { FSWatcher } from 'chokidar'
import type { ProjectDetector } from './project-detector'
import { Uri } from '../../index'

export namespace Watcher {
  export async function fromChokidar(projectDetector: ProjectDetector, watcher?: FSWatcher): Promise<FSWatcher> {
    if (!watcher) {
      const { watch } = await import('chokidar')
      const process = await import('node:process')
      watcher = watch([projectDetector.getWorkspaceFolder().fsPath], {
        // 忽略初始添加事件，避免启动时的大量事件
        ignoreInitial: true,
        // 等待文件写入完成，避免在文件写入过程中触发事件
        awaitWriteFinish: {
          stabilityThreshold: 100,
          pollInterval: 50,
        },
        // 在 Windows 上使用轮询可以避免文件锁定问题
        // 在 CI 环境中启用轮询
        usePolling: process.env.CI === 'true',
        // 轮询间隔
        interval: 100,
      })
    }
    watcher.on('change', path => projectDetector.emit('file-changed', Uri.file(path)))
    watcher.on('unlink', path => projectDetector.emit('file-deleted', Uri.file(path)))
    watcher.on('add', path => projectDetector.emit('file-created', Uri.file(path)))
    watcher.on('addDir', path => projectDetector.emit('file-created', Uri.file(path)))
    watcher.on('unlinkDir', path => projectDetector.emit('file-deleted', Uri.file(path)))
    await new Promise<void>(resolve => watcher.on('ready', resolve))
    return watcher
  }
}
