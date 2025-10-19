import { describe, expect } from 'vitest'
import { ModuleLevelBuildProfile } from '../src/node'

describe('build-profile.json5 checker', (it) => {
  it('should validate TargetSourceAbilityFaMode correctly', () => {
    const validAbility: ModuleLevelBuildProfile.TargetSourceAbilityFaMode = {
      name: 'MainAbility',
      pages: ['pages/Index', 'pages/Second'],
      res: ['$media:icon', '$string:app_name'],
      icon: '$media:app_icon',
      label: '$string:app_name',
      launchType: 'singleton',
    }

    expect(ModuleLevelBuildProfile.isTargetSourceAbilityFaMode(validAbility)).toBe(true)

    // 测试缺少必需字段
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityFaMode({})).toBe(false)
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityFaMode({ name: 123 })).toBe(false)

    // 测试无效的 pages 字段
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityFaMode({
      name: 'MainAbility',
      pages: ['valid', 123],
    })).toBe(false)

    // 测试可选字段
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityFaMode({
      name: 'MinimalAbility',
    })).toBe(true)
  })

  it('should validate TargetSourceAbilityStageMode correctly', () => {
    const validAbility: ModuleLevelBuildProfile.TargetSourceAbilityStageMode = {
      name: 'EntryAbility',
      icon: '$media:icon',
      label: '$string:entry_label',
      launchType: 'standard',
    }

    expect(ModuleLevelBuildProfile.isTargetSourceAbilityStageMode(validAbility)).toBe(true)

    // 测试空对象（所有字段都是可选的）
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityStageMode({})).toBe(true)

    // 测试无效类型
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityStageMode(null)).toBe(false)
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityStageMode('string')).toBe(false)
    expect(ModuleLevelBuildProfile.isTargetSourceAbilityStageMode({
      name: 123,
    })).toBe(false)
  })

  it('should validate TargetSourceFaMode correctly', () => {
    const validSource: ModuleLevelBuildProfile.TargetSourceFaMode = {
      abilities: [
        {
          name: 'MainAbility',
          pages: ['pages/Index'],
          icon: '$media:icon',
          label: '$string:app_name',
        },
        {
          name: 'SecondAbility',
          launchType: 'standard',
        },
      ],
    }

    expect(ModuleLevelBuildProfile.isTargetSourceFaMode(validSource)).toBe(true)

    // 测试空对象
    expect(ModuleLevelBuildProfile.isTargetSourceFaMode({})).toBe(true)

    // 测试无效的 abilities 数组
    expect(ModuleLevelBuildProfile.isTargetSourceFaMode({
      abilities: [
        { name: 'Valid' },
        { name: 123 }, // 无效
      ],
    })).toBe(false)
  })

  it('should validate TargetSourceStageMode correctly', () => {
    const validSource: ModuleLevelBuildProfile.TargetSourceStageMode = {
      abilities: [
        {
          name: 'EntryAbility',
          icon: '$media:icon',
          label: '$string:entry_label',
          launchType: 'singleton',
        },
      ],
      pages: ['pages/Index', 'pages/Second'],
      sourceRoots: ['src/main/ets'],
    }

    expect(ModuleLevelBuildProfile.isTargetSourceStageMode(validSource)).toBe(true)

    // 测试空对象
    expect(ModuleLevelBuildProfile.isTargetSourceStageMode({})).toBe(true)

    // 测试无效的 pages 数组
    expect(ModuleLevelBuildProfile.isTargetSourceStageMode({
      pages: ['valid', 123],
    })).toBe(false)

    // 测试无效的 sourceRoots 数组
    expect(ModuleLevelBuildProfile.isTargetSourceStageMode({
      sourceRoots: ['valid', null],
    })).toBe(false)

    // 测试无效的 abilities 数组
    expect(ModuleLevelBuildProfile.isTargetSourceStageMode({
      abilities: [{ icon: 123 }],
    })).toBe(false)
  })

  it('should validate complete ModuleLevelBuildProfile structure', () => {
    // StageMode 完整配置
    const stageModeProfile: ModuleLevelBuildProfile = {
      apiType: 'stageMode',
      showInServiceCenter: true,
      targets: [
        {
          name: 'default',
          runtimeOS: 'HarmonyOS',
          output: {
            artifactName: 'entry',
          },
          config: {
            distributionFilter: {
              apiVersion: {
                policy: 'include',
                value: [9, 10, 11],
              },
              screenShape: {
                policy: 'exclude',
                value: ['circle'],
              },
              screenDensity: {
                policy: 'include',
                value: ['mdpi', 'xldpi', 'xxldpi'],
              },
            },
            deviceType: ['phone', 'tablet', 'wearable'],
            atomicService: {
              preloads: [
                { moduleName: 'feature1' },
                { moduleName: 'feature2' },
              ],
            },
          },
          source: {
            abilities: [
              {
                name: 'EntryAbility',
                icon: '$media:app_icon',
                label: '$string:app_name',
                launchType: 'singleton',
              },
            ],
            pages: ['pages/Index', 'pages/Detail'],
            sourceRoots: ['src/main/ets'],
          },
          resource: {
            directories: ['src/main/resources'],
          },
        },
      ],
      buildOption: {
        arkOptions: {
          apPath: './apfile.json',
          hostPGO: true,
          types: ['@types/node'],
          obfuscation: {
            ruleOptions: {
              enable: true,
              files: ['obfuscation-rules.txt'],
            },
            consumerFiles: ['consumer-rules.txt'],
          },
          integratedHsp: true,
          branchElimination: false,
          autoLazyImport: true,
          skipOhModulesLint: false,
          reExportCheckMode: 'compatible',
        },
        resOptions: {
          compression: {
            media: {
              enable: true,
            },
            filters: [
              {
                method: {
                  type: 'astc',
                  blocks: '4x4',
                },
                files: {
                  path: ['src/main/resources/base/media'],
                  size: [[2048], ['4K']],
                },
              },
            ],
            sizeLimit: {
              ratio: 0.8,
            },
          },
          resCompileThreads: 4,
        },
        externalNativeOptions: {
          path: 'src/main/cpp/CMakeLists.txt',
          abiFilters: ['arm64-v8a', 'armeabi-v7a'],
          arguments: '-DCMAKE_BUILD_TYPE=Release',
          cppFlags: '-std=c++17',
          cFlags: '-O2',
          targets: ['hello'],
        },
        nativeLib: {
          collectAllLibs: true,
          debugSymbol: {
            strip: true,
            exclude: ['libtest.so'],
          },
          filter: {
            excludes: ['libexclude.so'],
            pickFirsts: ['libfirst.so'],
            enableOverride: true,
          },
          headerPath: 'src/main/cpp/include',
        },
      },
      buildOptionSet: [
        {
          name: 'release',
          debuggable: false,
          arkOptions: {
            hostPGO: true,
            reExportCheckMode: 'strict',
          },
        },
      ],
      buildModeBinder: [
        {
          buildModeName: 'release',
          mappings: [
            {
              targetName: 'default',
              buildOptionName: 'release',
            },
          ],
        },
      ],
      entryModules: ['entry'],
    }

    expect(stageModeProfile.apiType).toBe('stageMode')
    expect(stageModeProfile.targets?.length).toBe(1)
    expect(stageModeProfile.buildOption?.arkOptions?.reExportCheckMode).toBe('compatible')

    // FaMode 完整配置
    const faModeProfile: ModuleLevelBuildProfile = {
      apiType: 'faMode',
      targets: [
        {
          name: 'default',
          runtimeOS: 'OpenHarmony',
          config: {
            distroFilter: {
              apiVersion: {
                policy: 'include',
                value: [8, 9],
              },
              countryCode: {
                policy: 'include',
                value: ['CN', 'US'],
              },
            },
            deviceType: ['phone', 'tablet'],
          },
          source: {
            abilities: [
              {
                name: 'MainAbility',
                pages: ['pages/index/index'],
                res: ['$media:icon'],
                icon: '$media:app_icon',
                label: '$string:app_name',
                launchType: 'singleton',
              },
            ],
          },
        },
      ],
      buildOption: {
        sourceOption: {
          workers: ['./workers/Worker.ts'],
        },
        removePermissions: [
          {
            name: 'ohos.permission.LOCATION',
            reason: 'Location access',
            usedScene: {
              abilities: ['MainAbility'],
              when: 'inuse',
            },
          },
        ],
      },
    }

    expect(faModeProfile.apiType).toBe('faMode')
    expect(faModeProfile.targets?.[0].runtimeOS).toBe('OpenHarmony')
    expect(faModeProfile.buildOption?.removePermissions?.length).toBe(1)
  })
})
