use object::pe::{
    IMAGE_REL_AMD64_ABSOLUTE, IMAGE_REL_AMD64_ADDR32, IMAGE_REL_AMD64_ADDR32NB,
    IMAGE_REL_AMD64_ADDR64, IMAGE_REL_AMD64_PAIR, IMAGE_REL_AMD64_REL32, IMAGE_REL_AMD64_REL32_1,
    IMAGE_REL_AMD64_REL32_2, IMAGE_REL_AMD64_REL32_3, IMAGE_REL_AMD64_REL32_4,
    IMAGE_REL_AMD64_REL32_5, IMAGE_REL_AMD64_SECREL, IMAGE_REL_AMD64_SECREL7,
    IMAGE_REL_AMD64_SECTION, IMAGE_REL_AMD64_SREL32, IMAGE_REL_AMD64_SSPAN32,
    IMAGE_REL_AMD64_TOKEN, IMAGE_REL_I386_ABSOLUTE, IMAGE_REL_I386_DIR16, IMAGE_REL_I386_DIR32,
    IMAGE_REL_I386_DIR32NB, IMAGE_REL_I386_REL16, IMAGE_REL_I386_REL32, IMAGE_REL_I386_SECREL,
    IMAGE_REL_I386_SECREL7, IMAGE_REL_I386_SECTION, IMAGE_REL_I386_SEG12, IMAGE_REL_I386_TOKEN,
    IMAGE_SCN_CNT_CODE, IMAGE_SCN_CNT_INITIALIZED_DATA, IMAGE_SCN_CNT_UNINITIALIZED_DATA,
    IMAGE_SCN_GPREL, IMAGE_SCN_LNK_COMDAT, IMAGE_SCN_LNK_INFO, IMAGE_SCN_LNK_NRELOC_OVFL,
    IMAGE_SCN_LNK_OTHER, IMAGE_SCN_LNK_REMOVE, IMAGE_SCN_MEM_DISCARDABLE, IMAGE_SCN_MEM_EXECUTE,
    IMAGE_SCN_MEM_LOCKED, IMAGE_SCN_MEM_NOT_CACHED, IMAGE_SCN_MEM_NOT_PAGED, IMAGE_SCN_MEM_PRELOAD,
    IMAGE_SCN_MEM_PURGEABLE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_SHARED, IMAGE_SCN_MEM_WRITE,
    IMAGE_SCN_TYPE_NO_PAD,
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
    ser::SerializeSeq,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CoffYamlSection {
    pub name: String,

    #[serde(
        deserialize_with = "characteristics_deserializer",
        serialize_with = "characteristics_serializer"
    )]
    pub characteristics: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<usize>,

    #[serde(
        deserialize_with = "hex::serde::deserialize",
        serialize_with = "hex::serde::serialize_upper"
    )]
    pub section_data: Vec<u8>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_of_raw_data: Option<u32>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relocations: Vec<CoffYamlSectionRelocation>,
}

fn characteristics_deserializer<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    struct CharacteristicsVisitor;

    impl<'de> Visitor<'de> for CharacteristicsVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_SCN_*' string list or integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            u32::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut flags = 0;

            while let Some(val) = seq.next_element::<&str>()? {
                flags |= match val {
                    "IMAGE_SCN_TYPE_NO_PAD" => IMAGE_SCN_TYPE_NO_PAD,
                    "IMAGE_SCN_CNT_CODE" => IMAGE_SCN_CNT_CODE,
                    "IMAGE_SCN_CNT_INITIALIZED_DATA" => IMAGE_SCN_CNT_INITIALIZED_DATA,
                    "IMAGE_SCN_CNT_UNINITIALIZED_DATA" => IMAGE_SCN_CNT_UNINITIALIZED_DATA,
                    "IMAGE_SCN_LNK_OTHER" => IMAGE_SCN_LNK_OTHER,
                    "IMAGE_SCN_LNK_INFO" => IMAGE_SCN_LNK_INFO,
                    "IMAGE_SCN_LNK_REMOVE" => IMAGE_SCN_LNK_REMOVE,
                    "IMAGE_SCN_LNK_COMDAT" => IMAGE_SCN_LNK_COMDAT,
                    "IMAGE_SCN_GPREL" => IMAGE_SCN_GPREL,
                    "IMAGE_SCN_MEM_PURGEABLE" => IMAGE_SCN_MEM_PURGEABLE,
                    "IMAGE_SCN_MEM_LOCKED" => IMAGE_SCN_MEM_LOCKED,
                    "IMAGE_SCN_MEM_PRELOAD" => IMAGE_SCN_MEM_PRELOAD,
                    "IMAGE_SCN_LNK_NRELOC_OVFL" => IMAGE_SCN_LNK_NRELOC_OVFL,
                    "IMAGE_SCN_MEM_DISCARDABLE" => IMAGE_SCN_MEM_DISCARDABLE,
                    "IMAGE_SCN_MEM_NOT_CACHED" => IMAGE_SCN_MEM_NOT_CACHED,
                    "IMAGE_SCN_MEM_NOT_PAGED" => IMAGE_SCN_MEM_NOT_PAGED,
                    "IMAGE_SCN_MEM_SHARED" => IMAGE_SCN_MEM_SHARED,
                    "IMAGE_SCN_MEM_EXECUTE" => IMAGE_SCN_MEM_EXECUTE,
                    "IMAGE_SCN_MEM_READ" => IMAGE_SCN_MEM_READ,
                    "IMAGE_SCN_MEM_WRITE" => IMAGE_SCN_MEM_WRITE,
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid section characteristic value {val}"
                        )));
                    }
                }
            }

            Ok(flags)
        }
    }

    deserializer.deserialize_any(CharacteristicsVisitor)
}

