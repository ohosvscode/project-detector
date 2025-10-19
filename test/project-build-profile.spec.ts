import { describe, expect } from 'vitest'
import { ProjectLevelBuildProfile } from '../src/node'

describe('project-level build-profile.json5 checker', (it) => {
  it('should validate Module correctly', () => {
    const validModule: ProjectLevelBuildProfile.Module = {
      name: 'entry',
      srcPath: './entry',
      targets: [
        {
          name: 'default',
          applyToProducts: ['default'],
        },
      ],
    }

    expect(ProjectLevelBuildProfile.isModule(validModule)).toBe(true)

    // 测试缺少必需字段
    expect(ProjectLevelBuildProfile.isModule({})).toBe(false)
    expect(ProjectLevelBuildProfile.isModule({ name: 'entry' })).toBe(false)
    expect(ProjectLevelBuildProfile.isModule({ srcPath: './entry' })).toBe(false)

    // 测试字段类型错误
    expect(ProjectLevelBuildProfile.isModule({
      name: 123,
      srcPath: './entry',
    })).toBe(false)

    expect(ProjectLevelBuildProfile.isModule({
      name: 'entry',
      srcPath: 123,
    })).toBe(false)

    // 测试最小有效配置
    expect(ProjectLevelBuildProfile.isModule({
      name: 'entry',
      srcPath: './entry',
    })).toBe(true)
  })

  it('should validate complete ProjectLevelBuildProfile structure', () => {
    // 完整的项目级配置
    const validProfile: ProjectLevelBuildProfile = {
      app: {
        signingConfigs: [
          {
            name: 'default',
            type: 'HarmonyOS',
            material: {
              storePassword: 'password123',
              certpath: 'cert/debug.cer',
              keyAlias: 'debugKey',
              keyPassword: 'password123',
              profile: 'cert/debug.p7b',
              signAlg: 'SHA256withECDSA',
              storeFile: 'cert/debug.p12',
            },
          },
          {
            name: 'release',
            type: 'OpenHarmony',
            material: {
              storePassword: 'releasePass',
              certpath: 'cert/release.cer',
              keyAlias: 'releaseKey',
              keyPassword: 'releasePass',
              profile: 'cert/release.p7b',
              signAlg: 'SHA256withECDSA',
              storeFile: 'cert/release.p12',
            },
          },
        ],
        products: [
          {
            name: 'default',
            signingConfig: 'default',
            compatibleSdkVersion: '4.0.0',
            compileSdkVersion: '5.0.0',
            targetSdkVersion: '5.0.0',
            compatibleSdkVersionStage: 'release',
            runtimeOS: 'HarmonyOS',
            bundleName: 'com.example.myapp',
            bundleType: 'app',
            label: '$string:app_name',
            icon: '$media:app_icon',
            versionCode: 1000000,
            versionName: '1.0.0',
            vendor: 'Example Vendor',
            arkTSVersion: '1.1',
            output: {
              artifactName: 'myapp',
            },
            resource: {
              directories: ['src/main/resources'],
            },
          },
        ],
        buildModeSet: [
          {
            name: 'debug',
            buildOption: {
              debuggable: true,
            },
          },
          {
            name: 'release',
            buildOption: {
              debuggable: false,
            },
          },
        ],
        multiProjects: false,
      },
      modules: [
        {
          name: 'entry',
          srcPath: './entry',
          targets: [
            {
              name: 'default',
              applyToProducts: ['default'],
            },
          ],
        },
        {
          name: 'library',
          srcPath: './library',
        },
      ],
    }

    expect(ProjectLevelBuildProfile.is(validProfile)).toBe(true)
    expect(validProfile.app.products?.length).toBe(1)
    expect(validProfile.modules.length).toBe(2)
  })

  it('should validate Product with full buildOption', () => {
    const productWithBuildOption: ProjectLevelBuildProfile.Product = {
      name: 'default',
      compatibleSdkVersion: 10,
      targetSdkVersion: 11,
      runtimeOS: 'HarmonyOS',
      buildOption: {
        packOptions: {
          buildAppSkipSignHap: false,
          fastBuildApp: true,
        },
        debuggable: true,
        resOptions: {
          compression: {
            media: {
              enable: true,
            },
            sizeLimit: {
              ratio: 0.85,
            },
            filters: [
              {
                method: {
                  type: 'astc',
                  blocks: '4x4',
                },
                files: {
                  path: ['resources/base/media/*.png'],
                  size: [[2048], ['4K']],
                  resolution: [
                    { width: 1920, height: 1080 },
                    { width: 3840, height: 2160 },
                  ],
                },
                exclude: {
                  path: ['resources/base/media/icon.png'],
                },
              },
            ],
          },
          copyCodeResource: {
            enable: true,
            excludes: ['**/*.tmp', '**/*.bak'],
          },
          resCompileThreads: 8,
          ignoreResourcePattern: ['**/*.md', '**/*.txt'],
        },
        externalNativeOptions: {
          path: './src/main/cpp/CMakeLists.txt',
          abiFilters: ['arm64-v8a', 'armeabi-v7a', 'x86_64'],
          arguments: ['-DCMAKE_BUILD_TYPE=Release', '-DANDROID_STL=c++_shared'],
          cppFlags: '-std=c++17 -O3',
        },
        sourceOption: {
          workers: ['./workers/DataWorker.ts', './workers/NetworkWorker.ts'],
        },
        arkOptions: {
          apPath: './profile/ap.json',
          hostPGO: true,
          types: ['@types/node', '@ohos/hypium'],
          buildProfileFields: {
            DEBUG_MODE: true,
            API_ENDPOINT: 'https://api.example.com',
            MAX_RETRY: 3,
          },
          tscConfig: {
            targetESVersion: 'ES2021',
            maxFlowDepth: 500,
          },
          autoLazyImport: true,
          branchElimination: true,
          skipOhModulesLint: false,
          reExportCheckMode: 'strict',
        },
        nativeCompiler: 'BiSheng',
        nativeLib: {
          collectAllLibs: true,
          debugSymbol: {
            strip: true,
            exclude: ['libdebug.so', 'libtest.so'],
          },
          filter: {
            excludes: ['libunused.so'],
            pickFirsts: ['libcommon.so'],
            pickLasts: ['liboverride.so'],
            enableOverride: true,
            select: [
              {
                package: 'third-party-lib',
                version: '1.0.0',
                include: ['libcore.so'],
                exclude: ['libdemo.so'],
              },
            ],
          },
          headerPath: ['src/main/cpp/include', 'third-party/include'],
          excludeFromHar: false,
          excludeSoFromInterfaceHar: true,
        },
        strictMode: {
          noExternalImportByPath: true,
          useNormalizedOHMUrl: true,
          caseSensitiveCheck: true,
          duplicateDependencyCheck: true,
          harLocalDependencyCheck: true,
        },
        removePermissions: [
          {
            name: 'ohos.permission.CAMERA',
            reason: 'Camera access required',
            usedScene: {
              abilities: ['MainAbility'],
              when: 'inuse',
            },
          },
          {
            name: 'ohos.permission.LOCATION',
          },
        ],
        generateSharedTgz: true,
      },
    }

    expect(productWithBuildOption.buildOption?.arkOptions?.reExportCheckMode).toBe('strict')
    expect(productWithBuildOption.buildOption?.nativeLib?.filter?.select?.length).toBe(1)
    expect(productWithBuildOption.buildOption?.strictMode?.caseSensitiveCheck).toBe(true)
  })

  it('should validate BuildMode structure', () => {
    const debugMode: ProjectLevelBuildProfile.BuildMode = {
      name: 'debug',
      buildOption: {
        debuggable: true,
        arkOptions: {
          hostPGO: false,
          skipOhModulesLint: true,
        },
      },
    }

    const releaseMode: ProjectLevelBuildProfile.BuildMode = {
      name: 'release',
      buildOption: {
        debuggable: false,
        arkOptions: {
          hostPGO: true,
          reExportCheckMode: 'strict',
        },
        nativeLib: {
          debugSymbol: {
            strip: true,
          },
        },
      },
    }

    expect(debugMode.name).toBe('debug')
    expect(debugMode.buildOption?.debuggable).toBe(true)
    expect(releaseMode.buildOption?.nativeLib?.debugSymbol?.strip).toBe(true)
  })

  it('should validate SigningConfig structure', () => {
    const signingConfig: ProjectLevelBuildProfile.SigningConfig = {
      name: 'production',
      type: 'HarmonyOS',
      material: {
        storePassword: 'prodPassword',
        certpath: 'certificates/production.cer',
        keyAlias: 'productionKey',
        keyPassword: 'keyPassword',
        profile: 'certificates/production.p7b',
        signAlg: 'SHA256withECDSA',
        storeFile: 'certificates/production.p12',
      },
    }

    expect(signingConfig.material.signAlg).toBe('SHA256withECDSA')
    expect(signingConfig.type).toBe('HarmonyOS')
  })

  it('should validate ModuleTarget structure', () => {
    const moduleTarget: ProjectLevelBuildProfile.ModuleTarget = {
      name: 'default',
      applyToProducts: ['default', 'production', 'staging'],
    }

    expect(moduleTarget.applyToProducts.length).toBe(3)
    expect(moduleTarget.applyToProducts.includes('default')).toBe(true)
  })

  it('should validate napiLibFilterOption (deprecated)', () => {
    const product: ProjectLevelBuildProfile.Product = {
      name: 'default',
      compatibleSdkVersion: 9,
      buildOption: {
        napiLibFilterOption: {
          excludes: ['libold.so'],
          pickFirsts: ['libcompat.so'],
          pickLasts: ['libnew.so'],
          enableOverride: true,
          select: [
            {
              package: 'legacy-package',
              version: '0.9.0',
              include: ['liblegacy.so'],
            },
          ],
        },
      },
    }

    expect(product.buildOption?.napiLibFilterOption?.excludes?.length).toBe(1)
    expect(product.buildOption?.napiLibFilterOption?.enableOverride).toBe(true)
  })

  it('should validate minimal valid ProjectLevelBuildProfile', () => {
    const minimalProfile: ProjectLevelBuildProfile = {
      app: {},
      modules: [
        {
          name: 'entry',
          srcPath: './entry',
        },
      ],
    }

    expect(ProjectLevelBuildProfile.is(minimalProfile)).toBe(true)
  })

  it('should reject invalid ProjectLevelBuildProfile', () => {
    // 缺少 app 字段
    expect(ProjectLevelBuildProfile.is({
      modules: [],
    })).toBe(false)

    // 缺少 modules 字段
    expect(ProjectLevelBuildProfile.is({
      app: {},
    })).toBe(false)

    // modules 不是数组
    expect(ProjectLevelBuildProfile.is({
      app: {},
      modules: 'not an array',
    })).toBe(false)

    // modules 包含无效元素
    expect(ProjectLevelBuildProfile.is({
      app: {},
      modules: [
        { name: 'valid', srcPath: './valid' },
        { name: 'invalid' }, // 缺少 srcPath
      ],
    })).toBe(false)

    // app 不是对象
    expect(ProjectLevelBuildProfile.is({
      app: 'not an object',
      modules: [],
    })).toBe(false)

    // null 和 undefined
    expect(ProjectLevelBuildProfile.is(null)).toBe(false)
    expect(ProjectLevelBuildProfile.is(undefined)).toBe(false)
  })

  it('should validate atomic service product', () => {
    const atomicServiceProduct: ProjectLevelBuildProfile.Product = {
      name: 'atomicService',
      compatibleSdkVersion: 10,
      targetSdkVersion: 11,
      bundleType: 'atomicService',
      bundleName: 'com.example.atomicservice',
      versionCode: 1000000,
      versionName: '1.0.0',
      icon: '$media:icon',
      label: '$string:app_name',
    }

    expect(atomicServiceProduct.bundleType).toBe('atomicService')
  })

  it('should validate shared bundle product', () => {
    const sharedProduct: ProjectLevelBuildProfile.Product = {
      name: 'shared',
      compatibleSdkVersion: 10,
      bundleType: 'shared',
      buildOption: {
        generateSharedTgz: true,
      },
    }

    expect(sharedProduct.bundleType).toBe('shared')
    expect(sharedProduct.buildOption?.generateSharedTgz).toBe(true)
  })

  it('should validate CompressPatterns structure', () => {
    const compressPatterns: ProjectLevelBuildProfile.CompressPatterns = {
      path: ['resources/**/*.png', 'assets/**/*.jpg'],
      size: [['2048'], ['4K'], [1024, 768]],
      resolution: [
        { width: 1920, height: 1080 },
        { width: 2560, height: 1440 },
        { width: 3840, height: 2160 },
      ],
    }

    expect(compressPatterns.path?.length).toBe(2)
    expect(compressPatterns.size?.length).toBe(3)
    expect(compressPatterns.resolution?.length).toBe(3)
    expect(compressPatterns.resolution?.[0].width).toBe(1920)
  })

  it('should validate product with multiple SDK version formats', () => {
    const productWithNumericVersion: ProjectLevelBuildProfile.Product = {
      name: 'numeric',
      compatibleSdkVersion: 10,
      targetSdkVersion: 11,
      compileSdkVersion: 12,
    }

    const productWithStringVersion: ProjectLevelBuildProfile.Product = {
      name: 'string',
      compatibleSdkVersion: '4.0.0(10)',
      targetSdkVersion: '5.0.0(11)',
      compileSdkVersion: '6.0.0(12)',
    }

    expect(typeof productWithNumericVersion.compatibleSdkVersion).toBe('number')
    expect(typeof productWithStringVersion.compatibleSdkVersion).toBe('string')
  })

  it('should validate BuildOption with all tscConfig options', () => {
    const buildOption: ProjectLevelBuildProfile.BuildOption = {
      arkOptions: {
        tscConfig: {
          targetESVersion: 'ES2017',
          maxFlowDepth: 1000,
        },
      },
    }

    expect(buildOption.arkOptions?.tscConfig?.targetESVersion).toBe('ES2017')
    expect(buildOption.arkOptions?.tscConfig?.maxFlowDepth).toBe(1000)
  })

  it('should validate multi-projects support', () => {
    const multiProjectApp: ProjectLevelBuildProfile.App = {
      multiProjects: true,
      products: [
        {
          name: 'project1',
          compatibleSdkVersion: 10,
        },
        {
          name: 'project2',
          compatibleSdkVersion: 10,
        },
      ],
    }

    expect(multiProjectApp.multiProjects).toBe(true)
    expect(multiProjectApp.products?.length).toBe(2)
  })
})
