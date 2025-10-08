import { loadNativeBinding } from './load'

export * from './load'

// 加载并导出所有原生绑定
const nativeBinding = loadNativeBinding<typeof import('../dist/index')>()

export default nativeBinding
