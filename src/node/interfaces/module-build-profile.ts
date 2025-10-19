export interface ModuleLevelBuildProfile {
  apiType?: ModuleLevelBuildProfile.ApiType
  targets?: ModuleLevelBuildProfile.Target[]
  showInServiceCenter?: boolean
  buildOption?: ModuleLevelBuildProfile.BuildOption
  buildOptionSet?: ModuleLevelBuildProfile.BuildOptionSetItem[]
  buildModeBinder?: ModuleLevelBuildProfile.BuildModeBinderItem[]
  entryModules?: string[]
}

export namespace ModuleLevelBuildProfile {
  export type ApiType = 'faMode' | 'stageMode'

  export interface TargetOutput {
    artifactName: string
  }

  export type RuntimeOS = 'HarmonyOS' | 'OpenHarmony'

  export interface TargetConfigCommonFiltersPolicy<T> {
    policy: 'include' | 'exclude'
    value: T[]
  }

  export interface TargetConfigDistroFilter {
    apiVersion?: TargetConfigCommonFiltersPolicy<number>
    screenShape?: TargetConfigCommonFiltersPolicy<'circle' | 'rect'>
    screenWindow?: {
      policy: 'include'
      value: string[] // e.g. "454*454"
    }
    screenDensity?: TargetConfigCommonFiltersPolicy<
      'sdpi' | 'mdpi' | 'ldpi' | 'xldpi' | 'xxldpi' | 'xxxldpi'
    >
    countryCode?: TargetConfigCommonFiltersPolicy<string>
  }

  export interface TargetAtomicServicePreloadItem {
    moduleName?: string
  }

  export interface TargetAtomicService {
    preloads?: TargetAtomicServicePreloadItem[]
  }

  export interface TargetConfigStageMode {
    distributionFilter?: TargetConfigDistroFilter
    deviceType?: string[]
    buildOption?: BuildOption
    atomicService?: TargetAtomicService
  }

  export interface TargetConfigFaMode {
    distroFilter?: TargetConfigDistroFilter
    deviceType?: string[]
    buildOption?: BuildOption
  }

  export interface TargetSourceAbilityFaMode {
    name: string
    pages?: string[]
    res?: string[]
    icon?: string // "$media:..." or resource pattern
    label?: string // "$string:..." or resource pattern
    launchType?: 'standard' | 'singleton' | 'specified' | 'multiton'
  }

  export function isTargetSourceAbilityFaMode(value: unknown): value is TargetSourceAbilityFaMode {
    return typeof value === 'object'
      && value !== null
      && 'name' in value
      && typeof value.name === 'string'
      && ('pages' in value ? Array.isArray(value.pages) && value.pages.every(page => typeof page === 'string') : true)
      && ('res' in value ? Array.isArray(value.res) && value.res.every(res => typeof res === 'string') : true)
      && ('icon' in value ? typeof value.icon === 'string' : true)
      && ('label' in value ? typeof value.label === 'string' : true)
      && ('launchType' in value ? typeof value.launchType === 'string' : true)
  }

  export interface TargetSourceFaMode {
    abilities?: TargetSourceAbilityFaMode[]
  }

  export interface TargetSourceAbilityStageMode {
    name?: string
    icon?: string
    label?: string
    launchType?: 'standard' | 'singleton' | 'specified' | 'multiton'
  }

  export function isTargetSourceAbilityStageMode(value: unknown): value is TargetSourceAbilityStageMode {
    return typeof value === 'object'
      && value !== null
      && ('name' in value ? typeof value.name === 'string' : true)
      && ('icon' in value ? typeof value.icon === 'string' : true)
      && ('label' in value ? typeof value.label === 'string' : true)
      && ('launchType' in value ? typeof value.launchType === 'string' : true)
  }

  export interface TargetSourceStageMode {
    abilities?: TargetSourceAbilityStageMode[]
    pages?: string[]
    sourceRoots?: string[]
  }

  export interface TargetResource {
    directories?: string[]
  }

  export interface Target {
    name: string
    runtimeOS?: RuntimeOS
    output?: TargetOutput
    config?: TargetConfigFaMode | TargetConfigStageMode
    source?: TargetSourceFaMode | TargetSourceStageMode
    resource?: TargetResource
  }

  export function isTargetSourceFaMode(value: unknown): value is TargetSourceFaMode {
    return typeof value === 'object'
      && value !== null
      && ('abilities' in value ? Array.isArray(value.abilities) && value.abilities.every(ability => isTargetSourceAbilityFaMode(ability)) : true)
  }

  export function isTargetSourceStageMode(value: unknown): value is TargetSourceStageMode {
    return typeof value === 'object'
      && value !== null
      && ('abilities' in value ? Array.isArray(value.abilities) && value.abilities.every(ability => isTargetSourceAbilityStageMode(ability)) : true)
      && ('pages' in value ? Array.isArray(value.pages) && value.pages.every(page => typeof page === 'string') : true)
      && ('sourceRoots' in value ? Array.isArray(value.sourceRoots) && value.sourceRoots.every(root => typeof root === 'string') : true)
  }

