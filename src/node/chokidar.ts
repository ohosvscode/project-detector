import type { FSWatcher } from 'chokidar'
import type { ProjectDetector } from './project-detector'
import { watch } from 'chokidar'
import { Uri } from '../../index'

export namespace Chokidar {
  export async function create(projectDetector: ProjectDetector, watcher: FSWatcher = watch([projectDetector.getWorkspaceFolder().fsPath])): Promise<FSWatcher> {
    watcher.on('change', path => projectDetector.emit('file-changed', Uri.file(path)))
    watcher.on('unlink', path => projectDetector.emit('file-deleted', Uri.file(path)))
    watcher.on('add', path => projectDetector.emit('file-created', Uri.file(path)))
    await new Promise(resolve => watcher.on('ready', () => resolve(void 0)))
    return watcher
  }
}
