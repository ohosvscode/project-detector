import {
  createOnMessage as __wasmCreateOnMessageForFsProxy,
  getDefaultContext as __emnapiGetDefaultContext,
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'



const __wasi = new __WASI({
  version: 'preview1',
})

const __wasmUrl = new URL('./project-detector.wasm32-wasi.wasm', import.meta.url).href
const __emnapiContext = __emnapiGetDefaultContext()


const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    for (const name of Object.keys(instance.exports)) {
      if (name.startsWith('__napi_register__')) {
        instance.exports[name]()
      }
    }
  },
})
export default __napiModule.exports
export const ElementDirectory = __napiModule.exports.ElementDirectory
export const ElementJsonFile = __napiModule.exports.ElementJsonFile
export const ElementJsonFileReference = __napiModule.exports.ElementJsonFileReference
export const MediaDirectory = __napiModule.exports.MediaDirectory
export const Module = __napiModule.exports.Module
export const Product = __napiModule.exports.Product
export const Project = __napiModule.exports.Project
export const ProjectDetector = __napiModule.exports.ProjectDetector
export const QualifierUtils = __napiModule.exports.QualifierUtils
export const RawfileDirectory = __napiModule.exports.RawfileDirectory
export const ResfileDirectory = __napiModule.exports.ResfileDirectory
export const Resource = __napiModule.exports.Resource
export const ResourceDirectory = __napiModule.exports.ResourceDirectory
export const Uri = __napiModule.exports.Uri
export const QualifierType = __napiModule.exports.QualifierType
