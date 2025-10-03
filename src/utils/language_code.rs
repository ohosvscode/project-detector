use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
pub enum LanguageCode {
    /// 🇪🇹 Afar
    ///
    /// 阿法爾語
    Aa,
    /// 🇬🇪 Abkhazian
    ///
    /// 阿布哈茲語
    Ab,
    /// 🏛️ Avestan
    ///
    /// 阿維斯陀語
    Ae,
    /// 🇿🇦 Afrikaans
    ///
    /// 南非語
    Af,
    /// 🇬🇭 Akan
    ///
    /// 阿坎语
    Ak,
    /// 🇪🇹 Amharic
    ///
    /// 阿姆哈拉语
    Am,
    /// 🇪🇸 Aragonese
    ///
    /// 阿拉贡语
    An,
    /// 🇸🇦 Arabic
    ///
    /// 阿拉伯语
    Ar,
    /// 🇮🇳 Assamese
    ///
    /// 阿萨姆语
    As,
    /// 🇷🇺 Avaric
    ///
    /// 阿瓦尔语
    Av,
    /// 🇧🇴 Aymara
    ///
    /// 艾馬拉語
    Ay,
    /// 🇦🇿 Azerbaijani
    ///
    /// 阿塞拜疆语
    Az,
    /// 🇷🇺 Bashkir
    ///
    /// 巴什基尔语
    Ba,
    /// 🇧🇾 Belarusian
    ///
    /// 白俄罗斯语
    Be,
    /// 🇧🇬 Bulgarian
    ///
    /// 保加利亚语
    Bg,
    /// 🇻🇺 Bislama
    ///
    /// 比斯拉馬語
    Bi,
    /// 🇲🇱 Bambara
    ///
    /// 班巴拉語
    Bm,
    /// 🇧🇩 Bengali
    ///
    /// 孟加拉语
    Bn,
    /// 🇨🇳 Tibetan
    ///
    /// 藏語
    Bo,
    /// 🇫🇷 Breton
    ///
    /// 布列塔尼語
    Br,
    /// 🇧🇦 Bosnian
    ///
    /// 波斯尼亚语
    Bs,
    /// 🇪🇸 Catalan
    ///
    /// 加泰罗尼亚语
    Ca,
    /// 🇷🇺 Chechen
    ///
    /// 車臣語
    Ce,
    /// 🇬🇺 Chamorro
    ///
    /// 查莫罗语
    Ch,
    /// 🇫🇷 Corsican
    ///
    /// 科西嘉语
    Co,
    /// 🇨🇦 Cree
    ///
    /// 克里语
    Cr,
    /// 🇨🇿 Czech
    ///
    /// 捷克语
    Cs,
    /// 🏛️ Church Slavic
    ///
    /// 古教會斯拉夫語
    Cu,
    /// 🇷🇺 Chuvash
    ///
    /// 楚瓦什語
    Cv,
    /// 🇬🇧 Welsh
    ///
    /// 威尔士语
    Cy,
    /// 🇩🇰 Danish
    ///
    /// 丹麦语
    Da,
    /// 🇩🇪 German
    ///
    /// 德语
    De,
    /// 🇲🇻 Divehi
    ///
    /// 迪维希语
    Dv,
    /// 🇧🇹 Dzongkha
    ///
    /// 宗喀語
    Dz,
    /// 🇬🇭 Ewe
    ///
    /// 埃維語
    Ee,
    /// 🇬🇷 Greek
    ///
    /// 希腊语
    El,
    /// 🇺🇸 English
    ///
    /// 英语
    En,
    /// 🌍 Esperanto
    ///
    /// 世界语
    Eo,
    /// 🇪🇸 Spanish
    ///
    /// 西班牙语
    Es,
    /// 🇪🇪 Estonian
    ///
    /// 爱沙尼亚语
    Et,
    /// 🇪🇸 Basque
    ///
    /// 巴斯克語
    Eu,
    /// 🇮🇷 Persian
    ///
    /// 波斯语
    Fa,
    /// 🇧🇫 Fulah
    ///
    /// 富拉語
    Ff,
    /// 🇫🇮 Finnish
    ///
    /// 芬兰语
    Fi,
    /// 🇫🇯 Fijian
    ///
    /// 斐濟語
    Fj,
    /// 🇫🇴 Faroese
    ///
    /// 法罗语
    Fo,
    /// 🇫🇷 French
    ///
    /// 法语
    Fr,
    /// 🇳🇱 West Frisian
    ///
    /// 西弗里斯兰语
    Fy,
    /// 🇮🇪 Irish
    ///
    /// 愛爾蘭語
    Ga,
    /// 🇬🇧 Gaelic
    ///
    /// 苏格兰盖尔语
    Gd,
    /// 🇪🇸 Galician
    ///
    /// 加利西亞語
    Gl,
    /// 🇵🇾 Guarani
    ///
    /// 瓜拉尼語
    Gn,
    /// 🇮🇳 Gujarati
    ///
    /// 古吉拉特语
    Gu,
    /// 🇮🇲 Manx
    ///
    /// 曼島語
    Gv,
    /// 🇳🇬 Hausa
    ///
    /// 豪萨语
    Ha,
    /// 🇮🇱 Hebrew
    ///
    /// 希伯来语
    He,
    /// 🇮🇳 Hindi
    ///
    /// 印地语
    Hi,
    /// 🇵🇬 Hiri Motu
    ///
    /// 希里摩圖語
    Ho,
    /// 🇭🇷 Croatian
    ///
    /// 克罗地亚语
    Hr,
    /// 🇭🇹 Haitian
    ///
    /// 海地克里奧爾語
    Ht,
    /// 🇭🇺 Hungarian
    ///
    /// 匈牙利语
    Hu,
    /// 🇦🇲 Armenian
    ///
    /// 亚美尼亚语
    Hy,
    /// 🇳🇦 Herero
    ///
    /// 赫雷羅語
    Hz,
    /// 🌍 Interlingua
    ///
    /// 国际语
    Ia,
    /// 🇮🇩 Indonesian
    ///
    /// 印度尼西亚语
    Id,
    /// 🌍 Interlingue
    ///
    /// 西方國際語
    Ie,
    /// 🇳🇬 Igbo
    ///
    /// 伊博語
    Ig,
    /// 🇨🇳 Sichuan Yi
    ///
    /// 彝語北部方言
    Ii,
    /// 🇺🇸 Inupiaq
    ///
    /// 因纽皮雅特语
    Ik,
    /// 🌍 Ido
    ///
    /// 伊多語
    Io,
    /// 🇮🇸 Icelandic
    ///
    /// 冰岛语
    Is,
    /// 🇮🇹 Italian
    ///
    /// 意大利语
    It,
    /// 🇨🇦 Inuktitut
    ///
    /// 伊努克提圖特語
    Iu,
    /// 🇯🇵 Japanese
    ///
    /// 日语
    Ja,
    /// 🇮🇩 Javanese
    ///
    /// 爪哇語
    Jv,
    /// 🇬🇪 Georgian
    ///
    /// 格鲁吉亚语
    Ka,
    /// 🇨🇩 Kongo
    ///
    /// 剛果語
    Kg,
    /// 🇰🇪 Kikuyu
    ///
    /// 基庫尤語
    Ki,
    /// 🇳🇦 Kuanyama
    ///
    /// 寬亞瑪語
    Kj,
    /// 🇰🇿 Kazakh
    ///
    /// 哈萨克语
    Kk,
    /// 🇬🇱 Kalaallisut
    ///
    /// 格陵兰语
    Kl,
    /// 🇰🇭 Central Khmer
    ///
    /// 高棉语
    Km,
    /// 🇮🇳 Kannada
    ///
    /// 卡纳达语
    Kn,
    /// 🇰🇷 Korean
    ///
    /// 朝鮮語
    Ko,
    /// 🇳🇪 Kanuri
    ///
    /// 卡努里語
    Kr,
    /// 🇮🇳 Kashmiri
    ///
    /// 克什米爾語
    Ks,
    /// 🇮🇶 Kurdish
    ///
    /// 庫爾德語
    Ku,
    /// 🇷🇺 Komi
    ///
    /// 科米語
    Kv,
    /// 🇬🇧 Cornish
    ///
    /// 康瓦爾語
    Kw,
    /// 🇰🇬 Kirghiz
    ///
    /// 柯尔克孜语
    Ky,
    /// 🏛️ Latin
    ///
    /// 拉丁语
    La,
    /// 🇱🇺 Luxembourgish
    ///
    /// 卢森堡语
    Lb,
    /// 🇺🇬 Ganda
    ///
    /// 盧干達語
    Lg,
    /// 🇳🇱 Limburgan
    ///
    /// 林堡语
    Li,
    /// 🇨🇩 Lingala
    ///
    /// 林加拉语
    Ln,
    /// 🇱🇦 Lao
    ///
    /// 老挝语
    Lo,
    /// 🇱🇹 Lithuanian
    ///
    /// 立陶宛语
    Lt,
    /// 🇨🇩 Luba-Katanga
    ///
    /// 盧巴-卡丹加語
    Lu,
    /// 🇱🇻 Latvian
    ///
    /// 拉脱维亚语
    Lv,
    /// 🇲🇬 Malagasy
    ///
    /// 马达加斯加语
    Mg,
    /// 🇲🇭 Marshallese
    ///
    /// 馬紹爾語
    Mh,
    /// 🇳🇿 Maori
    ///
    /// 毛利语
    Mi,
    /// 🇲🇰 Macedonian
    ///
    /// 马其顿语
    Mk,
    /// 🇮🇳 Malayalam
    ///
    /// 马拉雅拉姆语
    Ml,
    /// 🇲🇳 Mongolian
    ///
    /// 蒙古语
    Mn,
    /// 🇮🇳 Marathi
    ///
    /// 马拉地语
    Mr,
    /// 🇲🇾 Malay
    ///
    /// 马来语
    Ms,
    /// 🇲🇹 Maltese
    ///
    /// 马耳他语
    Mt,
    /// 🇲🇲 Burmese
    ///
    /// 缅甸语
    My,
    /// 🇳🇷 Nauru
    ///
    /// 瑙鲁語
    Na,
    /// 🇳🇴 Norwegian Bokmål
    ///
    /// 書面挪威語
    Nb,
    /// 🇿🇼 North Ndebele
    ///
    /// 北恩德贝莱语
    Nd,
    /// 🇳🇵 Nepali
    ///
    /// 尼泊尔语
    Ne,
    /// 🇳🇦 Ndonga
    ///
    /// 恩敦加語
    Ng,
    /// 🇳🇱 Dutch
    ///
    /// 荷蘭語
    Nl,
    /// 🇳🇴 Norwegian Nynorsk
    ///
    /// 新挪威語
    Nn,
    /// 🇳🇴 Norwegian
    ///
    /// 挪威语
    No,
    /// 🇿🇦 South Ndebele
    ///
    /// 南恩德贝莱语
    Nr,
    /// 🇺🇸 Navajo
    ///
    /// 納瓦荷語
    Nv,
    /// 🇲🇼 Chichewa
    ///
    /// 齐切瓦语
    Ny,
    /// 🇫🇷 Occitan
    ///
    /// 奥克语
    Oc,
    /// 🇨🇦 Ojibwa
    ///
    /// 奥吉布瓦语
    Oj,
    /// 🇪🇹 Oromo
    ///
    /// 奧羅莫語
    Om,
    /// 🇮🇳 Oriya
    ///
    /// 奧里亞語
    Or,
    /// 🇷🇺 Ossetian
    ///
    /// 奧塞梯語
    Os,
    /// 🇮🇳 Punjabi
    ///
    /// 旁遮普語
    Pa,
    /// 🏛️ Pali
    ///
    /// 巴利语
    Pi,
    /// 🇵🇱 Polish
    ///
    /// 波兰语
    Pl,
    /// 🇦🇫 Pashto
    ///
    /// 普什图语
    Ps,
    /// 🇵🇹 Portuguese
    ///
    /// 葡萄牙語
    Pt,
    /// 🇵🇪 Quechua
    ///
    /// 克丘亞語
    Qu,
    /// 🇨🇭 Romansh
    ///
    /// 罗曼什语
    Rm,
    /// 🇧🇮 Rundi
    ///
    /// 基隆迪语
    Rn,
    /// 🇷🇴 Romanian
    ///
    /// 羅馬尼亞語
    Ro,
    /// 🇷🇺 Russian
    ///
    /// 俄语
    Ru,
    /// 🇷🇼 Kinyarwanda
    ///
    /// 盧安達語
    Rw,
    /// 🇮🇳 Sanskrit
    ///
    /// 梵语
    Sa,
    /// 🇮🇹 Sardinian
    ///
    /// 薩丁尼亞語
    Sc,
    /// 🇵🇰 Sindhi
    ///
    /// 信德语
    Sd,
    /// 🇳🇴 Northern Sami
    ///
    /// 北萨米语
    Se,
    /// 🇨🇫 Sango
    ///
    /// 桑戈語
    Sg,
    /// 🇱🇰 Sinhala
    ///
    /// 僧伽罗语
    Si,
    /// 🇸🇰 Slovak
    ///
    /// 斯洛伐克语
    Sk,
    /// 🇸🇮 Slovenian
    ///
    /// 斯洛文尼亚语
    Sl,
    /// 🇼🇸 Samoan
    ///
    /// 薩摩亞語
    Sm,
    /// 🇿🇼 Shona
    ///
    /// 紹納語
    Sn,
    /// 🇸🇴 Somali
    ///
    /// 索馬里語
    So,
    /// 🇦🇱 Albanian
    ///
    /// 阿尔巴尼亚语
    Sq,
    /// 🇷🇸 Serbian
    ///
    /// 塞尔维亚语
    Sr,
    /// 🇸🇿 Swati
    ///
    /// 史瓦帝語
    Ss,
    /// 🇱🇸 Southern Sotho
    ///
    /// 塞索托語
    St,
    /// 🇮🇩 Sundanese
    ///
    /// 巽他語
    Su,
    /// 🇸🇪 Swedish
    ///
    /// 瑞典語
    Sv,
    /// 🇹🇿 Swahili
    ///
    /// 斯瓦希里语
    Sw,
    /// 🇮🇳 Tamil
    ///
    /// 泰米尔语
    Ta,
    /// 🇮🇳 Telugu
    ///
    /// 泰卢固语
    Te,
    /// 🇹🇯 Tajik
    ///
    /// 塔吉克语
    Tg,
    /// 🇹🇭 Thai
    ///
    /// 泰语
    Th,
    /// 🇪🇷 Tigrinya
    ///
    /// 提格利尼亞語
    Ti,
        /// 🇹🇲 Turkmen
    ///
    /// 土库曼语
    Tk,
    /// 🇵🇭 Tagalog
    ///
    /// 他加祿語
    Tl,
    /// 🇧🇼 Tswana
    ///
    /// 茨瓦纳语
    Tn,
    /// 🇹🇴 Tonga
    ///
    /// 湯加語
    To,
    /// 🇹🇷 Turkish
    ///
    /// 土耳其语
    Tr,
    /// 🇿🇦 Tsonga
    ///
    /// 聪加语
    Ts,
    /// 🇷🇺 Tatar
    ///
    /// 鞑靼语
    Tt,
    /// 🇬🇭 Twi
    ///
    /// 契維語
    Tw,
    /// 🇵🇫 Tahitian
    ///
    /// 大溪地語
    Ty,
    /// 🇨🇳 Uighur
    ///
    /// 维吾尔语
    Ug,
    /// 🇺🇦 Ukrainian
    ///
    /// 乌克兰语
    Uk,
    /// 🇵🇰 Urdu
    ///
    /// 乌尔都语
    Ur,
    /// 🇺🇿 Uzbek
    ///
    /// 乌孜别克语
    Uz,
    /// 🇿🇦 Venda
    ///
    /// 文達語
    Ve,
    /// 🇻🇳 Vietnamese
    ///
    /// 越南语
    Vi,
    /// 🌍 Volapük
    ///
    /// 沃拉普克语
    Vo,
    /// 🇧🇪 Walloon
    ///
    /// 瓦隆语
    Wa,
    /// 🇸🇳 Wolof
    ///
    /// 沃洛夫語
    Wo,
    /// 🇿🇦 Xhosa
    ///
    /// 科萨语
    Xh,
    /// 🇮🇱 Yiddish
    ///
    /// 意第緒語
    Yi,
    /// 🇳🇬 Yoruba
    ///
    /// 約魯巴語
    Yo,
    /// 🇨🇳 Zhuang
    ///
    /// 壮语
    Za,
    /// 🇨🇳 Chinese
    ///
    /// 中文
    Zh,
    /// 🇿🇦 Zulu
    ///
    /// 祖鲁语
    Zu,
}

