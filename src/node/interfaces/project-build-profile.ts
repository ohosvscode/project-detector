/**
 * OpenHarmony project-level build-profile configuration
 * This document describes the configuration details of the openHarmony project-level configuration file build-profile.json5.
 */
export interface ProjectLevelBuildProfile {
  /** OpenHarmony application project-level configuration information, which is shared by all FA/PA modules, including the compilation API level, signature, and product information. */
  app: ProjectLevelBuildProfile.App
  /** Describes all modules in the openHarmony application. The module configuration includes the name, path, and target-product association configuration. */
  modules: ProjectLevelBuildProfile.Module[]
}

export namespace ProjectLevelBuildProfile {
  export function is(value: unknown): value is ProjectLevelBuildProfile {
    return typeof value === 'object'
      && value !== null
      && 'app' in value
      && typeof value.app === 'object'
      && 'modules' in value
      && Array.isArray(value.modules)
      && value.modules.every(mod => ProjectLevelBuildProfile.isModule(mod))
  }
}

export namespace ProjectLevelBuildProfile {
  /**
   * OpenHarmony project-level build-profile configuration
   * This document describes the configuration details of the openHarmony project-level configuration file build-profile.json5.
   */

  /**
   * Declare the files for compressing or not. items are ORed relationship
   */
  export interface CompressPatterns {
  /** example: "/*.png". all items are ORed relationship */
    path?: string[]
    /** example: `["2048","4K"]`. */
    size?: (string | number)[][]
    /** example: `[{"width":1024, "height":768}]`. */
    resolution?: Array<{
      width: number
      height: number
    }>
  }

