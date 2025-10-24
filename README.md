<div align="center">

![ArkTS Project Detector](https://github.com/ohosvscode/project-detector/blob/main/logo.svg?raw=true)

# ArkTS Project Detector

</div>

> 准确的来说，这应该是一个 Hvigor Project Detector. (QWQ)

指定一个基础的工作目录，扫描这个目录下的所有鸿蒙工程。

提供`napi`绑定，用于在 Node.js 中使用。

## 关于 `bundled` 包

`bundled` 包将node.js层的封装包含 `node_modules` 中的依赖一起打包，可以直接在 `Node.js` / `Bun` 中直接导入使用（可以无需依赖安装），避免了依赖安装不全的问题。导入直接改为这样即可：

```ts
import { ProjectDetector } from '@arkts/project-detector/bundled'
```