impl LanguageCode {
  /// Check if the language code is a valid language code.
  /// Supports both String and LanguageCode enum types.
  /// Example: "en" => true, "en-US" => false
  pub fn is<T: Into<LanguageCodeInput>>(language_code: T) -> bool {
    match language_code.into() {
      LanguageCodeInput::String(s) => {
        // 将输入转换为首字母大写，其余小写的格式
        let formatted_code = if s.len() >= 2 {
          let mut chars = s.chars();
          let first_char = chars.next().unwrap().to_uppercase().next().unwrap();
          let rest: String = chars.map(|c| c.to_lowercase().next().unwrap()).collect();
          format!("{}{}", first_char, rest)
        } else {
          s.to_uppercase()
        };
        
        // 遍历所有 LanguageCode 枚举值
        for variant in LanguageCode::iter() {
          // 将枚举变体转换为字符串进行比较
          if format!("{:?}", variant) == formatted_code {
            return true;
          }
        }
        
        false
      }
      LanguageCodeInput::LanguageCode(_lc) => {
        // 如果传入的是 LanguageCode 枚举，直接返回 true
        // 因为如果能够构造出 LanguageCode，说明它是有效的
        true
      }
    }
  }
}

/// 用于表示语言代码输入的枚举
pub enum LanguageCodeInput {
  String(String),
  LanguageCode(LanguageCode),
}