  /**
   * Configure the related configurations used by the project during the build process.
   */
  export interface BuildOption {
  /** Package configuration options, which are used to limit the package size and number of packages. */
    packOptions?: {
    /** The build APP does not generate a signed HAP package. */
      buildAppSkipSignHap?: boolean
      /** Optimize the build time for HAP/HSP packaging. */
      fastBuildApp?: boolean
    }
    /** Indicates the permissions to remove. */
    removePermissions?: Array<{
    /** Specifies the permission name to be used. */
      name: string
      /** @deprecated This property is not necessary here. */
      reason?: string
      /** @deprecated This property is not necessary here. */
      usedScene?: {
      /** This tag identifies the abilities that need the permission. */
        abilities?: string[]
        /** This tag identifies the time when the permission is used. */
        when?: 'inuse' | 'always'
      }
    }>
    /** Debuggable configuration. */
    debuggable?: boolean
    /** Configurations related to resource compilation */
    resOptions?: {
    /** Configurations related to resources compression. */
      compression?: {
      /** Configurations related to media compression */
        media?: {
        /** Enable the media compressing */
          enable?: boolean
        }
        /** Configurations related to total size limitation */
        sizeLimit?: {
        /** Ratio of total size limitation. 0 for no limit */
          ratio?: number
        }
        /** Declare the methods to compression. */
        filters?: Array<{
        /** Declare the methods to compression. */
          method: {
            type: 'astc' | 'sut'
            blocks: '4x4'
          }
          files?: CompressPatterns
          exclude?: CompressPatterns
        }>
      }
      /** Configurations related to copying static resource files in the ets directory. */
      copyCodeResource?: {
      /** Whether to copy and pack static resource files in the ets directory. The default value is true. */
        enable?: boolean
        /** Filters defined with glob patterns. The setting does not take effect when enable is set to false */
        excludes?: string[]
      }
      /** Number of resource compilation threads */
      resCompileThreads?: number
      /** Filters defined with glob patterns. Files in resources matching any of these patterns will not be packaged. */
      ignoreResourcePattern?: string[]
    }
    /** Configurations related to native compilation. */
    externalNativeOptions?: {
    /** Path of the CMakeLists.txt file, for example, ./src/main/cpp/CMakeLists.txt or D:/CMakeLists.txt. */
      path?: string
      /** Defines the CPU architecture type of the system where the CPP application runs. Multiple CPU architecture types can be included, for example, Arm64-v8a. */
      abiFilters?: ('arm64-v8a' | 'armeabi-v7a' | 'x86_64')[]
      /** Specifies cmake compilation parameters, for example, -v -DOHOS_STL=c++_static */
      arguments?: string | string[]
      /** Specifies the settings related to CMAKE_CXX_FLAGS. */
      cppFlags?: string
    }
    /** Different tags are used to classify source codes so that different source codes can be processed differently during the build process. */
    sourceOption?: {
    /** Specifies the JS/TS source code that uses node.js worker, The source code is processed separately during the build process. */
      workers?: string[]
    }
    /** @deprecated Please use 'nativeLib/filter' instead in API10 or later. */
    napiLibFilterOption?: {
    /** Set of excluded patterns. Libraries matching any of these patterns will not be packaged. */
      excludes?: string[]
      /** Set of patterns where the first occurrence is packaged into HAP/HSP. For each libraries matched any of these pattern, only the first one will be packaged. */
      pickFirsts?: string[]
      /** Set of patterns where the last occurrence is packaged into HAP/HSP. For each libraries matched any of these pattern, only the last one will be packaged. */
      pickLasts?: string[]
      /** Enable the override of native library. Libraries with the same path will be allowed. */
      enableOverride?: boolean
      select?: Array<{
        package?: string
        version?: string
        include?: string[]
        exclude?: string[]
      }>
    }
    /** Configurations related to ark compilation. */
    arkOptions?: {
    /** Application hotspot information file */
      apPath?: string
      /** Enable profile guided optimization ability. Support in API10+. */
      hostPGO?: boolean
      /** Configurations related to custom types */
      types?: string[]
      /** build profile used for ArkTS. */
      buildProfileFields?: Record<string, string | number | boolean>
      /** Configurations related to tsconfig.json */
      tscConfig?: {
        targetESVersion?: 'ES2017' | 'ES2021'
        maxFlowDepth?: number
      }
      /** Enable the import dependency to have the automatic lazy loading capability. */
      autoLazyImport?: boolean
      /** Enable branchElimination. */
      branchElimination?: boolean
      /** Specifies whether to skip the ArkTS linter check for the oh_modules directory in the project. */
      skipOhModulesLint?: boolean
      /** Specify the checking mode of re-export. */
      reExportCheckMode?: 'noCheck' | 'compatible' | 'strict'
    }
    /** Used to configure the native compiler */
    nativeCompiler?: 'Original' | 'BiSheng'
    /** Native lib options */
    nativeLib?: {
    /** Whether to lift the restriction that only files with the .so extension are collected under the libs directory. */
      collectAllLibs?: boolean
      /** debugSymbol option */
      debugSymbol?: {
      /** Specifies whether to strip .so files. */
        strip?: boolean
        /** Lists the .so files to be excluded from strip. If strip is set to true, the matched .so files in the list are not stripped. If strip is set to false, only the matched .so files in the list are stripped. */
        exclude?: string[]
      }
      /** Native libs filter option */
      filter?: {
      /** Set of excluded patterns. Libraries matching any of these patterns will not be packaged. */
        excludes?: string[]
        /** Set of patterns where the first occurrence is packaged into HAP/HSP. For each libraries matched any of these pattern, only the first one will be packaged. */
        pickFirsts?: string[]
        /** Set of patterns where the last occurrence is packaged into HAP/HSP. For each libraries matched any of these pattern, only the last one will be packaged. */
        pickLasts?: string[]
        /** Enable the override of native library. Libraries with the same path will be allowed. */
        enableOverride?: boolean
        select?: Array<{
          package?: string
          version?: string
          include?: string[]
          exclude?: string[]
        }>
      }
      /** Path to a directory containing headers to export to dependents of this module. */
      headerPath?: string | string[]
      /** Indicates whether to exclude so in the har dependency. The default value is true. */
      excludeFromHar?: boolean
      /** While true, the .so file won't be included in the interface har when packaging the HSP; this only applies to the HSP package. */
      excludeSoFromInterfaceHar?: boolean
    }
    /** Defines the strict check mode. */
    strictMode?: {
    /** Specifies whether to prevent file imports using absolute paths and file imports outside of the current module using relative paths. The value true means to prevent such imports. */
      noExternalImportByPath?: boolean
      /** Whether to use the new ohmurl format. */
      useNormalizedOHMUrl?: boolean
      /** enable caseSensitive. */
      caseSensitiveCheck?: boolean
      /** Specifies whether to check duplicated har dependencies between hsp during building app. */
      duplicateDependencyCheck?: boolean
      /** Specifies whether to check for local dependencies outside the hsp and har dependent modules during application build. */
      harLocalDependencyCheck?: boolean
    }
    /** Determine to generate shared tgz while compiling HSP. */
    generateSharedTgz?: boolean
  }

  /**
   * This field contains the signature materials used for modifying the openHarmony application. Multiple signature materials can be configured.
   */
  export interface SigningConfig {
  /** This field contains the signature materials used for modifying the openHarmony application. Multiple signature materials can be configured. */
    material: {
      storePassword: string
      certpath: string
      keyAlias: string
      keyPassword: string
      profile: string
      signAlg: 'SHA256withECDSA'
      storeFile: string
    }
    /** Defines the product type name of the openHarmony application. By default, there is a product named default. */
    name: string
    type?: 'HarmonyOS' | 'OpenHarmony'
  }

