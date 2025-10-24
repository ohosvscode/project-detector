<div align="center" style="margin-top: 24px;">

![ArkTS Project Detector](https://github.com/ohosvscode/project-detector/blob/main/logo.svg?raw=true)

# ArkTS Project Detector

</div>

> 准确的来说，这应该是一个 Hvigor Project Detector. (QWQ)

指定一个基础的工作目录，扫描这个目录下的所有鸿蒙工程。

提供`napi`绑定，用于在 Node.js 中使用。

## 安装 ⏬

```bash
pnpm install @arkts/project-detector
```

现代包管理器（如`pnpm`）会自动下载当前您计算机架构和系统的二进制绑定，无需担心兼容性问题。如果您的 `node_modules` 需要用于打包，则推荐把所有的相关依赖都装上，或者使用 `pnpm install --force` 安装依赖。

## 📦 关于 `bundled` 包

`bundled` 包将node.js层的封装包含 `node_modules` 中的依赖一起打包，可以直接在 `Node.js` / `Bun` 中直接导入使用（可以无需依赖安装），避免了依赖安装不全的问题。导入直接改为这样即可：

```ts
import { ProjectDetector } from '@arkts/project-detector/bundled'
```

## 使用 📝

```ts
import { Module, Project, ProjectDetector } from '@arkts/project-detector'

const projectDetector = ProjectDetector.create(/** 基础工作目录 */)
// 扫描当前工作目录下的所有鸿蒙工程
const projects = Project.findAll(projectDetector)
// 扫描当前鸿蒙工程下的所有模块
const modules = projects[0] && Module.findAll(projects()[0])
// 扫描当前鸿蒙工程下的所有产品
const products = modules[0] && Product.findAll(modules()[0])
// ...
```

更多API请参考声明文件。

## 注意事项 ⚠️

本库使用 `TypeScript` 包裹了一层Rust的API，在Rust的API基础上提供了一个基于 `mitt` + `alien-signals` 的文件事件系统，用于监听文件的创建、修改、删除事件，并即时地动态更新每个对象数据。

您可以使用内置的 `chokidar` 直接监听文件事件，也可以自己调用 `mitt` 的 `emit` 方法处理文件事件。比如在[ohosvscode/arkTS](https://github.com/ohosvscode/arkTS)的语言服务器中，我们使用vscode内置的文件系统监听器，将事件发送到`language-server`进程中并传给了`ProjectDetector`实例。

![Alt](https://repobeats.axiom.co/api/embed/185238ec9e854ad550585f96f1707b5951492026.svg "Repobeats analytics image")

## Contact to Author 📧

- Telegram: [@GCZ_Zero](https://t.me/GCZ_Zero)
- X (Twitter): [@GCZ_Zero](https://x.com/GCZ_Zero)
- QQ: 1203970284，QQ群: 746153004
- WeChat: gcz-zero

## License 📜

MIT