/// 为 String 实现 Into<LanguageCodeInput>
impl From<String> for LanguageCodeInput {
  fn from(s: String) -> Self {
    LanguageCodeInput::String(s)
  }
}

/// 为 &str 实现 Into<LanguageCodeInput>
impl From<&str> for LanguageCodeInput {
  fn from(s: &str) -> Self {
    LanguageCodeInput::String(s.to_string())
  }
}

/// 为 LanguageCode 实现 Into<LanguageCodeInput>
impl From<LanguageCode> for LanguageCodeInput {
  fn from(lc: LanguageCode) -> Self {
    LanguageCodeInput::LanguageCode(lc)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_language_code_is() {
    // 测试 String 输入
    assert!(LanguageCode::is("en".to_string()));
    assert!(LanguageCode::is("zh".to_string()));
    assert!(LanguageCode::is("ja".to_string()));
    assert!(LanguageCode::is("fr".to_string()));
    assert!(LanguageCode::is("de".to_string()));
    
    // 测试 &str 输入
    assert!(LanguageCode::is("en"));
    assert!(LanguageCode::is("zh"));
    assert!(LanguageCode::is("ja"));
    
    // 测试大小写不敏感
    assert!(LanguageCode::is("EN".to_string()));
    assert!(LanguageCode::is("Zh".to_string()));
    assert!(LanguageCode::is("JA".to_string()));
    
    // 测试 LanguageCode 枚举输入
    assert!(LanguageCode::is(LanguageCode::En));
    assert!(LanguageCode::is(LanguageCode::Zh));
    assert!(LanguageCode::is(LanguageCode::Ja));
    assert!(LanguageCode::is(LanguageCode::Fr));
    assert!(LanguageCode::is(LanguageCode::De));
    
    // 测试无效的语言代码
    assert!(!LanguageCode::is("en-US".to_string()));
    assert!(!LanguageCode::is("zh-CN".to_string()));
    assert!(!LanguageCode::is("invalid".to_string()));
    assert!(!LanguageCode::is("".to_string()));
    assert!(!LanguageCode::is("xyz".to_string()));
  }
}