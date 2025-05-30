use object::pe::{
    IMAGE_FILE_32BIT_MACHINE, IMAGE_FILE_AGGRESIVE_WS_TRIM, IMAGE_FILE_BYTES_REVERSED_HI,
    IMAGE_FILE_BYTES_REVERSED_LO, IMAGE_FILE_DEBUG_STRIPPED, IMAGE_FILE_DLL,
    IMAGE_FILE_EXECUTABLE_IMAGE, IMAGE_FILE_LARGE_ADDRESS_AWARE, IMAGE_FILE_LINE_NUMS_STRIPPED,
    IMAGE_FILE_LOCAL_SYMS_STRIPPED, IMAGE_FILE_MACHINE_AM33, IMAGE_FILE_MACHINE_AMD64,
    IMAGE_FILE_MACHINE_ARM, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_ARMNT,
    IMAGE_FILE_MACHINE_EBC, IMAGE_FILE_MACHINE_I386, IMAGE_FILE_MACHINE_IA64,
    IMAGE_FILE_MACHINE_M32R, IMAGE_FILE_MACHINE_MIPS16, IMAGE_FILE_MACHINE_MIPSFPU,
    IMAGE_FILE_MACHINE_MIPSFPU16, IMAGE_FILE_MACHINE_POWERPC, IMAGE_FILE_MACHINE_POWERPCFP,
    IMAGE_FILE_MACHINE_R4000, IMAGE_FILE_MACHINE_SH3, IMAGE_FILE_MACHINE_SH3DSP,
    IMAGE_FILE_MACHINE_SH4, IMAGE_FILE_MACHINE_SH5, IMAGE_FILE_MACHINE_THUMB,
    IMAGE_FILE_MACHINE_UNKNOWN, IMAGE_FILE_MACHINE_WCEMIPSV2, IMAGE_FILE_NET_RUN_FROM_SWAP,
    IMAGE_FILE_RELOCS_STRIPPED, IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP, IMAGE_FILE_SYSTEM,
    IMAGE_FILE_UP_SYSTEM_ONLY,
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
    ser::SerializeSeq,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CoffYamlHeader {
    #[serde(
        deserialize_with = "machine_deserializer",
        serialize_with = "machine_serializer"
    )]
    pub machine: u16,

    #[serde(
        deserialize_with = "characteristics_deserializer",
        serialize_with = "characteristics_serializer"
    )]
    pub characteristics: u16,
}

fn machine_deserializer<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    struct MachineVisitor;

    impl Visitor<'_> for MachineVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_FILE_MACHINE_*' string or integer")
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
                "IMAGE_FILE_MACHINE_UNKNOWN" => IMAGE_FILE_MACHINE_UNKNOWN,
                "IMAGE_FILE_MACHINE_AM33" => IMAGE_FILE_MACHINE_AM33,
                "IMAGE_FILE_MACHINE_AMD64" => IMAGE_FILE_MACHINE_AMD64,
                "IMAGE_FILE_MACHINE_ARM" => IMAGE_FILE_MACHINE_ARM,
                "IMAGE_FILE_MACHINE_ARMNT" => IMAGE_FILE_MACHINE_ARMNT,
                "IMAGE_FILE_MACHINE_ARM64" => IMAGE_FILE_MACHINE_ARM64,
                "IMAGE_FILE_MACHINE_EBC" => IMAGE_FILE_MACHINE_EBC,
                "IMAGE_FILE_MACHINE_I386" => IMAGE_FILE_MACHINE_I386,
                "IMAGE_FILE_MACHINE_IA64" => IMAGE_FILE_MACHINE_IA64,
                "IMAGE_FILE_MACHINE_M32R" => IMAGE_FILE_MACHINE_M32R,
                "IMAGE_FILE_MACHINE_MIPS16" => IMAGE_FILE_MACHINE_MIPS16,
                "IMAGE_FILE_MACHINE_MIPSFPU" => IMAGE_FILE_MACHINE_MIPSFPU,
                "IMAGE_FILE_MACHINE_MIPSFPU16" => IMAGE_FILE_MACHINE_MIPSFPU16,
                "IMAGE_FILE_MACHINE_POWERPC" => IMAGE_FILE_MACHINE_POWERPC,
                "IMAGE_FILE_MACHINE_POWERPCFP" => IMAGE_FILE_MACHINE_POWERPCFP,
                "IMAGE_FILE_MACHINE_R4000" => IMAGE_FILE_MACHINE_R4000,
                "IMAGE_FILE_MACHINE_SH3" => IMAGE_FILE_MACHINE_SH3,
                "IMAGE_FILE_MACHINE_SH3DSP" => IMAGE_FILE_MACHINE_SH3DSP,
                "IMAGE_FILE_MACHINE_SH4" => IMAGE_FILE_MACHINE_SH4,
                "IMAGE_FILE_MACHINE_SH5" => IMAGE_FILE_MACHINE_SH5,
                "IMAGE_FILE_MACHINE_THUMB" => IMAGE_FILE_MACHINE_THUMB,
                "IMAGE_FILE_MACHINE_WCEMIPSV2" => IMAGE_FILE_MACHINE_WCEMIPSV2,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid machine type {v}"
                    )));
                }
            })
        }
    }

    deserializer.deserialize_any(MachineVisitor)
}

