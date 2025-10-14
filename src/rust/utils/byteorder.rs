// 自定义字节序转换函数，用于解决 le16toh 符号缺失问题

/// 将小端序的 16 位整数转换为主机字节序
/// 如果系统是小端序，则直接返回；如果是大端序，则进行字节交换
#[inline]
pub fn le16toh(val: u16) -> u16 {
  if cfg!(target_endian = "little") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将主机字节序的 16 位整数转换为小端序
#[inline]
pub fn htole16(val: u16) -> u16 {
  if cfg!(target_endian = "little") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将大端序的 16 位整数转换为主机字节序
#[inline]
pub fn be16toh(val: u16) -> u16 {
  if cfg!(target_endian = "big") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将主机字节序的 16 位整数转换为大端序
#[inline]
pub fn htobe16(val: u16) -> u16 {
  if cfg!(target_endian = "big") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将小端序的 32 位整数转换为主机字节序
#[inline]
pub fn le32toh(val: u32) -> u32 {
  if cfg!(target_endian = "little") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将主机字节序的 32 位整数转换为小端序
#[inline]
pub fn htole32(val: u32) -> u32 {
  if cfg!(target_endian = "little") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将大端序的 32 位整数转换为主机字节序
#[inline]
pub fn be32toh(val: u32) -> u32 {
  if cfg!(target_endian = "big") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将主机字节序的 32 位整数转换为大端序
#[inline]
pub fn htobe32(val: u32) -> u32 {
  if cfg!(target_endian = "big") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将小端序的 64 位整数转换为主机字节序
#[inline]
pub fn le64toh(val: u64) -> u64 {
  if cfg!(target_endian = "little") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将主机字节序的 64 位整数转换为小端序
#[inline]
pub fn htole64(val: u64) -> u64 {
  if cfg!(target_endian = "little") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将大端序的 64 位整数转换为主机字节序
#[inline]
pub fn be64toh(val: u64) -> u64 {
  if cfg!(target_endian = "big") {
    val
  } else {
    val.swap_bytes()
  }
}

/// 将主机字节序的 64 位整数转换为大端序
#[inline]
pub fn htobe64(val: u64) -> u64 {
  if cfg!(target_endian = "big") {
    val
  } else {
    val.swap_bytes()
  }
}

// 为避免在部分 Linux 发行版/架构上缺失 libbsd 导致的运行期符号解析失败，
// 这里显式导出与 libbsd 同名的 C 符号，以便动态链接器可在本模块内解析。
// 这些实现仅依赖于主机端大小端判断和字节交换，与上方纯 Rust 版本一致。

#[export_name = "le16toh"]
pub extern "C" fn c_le16toh(x: u16) -> u16 {
  le16toh(x)
}

#[export_name = "be16toh"]
pub extern "C" fn c_be16toh(x: u16) -> u16 {
  be16toh(x)
}

#[export_name = "le32toh"]
pub extern "C" fn c_le32toh(x: u32) -> u32 {
  le32toh(x)
}

#[export_name = "be32toh"]
pub extern "C" fn c_be32toh(x: u32) -> u32 {
  be32toh(x)
}

#[export_name = "le64toh"]
pub extern "C" fn c_le64toh(x: u64) -> u64 {
  le64toh(x)
}

#[export_name = "be64toh"]
pub extern "C" fn c_be64toh(x: u64) -> u64 {
  be64toh(x)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_le16toh() {
    // 在小端序系统上，le16toh 应该直接返回原值
    let val = 0x1234u16;
    assert_eq!(le16toh(val), val);
  }

  #[test]
  fn test_htole16() {
    let val = 0x1234u16;
    assert_eq!(htole16(val), val);
  }
}
