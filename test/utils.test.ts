import { describe, expect } from 'vitest'
import { createRequire } from 'node:module'

describe.sequential('Utils', (it) => {
  const require = createRequire(import.meta.url)
  const { Utils, QualifierType }: typeof import('../dist') = require('../dist')

  it('should analyze empty qualifier', () => {
    const result = Utils.analyzeQualifier('')
    expect(result).toHaveLength(0)
  })

  it('should analyze mcc_mnc-language-region', () => {
    const result = Utils.analyzeQualifier('mcc460_mnc00-zh_Hant_CN')
    expect(result).toHaveLength(5)
    
    expect(result[0].qualifierType).toBe(QualifierType.MCC)
    expect(result[0].qualifierValue).toBe('mcc460')
    
    expect(result[1].qualifierType).toBe(QualifierType.MNC)
    expect(result[1].qualifierValue).toBe('mnc00')
    
    expect(result[2].qualifierType).toBe(QualifierType.LanguageCode)
    expect(result[2].qualifierValue).toBe('zh')
    
    expect(result[3].qualifierType).toBe(QualifierType.LanguageCode)
    expect(result[3].qualifierValue).toBe('Hant')
    
    expect(result[4].qualifierType).toBe(QualifierType.RegionCode)
    expect(result[4].qualifierValue).toBe('CN')
  })

  it('should analyze language-device-density', () => {
    const result = Utils.analyzeQualifier('zh_CN-car-ldpi')
    expect(result).toHaveLength(4)
    
    expect(result[0].qualifierType).toBe(QualifierType.LanguageCode)
    expect(result[0].qualifierValue).toBe('zh')
    
    expect(result[1].qualifierType).toBe(QualifierType.RegionCode)
    expect(result[1].qualifierValue).toBe('CN')
    
    expect(result[2].qualifierType).toBe(QualifierType.DeviceType)
    expect(result[2].qualifierValue).toBe('car')
    
    expect(result[3].qualifierType).toBe(QualifierType.ScreenDensity)
    expect(result[3].qualifierValue).toBe('ldpi')
  })

  it('should analyze complex qualifier', () => {
    const result = Utils.analyzeQualifier('mcc460_mnc00-zh_CN-vertical-phone-light-xldpi')
    expect(result).toHaveLength(8)
    
    expect(result[0].qualifierType).toBe(QualifierType.MCC)
    expect(result[0].qualifierValue).toBe('mcc460')
    
    expect(result[1].qualifierType).toBe(QualifierType.MNC)
    expect(result[1].qualifierValue).toBe('mnc00')
    
    expect(result[2].qualifierType).toBe(QualifierType.LanguageCode)
    expect(result[2].qualifierValue).toBe('zh')
    
    expect(result[3].qualifierType).toBe(QualifierType.RegionCode)
    expect(result[3].qualifierValue).toBe('CN')
    
    expect(result[4].qualifierType).toBe(QualifierType.Orientation)
    expect(result[4].qualifierValue).toBe('vertical')
    
    expect(result[5].qualifierType).toBe(QualifierType.DeviceType)
    expect(result[5].qualifierValue).toBe('phone')
    
    expect(result[6].qualifierType).toBe(QualifierType.ColorMode)
    expect(result[6].qualifierValue).toBe('light')
    
    expect(result[7].qualifierType).toBe(QualifierType.ScreenDensity)
    expect(result[7].qualifierValue).toBe('xldpi')
  })

  it('should analyze single qualifiers', () => {
    const darkResult = Utils.analyzeQualifier('dark')
    expect(darkResult).toHaveLength(1)
    expect(darkResult[0].qualifierType).toBe(QualifierType.ColorMode)
    expect(darkResult[0].qualifierValue).toBe('dark')

    const mdpiResult = Utils.analyzeQualifier('mdpi')
    expect(mdpiResult).toHaveLength(1)
    expect(mdpiResult[0].qualifierType).toBe(QualifierType.ScreenDensity)
    expect(mdpiResult[0].qualifierValue).toBe('mdpi')

    const wearableResult = Utils.analyzeQualifier('wearable')
    expect(wearableResult).toHaveLength(1)
    expect(wearableResult[0].qualifierType).toBe(QualifierType.DeviceType)
    expect(wearableResult[0].qualifierValue).toBe('wearable')
  })

  it('should fail on invalid MCC code', () => {
    const result = Utils.analyzeQualifier('mcc9999-zh_CN')
    expect(result).toHaveLength(0)
  })

  it('should fail on invalid MNC code', () => {
    const result = Utils.analyzeQualifier('mcc460_mnc9999-zh_CN')
    expect(result).toHaveLength(0)
  })

  it('should fail on invalid language code', () => {
    const result = Utils.analyzeQualifier('xyz')
    expect(result).toHaveLength(0)
  })

  it('should fail on invalid device type', () => {
    const result = Utils.analyzeQualifier('zh_CN-superdevice-mdpi')
    expect(result).toHaveLength(0)
  })

  it('should fail on invalid orientation', () => {
    const result = Utils.analyzeQualifier('diagonal-phone')
    expect(result).toHaveLength(0)
  })

  it('should fail on invalid color mode', () => {
    const result = Utils.analyzeQualifier('phone-blue-mdpi')
    expect(result).toHaveLength(0)
  })

  it('should fail on invalid screen density', () => {
    const result = Utils.analyzeQualifier('phone-dark-ultra4k')
    expect(result).toHaveLength(0)
  })

  it('should fail on wrong order (device before orientation)', () => {
    // phone-vertical 顺序错误（设备在方向之前）
    const result = Utils.analyzeQualifier('phone-vertical')
    expect(result).toHaveLength(0)
  })

  it('should pass on correct order (orientation before device)', () => {
    // vertical-phone 顺序正确
    const result = Utils.analyzeQualifier('vertical-phone')
    expect(result).toHaveLength(2)
    expect(result[0].qualifierType).toBe(QualifierType.Orientation)
    expect(result[1].qualifierType).toBe(QualifierType.DeviceType)
  })

  it('should fail on mixed valid and invalid qualifiers', () => {
    const result = Utils.analyzeQualifier('zh_CN-invalid-phone-wrongcolor-mdpi')
    expect(result).toHaveLength(0)
  })

  it('should fail on only invalid qualifiers', () => {
    const result = Utils.analyzeQualifier('invalid-wrong-bad')
    expect(result).toHaveLength(0)
  })

  it('should handle multiple consecutive dashes', () => {
    const result = Utils.analyzeQualifier('zh_CN--phone')
    expect(result).toHaveLength(3)
    expect(result[0].qualifierType).toBe(QualifierType.LanguageCode)
    expect(result[1].qualifierType).toBe(QualifierType.RegionCode)
    expect(result[2].qualifierType).toBe(QualifierType.DeviceType)
  })

  it('should handle trailing dash', () => {
    const result = Utils.analyzeQualifier('zh_CN-phone-')
    expect(result).toHaveLength(3)
    expect(result[0].qualifierType).toBe(QualifierType.LanguageCode)
    expect(result[1].qualifierType).toBe(QualifierType.RegionCode)
    expect(result[2].qualifierType).toBe(QualifierType.DeviceType)
  })

  it('should handle leading dash', () => {
    const result = Utils.analyzeQualifier('-zh_CN-phone')
    expect(result).toHaveLength(3)
    expect(result[0].qualifierType).toBe(QualifierType.LanguageCode)
    expect(result[1].qualifierType).toBe(QualifierType.RegionCode)
    expect(result[2].qualifierType).toBe(QualifierType.DeviceType)
  })

  it('should handle case insensitivity', () => {
    const resultUpper = Utils.analyzeQualifier('DARK')
    const resultLower = Utils.analyzeQualifier('dark')
    const resultMixed = Utils.analyzeQualifier('Dark')
    
    expect(resultUpper).toHaveLength(1)
    expect(resultLower).toHaveLength(1)
    expect(resultMixed).toHaveLength(1)
    
    expect(resultUpper[0].qualifierType).toBe(QualifierType.ColorMode)
    expect(resultLower[0].qualifierType).toBe(QualifierType.ColorMode)
    expect(resultMixed[0].qualifierType).toBe(QualifierType.ColorMode)
  })

  it('should fail on duplicate qualifiers', () => {
    // 重复的限定词应该被拒绝
    const result = Utils.analyzeQualifier('phone-phone')
    expect(result).toHaveLength(0)
  })

  it('should fail on single MCC without MNC', () => {
    const result = Utils.analyzeQualifier('mcc460-zh_CN')
    expect(result).toHaveLength(0)
  })

  it('should allow skipping stages', () => {
    // 测试跳过某些阶段
    const result1 = Utils.analyzeQualifier('zh-dark')
    expect(result1).toHaveLength(2)
    
    const result2 = Utils.analyzeQualifier('phone-mdpi')
    expect(result2).toHaveLength(2)
    
    const result3 = Utils.analyzeQualifier('vertical-dark')
    expect(result3).toHaveLength(2)
  })
})