fn machine_serializer<S>(machine: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *machine {
        IMAGE_FILE_MACHINE_UNKNOWN => serializer.serialize_str("IMAGE_FILE_MACHINE_UNKNOWN"),
        IMAGE_FILE_MACHINE_AM33 => serializer.serialize_str("IMAGE_FILE_MACHINE_AM33"),
        IMAGE_FILE_MACHINE_AMD64 => serializer.serialize_str("IMAGE_FILE_MACHINE_AMD64"),
        IMAGE_FILE_MACHINE_ARM => serializer.serialize_str("IMAGE_FILE_MACHINE_ARM"),
        IMAGE_FILE_MACHINE_ARMNT => serializer.serialize_str("IMAGE_FILE_MACHINE_ARMNT"),
        IMAGE_FILE_MACHINE_ARM64 => serializer.serialize_str("IMAGE_FILE_MACHINE_ARM64"),
        IMAGE_FILE_MACHINE_EBC => serializer.serialize_str("IMAGE_FILE_MACHINE_EBC"),
        IMAGE_FILE_MACHINE_I386 => serializer.serialize_str("IMAGE_FILE_MACHINE_I386"),
        IMAGE_FILE_MACHINE_IA64 => serializer.serialize_str("IMAGE_FILE_MACHINE_IA64"),
        IMAGE_FILE_MACHINE_M32R => serializer.serialize_str("IMAGE_FILE_MACHINE_M32R"),
        IMAGE_FILE_MACHINE_MIPS16 => serializer.serialize_str("IMAGE_FILE_MACHINE_MIPS16"),
        IMAGE_FILE_MACHINE_MIPSFPU => serializer.serialize_str("IMAGE_FILE_MACHINE_MIPSFPU"),
        IMAGE_FILE_MACHINE_MIPSFPU16 => serializer.serialize_str("IMAGE_FILE_MACHINE_MIPSFPU16"),
        IMAGE_FILE_MACHINE_POWERPC => serializer.serialize_str("IMAGE_FILE_MACHINE_POWERPC"),
        IMAGE_FILE_MACHINE_POWERPCFP => serializer.serialize_str("IMAGE_FILE_MACHINE_POWERPCFP"),
        IMAGE_FILE_MACHINE_R4000 => serializer.serialize_str("IMAGE_FILE_MACHINE_R4000"),
        IMAGE_FILE_MACHINE_SH3 => serializer.serialize_str("IMAGE_FILE_MACHINE_SH3"),
        IMAGE_FILE_MACHINE_SH3DSP => serializer.serialize_str("IMAGE_FILE_MACHINE_SH3DSP"),
        IMAGE_FILE_MACHINE_SH4 => serializer.serialize_str("IMAGE_FILE_MACHINE_SH4"),
        IMAGE_FILE_MACHINE_SH5 => serializer.serialize_str("IMAGE_FILE_MACHINE_SH5"),
        IMAGE_FILE_MACHINE_THUMB => serializer.serialize_str("IMAGE_FILE_MACHINE_THUMB"),
        IMAGE_FILE_MACHINE_WCEMIPSV2 => serializer.serialize_str("IMAGE_FILE_MACHINE_WCEMIPSV2"),
        o => serializer.serialize_u16(o),
    }
}