fn characteristics_serializer<S>(characteristics: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    const FLAGS: [(u32, &str); 20] = [
        (IMAGE_SCN_TYPE_NO_PAD, "IMAGE_SCN_TYPE_NO_PAD"),
        (IMAGE_SCN_CNT_CODE, "IMAGE_SCN_CNT_CODE"),
        (
            IMAGE_SCN_CNT_INITIALIZED_DATA,
            "IMAGE_SCN_CNT_INITIALIZED_DATA",
        ),
        (
            IMAGE_SCN_CNT_UNINITIALIZED_DATA,
            "IMAGE_SCN_CNT_UNINITIALIZED_DATA",
        ),
        (IMAGE_SCN_LNK_OTHER, "IMAGE_SCN_LNK_OTHER"),
        (IMAGE_SCN_LNK_INFO, "IMAGE_SCN_LNK_INFO"),
        (IMAGE_SCN_LNK_REMOVE, "IMAGE_SCN_LNK_REMOVE"),
        (IMAGE_SCN_LNK_COMDAT, "IMAGE_SCN_LNK_COMDAT"),
        (IMAGE_SCN_GPREL, "IMAGE_SCN_GPREL"),
        (IMAGE_SCN_MEM_PURGEABLE, "IMAGE_SCN_MEM_PURGEABLE"),
        (IMAGE_SCN_MEM_LOCKED, "IMAGE_SCN_MEM_LOCKED"),
        (IMAGE_SCN_MEM_PRELOAD, "IMAGE_SCN_MEM_PRELOAD"),
        (IMAGE_SCN_LNK_NRELOC_OVFL, "IMAGE_SCN_LNK_NRELOC_OVFL"),
        (IMAGE_SCN_MEM_DISCARDABLE, "IMAGE_SCN_MEM_DISCARDABLE"),
        (IMAGE_SCN_MEM_NOT_CACHED, "IMAGE_SCN_MEM_NOT_CACHED"),
        (IMAGE_SCN_MEM_NOT_PAGED, "IMAGE_SCN_MEM_NOT_PAGED"),
        (IMAGE_SCN_MEM_SHARED, "IMAGE_SCN_MEM_SHARED"),
        (IMAGE_SCN_MEM_EXECUTE, "IMAGE_SCN_MEM_EXECUTE"),
        (IMAGE_SCN_MEM_READ, "IMAGE_SCN_MEM_READ"),
        (IMAGE_SCN_MEM_WRITE, "IMAGE_SCN_MEM_WRITE"),
    ];

    let contains_flag = |bits: u32, flag: u32| bits & flag != 0;

    let flagcount = FLAGS
        .iter()
        .filter(|(flag, _)| contains_flag(*characteristics, *flag))
        .count();

    let mut seq = serializer.serialize_seq(Some(flagcount))?;
    for (flag, val) in FLAGS {
        if contains_flag(*characteristics, flag) {
            seq.serialize_element(val)?;
        }
    }

    seq.end()
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CoffYamlSectionRelocation {
    pub virtual_address: u32,
    pub symbol_name: String,

    #[serde(rename = "Type", deserialize_with = "relocation_type_deserializer")]
    pub typ: u16,
}

fn relocation_type_deserializer<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    struct RelocationTypeVisitor;

    impl Visitor<'_> for RelocationTypeVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_REL_*' string or integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            u16::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(match v {
                "IMAGE_REL_AMD64_ABSOLUTE" => IMAGE_REL_AMD64_ABSOLUTE,
                "IMAGE_REL_AMD64_ADDR64" => IMAGE_REL_AMD64_ADDR64,
                "IMAGE_REL_AMD64_ADDR32" => IMAGE_REL_AMD64_ADDR32,
                "IMAGE_REL_AMD64_ADDR32NB" => IMAGE_REL_AMD64_ADDR32NB,
                "IMAGE_REL_AMD64_REL32" => IMAGE_REL_AMD64_REL32,
                "IMAGE_REL_AMD64_REL32_1" => IMAGE_REL_AMD64_REL32_1,
                "IMAGE_REL_AMD64_REL32_2" => IMAGE_REL_AMD64_REL32_2,
                "IMAGE_REL_AMD64_REL32_3" => IMAGE_REL_AMD64_REL32_3,
                "IMAGE_REL_AMD64_REL32_4" => IMAGE_REL_AMD64_REL32_4,
                "IMAGE_REL_AMD64_REL32_5" => IMAGE_REL_AMD64_REL32_5,
                "IMAGE_REL_AMD64_SECTION" => IMAGE_REL_AMD64_SECTION,
                "IMAGE_REL_AMD64_SECREL" => IMAGE_REL_AMD64_SECREL,
                "IMAGE_REL_AMD64_SECREL7" => IMAGE_REL_AMD64_SECREL7,
                "IMAGE_REL_AMD64_TOKEN" => IMAGE_REL_AMD64_TOKEN,
                "IMAGE_REL_AMD64_SREL32" => IMAGE_REL_AMD64_SREL32,
                "IMAGE_REL_AMD64_PAIR" => IMAGE_REL_AMD64_PAIR,
                "IMAGE_REL_AMD64_SSPAN32" => IMAGE_REL_AMD64_SSPAN32,
                "IMAGE_REL_I386_ABSOLUTE" => IMAGE_REL_I386_ABSOLUTE,
                "IMAGE_REL_I386_DIR16" => IMAGE_REL_I386_DIR16,
                "IMAGE_REL_I386_DIR32" => IMAGE_REL_I386_DIR32,
                "IMAGE_REL_I386_DIR32NB" => IMAGE_REL_I386_DIR32NB,
                "IMAGE_REL_I386_SEG12" => IMAGE_REL_I386_SEG12,
                "IMAGE_REL_I386_SECTION" => IMAGE_REL_I386_SECTION,
                "IMAGE_REL_I386_SECREL" => IMAGE_REL_I386_SECREL,
                "IMAGE_REL_I386_TOKEN" => IMAGE_REL_I386_TOKEN,
                "IMAGE_REL_I386_SECREL7" => IMAGE_REL_I386_SECREL7,
                "IMAGE_REL_I386_REL16" => IMAGE_REL_I386_REL16,
                "IMAGE_REL_I386_REL32" => IMAGE_REL_I386_REL32,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid relocation type {v}"
                    )));
                }
            })
        }
    }

    deserializer.deserialize_any(RelocationTypeVisitor)
}