  /**
   * This field is used to describe different product types defined by the openHarmony application. By default, a default product exists and different signature materials can be specified.
   */
  export interface Product {
  /** Defines the product type name of the openHarmony application. By default, there is a product named default. */
    name: string
    /** Specifies the signature material used by the product. The current signature material must be the material declared in app/signingConfigs. You can select the signature material based on its name. */
    signingConfig?: string
    /** Indicates the bundle name of the application. It uniquely identifies the application. */
    bundleName?: string
    /** Configure the related configurations used by the project during the build process. */
    buildOption?: BuildOption
    /** The type of runtimeOS specified by the product */
    runtimeOS?: 'HarmonyOS' | 'OpenHarmony'
    /** Specifies the SDK API version when the OpenHarmony application is compiled. This value determines the value of apiTargetVersion in the HAP package. */
    compileSdkVersion?: number | string
    /** Specifies the lowest SDK API level version compatible with the OpenHarmony application during compilation. This value determines the value of apiCompatibleVersion in the HAP package. */
    compatibleSdkVersion: number | string
    /** Specifies the abc compiler version compatible with the OpenHarmony application during compilation. If compatibleSdkVersion is set to a version in API version 12, the default value is "beta1." This field is applicable only to API version 12. */
    compatibleSdkVersionStage?: 'beta1' | 'beta2' | 'beta3' | 'beta5' | 'beta6' | 'release'
    /** Specifies the SDK API version when the OpenHarmony application is targeted. This value determines the value of apiCompatibleVersion in the HAP package. */
    targetSdkVersion?: number | string
    /** Defines the type of bundle. */
    bundleType?: 'app' | 'atomicService' | 'shared'
    /** Defines the label of the application. */
    label?: string
    /** Defines the index to the application icon file, in the format of "$media:application_icon".This label can be left blank by default. */
    icon?: string
    /** Defines the versionCode number of the application. The value is an integer greater than 0. A larger value generally represents a later version.The system determines the application version based on the tag value.This label cannot be left blank. */
    versionCode?: number
    /** Defines the text description of the application version.Used for displaying to users.A string can contain a maximum of 127 bytes.This label cannot be left blank. */
    versionName?: string
    /** Defines the resource of the application. */
    resource?: {
    /** Defines the directories of the resources */
      directories: string[]
    }
    /** Customize the configuration of the application package generated by the product. */
    output?: {
    /** Customize the name of the application package generated by the product. */
      artifactName: string
    }
    /** ArkTS Language Version Configuration. */
    arkTSVersion?: '1.0' | '1.1'
    /** Defines the vendor of the different products.Used for displaying to users.A string can contain a maximum of 127 bytes.This label can be left blank by default. */
    vendor?: string
  }

  /**
   * Set of build mode, each of which refers to a solution for using different build configurations when executing different target tasks. In default configurations, debug mode used in packageHap and release mode used in packageApp.
   */
  export interface BuildMode {
  /** BuildOption name. */
    name: string
    /** Configure the related configurations used by the project during the build process. */
    buildOption?: BuildOption
  }

  /**
   * OpenHarmony application project-level configuration information, which is shared by all FA/PA modules, including the compilation API level, signature, and product information.
   */
  export interface App {
  /** This field contains the signature materials used for modifying the openHarmony application. Multiple signature materials can be configured. */
    signingConfigs?: SigningConfig[]
    /** This field is used to describe different product types defined by the openHarmony application. By default, a default product exists and different signature materials can be specified. */
    products?: Product[]
    /** Set of build mode, each of which refers to a solution for using different build configurations when executing different target tasks. In default configurations, debug mode used in packageHap and release mode used in packageApp. */
    buildModeSet?: BuildMode[]
    /** Indicates whether current project supports multiple project. */
    multiProjects?: boolean
  }

  /**
   * Configure the association between the target of the module and the product of the application.
   */
  export interface ModuleTarget {
  /** Name of the module target. The target name must be in the target configuration in build-profile.json5 of the module-level. */
    name: string
    /** Describes which products the target is used for, which means that different products can contain different targets. */
    applyToProducts: string[]
  }

  /**
   * Describes all modules in the openHarmony application. The module configuration includes the name, path, and target-product association configuration.
   */
  export interface Module {
  /** This section describes the source code path of the module. It is the relative path of the current configuration file. The path must contain configurations related to hvigorfile.ts/js and package.json. */
    srcPath: string
    /** Logical name of a module. The value must be the same as that of moduleName in the config.json/module.json file. */
    name: string
    /** Configure the association between the target of the module and the product of the application. */
    targets?: ModuleTarget[]
  }

  export function isModule(value: unknown): value is Module {
    return typeof value === 'object'
      && value !== null
      && 'srcPath' in value
      && typeof value.srcPath === 'string'
      && 'name' in value
      && typeof value.name === 'string'
  }
}