fn characteristics_deserializer<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    struct CharacteristicsVisitor;

    impl<'de> Visitor<'de> for CharacteristicsVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_FILE_*' characteristics string list or integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            u16::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut flags = 0;

            while let Some(val) = seq.next_element::<&str>()? {
                flags |= match val {
                    "IMAGE_FILE_RELOCS_STRIPPED" => IMAGE_FILE_RELOCS_STRIPPED,
                    "IMAGE_FILE_EXECUTABLE_IMAGE" => IMAGE_FILE_EXECUTABLE_IMAGE,
                    "IMAGE_FILE_LINE_NUMS_STRIPPED" => IMAGE_FILE_LINE_NUMS_STRIPPED,
                    "IMAGE_FILE_LOCAL_SYMS_STRIPPED" => IMAGE_FILE_LOCAL_SYMS_STRIPPED,
                    "IMAGE_FILE_AGGRESSIVE_WS_TRIM" => IMAGE_FILE_AGGRESIVE_WS_TRIM,
                    "IMAGE_FILE_LARGE_ADDRESS_AWARE" => IMAGE_FILE_LARGE_ADDRESS_AWARE,
                    "IMAGE_FILE_BYTES_REVERSED_LO" => IMAGE_FILE_BYTES_REVERSED_LO,
                    "IMAGE_FILE_32BIT_MACHINE" => IMAGE_FILE_32BIT_MACHINE,
                    "IMAGE_FILE_DEBUG_STRIPPED" => IMAGE_FILE_DEBUG_STRIPPED,
                    "IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP" => IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP,
                    "IMAGE_FILE_NET_RUN_FROM_SWAP" => IMAGE_FILE_NET_RUN_FROM_SWAP,
                    "IMAGE_FILE_SYSTEM" => IMAGE_FILE_SYSTEM,
                    "IMAGE_FILE_DLL" => IMAGE_FILE_DLL,
                    "IMAGE_FILE_UP_SYSTEM_ONLY" => IMAGE_FILE_UP_SYSTEM_ONLY,
                    "IMAGE_FILE_BYTES_REVERSED_HI" => IMAGE_FILE_BYTES_REVERSED_HI,
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid header characteristic value {val}"
                        )));
                    }
                }
            }

            Ok(flags)
        }
    }

    deserializer.deserialize_any(CharacteristicsVisitor)
}