#[cfg(test)]
mod tests {
    use super::{characteristics_deserializer, relocation_type_deserializer};
    use crate::testutils;
    use object::pe::{
        IMAGE_REL_AMD64_ADDR32, IMAGE_REL_I386_DIR32, IMAGE_SCN_CNT_CODE,
        IMAGE_SCN_CNT_INITIALIZED_DATA,
    };

    #[test]
    fn characteristic_scalar_deserialize() {
        testutils::run_deserializer_tests(
            characteristics_deserializer,
            [("32", IMAGE_SCN_CNT_CODE)],
        );
    }

    #[test]
    fn characteristic_string_deserialize() {
        testutils::run_deserializer_tests(
            characteristics_deserializer,
            [
                ("[ IMAGE_SCN_CNT_CODE ]", IMAGE_SCN_CNT_CODE),
                (
                    "[ IMAGE_SCN_CNT_CODE, IMAGE_SCN_CNT_INITIALIZED_DATA ]",
                    IMAGE_SCN_CNT_CODE | IMAGE_SCN_CNT_INITIALIZED_DATA,
                ),
            ],
        );
    }

    #[test]
    fn relocation_type_string_deserialize() {
        testutils::run_deserializer_tests(
            relocation_type_deserializer,
            [
                ("IMAGE_REL_AMD64_ADDR32", IMAGE_REL_AMD64_ADDR32),
                ("IMAGE_REL_I386_DIR32", IMAGE_REL_I386_DIR32),
            ],
        );
    }
}