  export interface CompressPatternsSizeItemInner {
    width?: number
    height?: number
  }

  export interface CompressPatterns {
    path?: string[]
    size?: (number | string)[][] // e.g. [[2048], ["4K"]]
    resolution?: CompressPatternsSizeItemInner[][]
  }

  export interface ResOptionsCompressionMethod {
    type: 'astc' | 'sut'
    blocks: '4x4'
  }

  export interface ResOptionsCompressionFilter {
    method: ResOptionsCompressionMethod
    files?: CompressPatterns
    exclude?: CompressPatterns
  }

  export interface ResOptionsCompressionMedia {
    enable?: boolean
  }

  export interface ResOptionsCompressionSizeLimit {
    ratio?: number // 0 means no limit
  }

  export interface ResOptionsCompression {
    media?: ResOptionsCompressionMedia
    filters?: ResOptionsCompressionFilter[]
    sizeLimit?: ResOptionsCompressionSizeLimit
  }

  export interface ResOptionsCopyCodeResource {
    enable?: boolean
    excludes?: string[]
  }

  export interface ResOptions {
    compression?: ResOptionsCompression
    copyCodeResource?: ResOptionsCopyCodeResource
    resCompileThreads?: number // >= 1
    ignoreResourcePattern?: string[]
  }

  export interface ExternalNativeOptions {
    path?: string
    abiFilters?: Array<'arm64-v8a' | 'armeabi-v7a' | 'x86_64'>
    arguments?: string | string[]
    cppFlags?: string
    cFlags?: string
    targets?: string[]
  }

  export interface SourceOption {
    workers?: string[] // must be relative path pattern ./ or ../
  }

  export interface NapiLibFilterOption {
    excludes?: string[]
    pickFirsts?: string[]
    pickLasts?: string[]
    enableOverride?: boolean
  }

  export interface NativeLibFilterSelectItem {
    package?: string
    version?: string
    include?: string[]
    exclude?: string[]
  }

  export interface NativeLibFilterOption {
    excludes?: string[]
    pickFirsts?: string[]
    pickLasts?: string[]
    enableOverride?: boolean
    select?: NativeLibFilterSelectItem[]
  }

  export interface NativeLibDebugSymbolOption {
    strip?: boolean
    exclude?: string[]
  }

  export interface NativeLibLibrariesInfoItem {
    name: string
    linkLibraries: string[]
  }

  export interface NativeLibOption {
    collectAllLibs?: boolean
    debugSymbol?: NativeLibDebugSymbolOption
    filter?: NativeLibFilterOption
    headerPath?: string | string[]
    librariesInfo?: NativeLibLibrariesInfoItem[]
    excludeSoFromInterfaceHar?: boolean
  }

  export interface ArkObfuscationRuleOptions {
    enable: boolean
    files?: string | string[]
  }

  export interface ArkObfuscationOption {
    ruleOptions?: ArkObfuscationRuleOptions
    consumerFiles?: string | string[]
  }

  export interface ArkRuntimeOnlyOption {
    sources?: string[] // relative path strings
    packages?: string[] // package names or filenames
  }

  export type ReExportCheckMode = 'noCheck' | 'compatible' | 'strict'

  export interface ArkOptions {
    apPath?: string
    hostPGO?: boolean
    types?: string[]
    obfuscation?: ArkObfuscationOption
    buildProfileFields?: Record<string, string | number | boolean>
    runtimeOnly?: ArkRuntimeOnlyOption
    integratedHsp?: boolean
    branchElimination?: boolean
    transformLib?: string
    autoLazyImport?: boolean
    skipOhModulesLint?: boolean
    reExportCheckMode?: ReExportCheckMode
  }

  export interface BuildOption {
    resOptions?: ResOptions
    externalNativeOptions?: ExternalNativeOptions
    sourceOption?: SourceOption
    napiLibFilterOption?: NapiLibFilterOption
    arkOptions?: ArkOptions
    nativeLib?: NativeLibOption
    removePermissions?: RemovePermissionItem[]
    generateSharedTgz?: boolean
  }

  export interface RemovePermissionUsedScene {
    abilities?: string[] // deprecated
    when?: 'inuse' | 'always' // deprecated
  }

  export interface RemovePermissionItem {
    name: string
    reason?: string // deprecated
    usedScene?: RemovePermissionUsedScene // deprecated
  }

  export interface BuildOptionSetItem {
    name?: string
    debuggable?: boolean
    copyFrom?: string
    resOptions?: ResOptions
    externalNativeOptions?: ExternalNativeOptions
    sourceOption?: SourceOption
    removePermissions?: RemovePermissionItem[]
    napiLibFilterOption?: NapiLibFilterOption
    arkOptions?: ArkOptions
    nativeLib?: NativeLibOption
    generateSharedTgz?: boolean
  }

  export interface BuildModeBinderMappingItem {
    targetName?: string
    buildOptionName?: string
  }

  export interface BuildModeBinderItem {
    buildModeName?: string
    mappings?: BuildModeBinderMappingItem[]
  }
}