fn characteristics_serializer<S>(characteristics: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    const FLAGS: [(u16, &str); 15] = [
        (IMAGE_FILE_RELOCS_STRIPPED, "IMAGE_FILE_RELOCS_STRIPPED"),
        (IMAGE_FILE_EXECUTABLE_IMAGE, "IMAGE_FILE_EXECUTABLE_IMAGE"),
        (
            IMAGE_FILE_LINE_NUMS_STRIPPED,
            "IMAGE_FILE_LINE_NUMS_STRIPPED",
        ),
        (
            IMAGE_FILE_LOCAL_SYMS_STRIPPED,
            "IMAGE_FILE_LOCAL_SYMS_STRIPPED",
        ),
        (
            IMAGE_FILE_AGGRESIVE_WS_TRIM,
            "IMAGE_FILE_AGGRESSIVE_WS_TRIM",
        ),
        (
            IMAGE_FILE_LARGE_ADDRESS_AWARE,
            "IMAGE_FILE_LARGE_ADDRESS_AWARE",
        ),
        (IMAGE_FILE_BYTES_REVERSED_LO, "IMAGE_FILE_BYTES_REVERSED_LO"),
        (IMAGE_FILE_32BIT_MACHINE, "IMAGE_FILE_32BIT_MACHINE"),
        (IMAGE_FILE_DEBUG_STRIPPED, "IMAGE_FILE_DEBUG_STRIPPED"),
        (
            IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP,
            "IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP",
        ),
        (IMAGE_FILE_NET_RUN_FROM_SWAP, "IMAGE_FILE_NET_RUN_FROM_SWAP"),
        (IMAGE_FILE_SYSTEM, "IMAGE_FILE_SYSTEM"),
        (IMAGE_FILE_DLL, "IMAGE_FILE_DLL"),
        (IMAGE_FILE_UP_SYSTEM_ONLY, "IMAGE_FILE_UP_SYSTEM_ONLY"),
        (IMAGE_FILE_BYTES_REVERSED_HI, "IMAGE_FILE_BYTES_REVERSED_HI"),
    ];

    let contains_flag = |bits: u16, flag: u16| bits & flag != 0;

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

#[cfg(test)]
mod tests {
    use super::{CoffYamlHeader, characteristics_deserializer, machine_deserializer};
    use crate::testutils;
    use object::pe::{
        IMAGE_FILE_LINE_NUMS_STRIPPED, IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386,
        IMAGE_FILE_MACHINE_UNKNOWN, IMAGE_FILE_RELOCS_STRIPPED,
    };
    use serde::Deserialize;

    #[test]
    fn machine_scalar_deserialize() {
        testutils::run_deserializer_tests(
            machine_deserializer,
            [
                ("34404", IMAGE_FILE_MACHINE_AMD64),
                ("0x8664", IMAGE_FILE_MACHINE_AMD64),
                ("332", IMAGE_FILE_MACHINE_I386),
                ("0x14c", IMAGE_FILE_MACHINE_I386),
                ("0", IMAGE_FILE_MACHINE_UNKNOWN),
            ],
        );
    }

    #[test]
    fn machine_string_deserialize() {
        testutils::run_deserializer_tests(
            machine_deserializer,
            [
                ("IMAGE_FILE_MACHINE_AMD64", IMAGE_FILE_MACHINE_AMD64),
                ("IMAGE_FILE_MACHINE_I386", IMAGE_FILE_MACHINE_I386),
                ("IMAGE_FILE_MACHINE_UNKNOWN", IMAGE_FILE_MACHINE_UNKNOWN),
            ],
        );
    }

    #[test]
    fn characteristics_scalar_deserialize() {
        testutils::run_deserializer_tests(
            characteristics_deserializer,
            [
                ("1", IMAGE_FILE_RELOCS_STRIPPED),
                (
                    "5",
                    IMAGE_FILE_RELOCS_STRIPPED | IMAGE_FILE_LINE_NUMS_STRIPPED,
                ),
                ("0x1", IMAGE_FILE_RELOCS_STRIPPED),
            ],
        );
    }

    #[test]
    fn characteristics_string_deserialize() {
        testutils::run_deserializer_tests(
            characteristics_deserializer,
            [
                ("[ IMAGE_FILE_RELOCS_STRIPPED ]", IMAGE_FILE_RELOCS_STRIPPED),
                (
                    "[ IMAGE_FILE_RELOCS_STRIPPED, IMAGE_FILE_LINE_NUMS_STRIPPED ]",
                    IMAGE_FILE_RELOCS_STRIPPED | IMAGE_FILE_LINE_NUMS_STRIPPED,
                ),
            ],
        );
    }

    #[test]
    fn header_deserialize() {
        testutils::run_deserializer_tests(
            CoffYamlHeader::deserialize,
            [
                (
                    r#"
                Machine: IMAGE_FILE_MACHINE_UNKNOWN
                Characteristics: [ IMAGE_FILE_RELOCS_STRIPPED ]
                "#,
                    CoffYamlHeader {
                        machine: IMAGE_FILE_MACHINE_UNKNOWN,
                        characteristics: IMAGE_FILE_RELOCS_STRIPPED,
                    },
                ),
                (
                    r#"
                Machine: IMAGE_FILE_MACHINE_AMD64
                Characteristics: [ IMAGE_FILE_RELOCS_STRIPPED, IMAGE_FILE_LINE_NUMS_STRIPPED ]
                "#,
                    CoffYamlHeader {
                        machine: IMAGE_FILE_MACHINE_AMD64,
                        characteristics: IMAGE_FILE_RELOCS_STRIPPED | IMAGE_FILE_LINE_NUMS_STRIPPED,
                    },
                ),
            ],
        );
    }
}
