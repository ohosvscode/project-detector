import type { FSWatcher } from 'chokidar'
import type { ProjectDetector } from './project-detector'
import { Uri } from '../../index'

export namespace Watcher {
  export async function fromChokidar(projectDetector: ProjectDetector, watcher?: FSWatcher): Promise<FSWatcher> {
    if (!watcher) {
      const { watch } = await import('chokidar')
      watcher = watch([projectDetector.getWorkspaceFolder().fsPath])
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
