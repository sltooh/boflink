use object::pe::{
    IMAGE_COMDAT_SELECT_ANY, IMAGE_COMDAT_SELECT_ASSOCIATIVE, IMAGE_COMDAT_SELECT_EXACT_MATCH,
    IMAGE_COMDAT_SELECT_LARGEST, IMAGE_COMDAT_SELECT_NODUPLICATES, IMAGE_COMDAT_SELECT_SAME_SIZE,
    IMAGE_SYM_ABSOLUTE, IMAGE_SYM_CLASS_ARGUMENT, IMAGE_SYM_CLASS_AUTOMATIC,
    IMAGE_SYM_CLASS_BIT_FIELD, IMAGE_SYM_CLASS_BLOCK, IMAGE_SYM_CLASS_CLR_TOKEN,
    IMAGE_SYM_CLASS_END_OF_FUNCTION, IMAGE_SYM_CLASS_END_OF_STRUCT, IMAGE_SYM_CLASS_ENUM_TAG,
    IMAGE_SYM_CLASS_EXTERNAL, IMAGE_SYM_CLASS_EXTERNAL_DEF, IMAGE_SYM_CLASS_FILE,
    IMAGE_SYM_CLASS_FUNCTION, IMAGE_SYM_CLASS_LABEL, IMAGE_SYM_CLASS_MEMBER_OF_ENUM,
    IMAGE_SYM_CLASS_MEMBER_OF_STRUCT, IMAGE_SYM_CLASS_MEMBER_OF_UNION, IMAGE_SYM_CLASS_NULL,
    IMAGE_SYM_CLASS_REGISTER, IMAGE_SYM_CLASS_REGISTER_PARAM, IMAGE_SYM_CLASS_SECTION,
    IMAGE_SYM_CLASS_STATIC, IMAGE_SYM_CLASS_STRUCT_TAG, IMAGE_SYM_CLASS_TYPE_DEFINITION,
    IMAGE_SYM_CLASS_UNDEFINED_LABEL, IMAGE_SYM_CLASS_UNDEFINED_STATIC, IMAGE_SYM_CLASS_UNION_TAG,
    IMAGE_SYM_CLASS_WEAK_EXTERNAL, IMAGE_SYM_DEBUG, IMAGE_SYM_DTYPE_ARRAY,
    IMAGE_SYM_DTYPE_FUNCTION, IMAGE_SYM_DTYPE_NULL, IMAGE_SYM_DTYPE_POINTER, IMAGE_SYM_TYPE_BYTE,
    IMAGE_SYM_TYPE_CHAR, IMAGE_SYM_TYPE_DOUBLE, IMAGE_SYM_TYPE_DWORD, IMAGE_SYM_TYPE_ENUM,
    IMAGE_SYM_TYPE_FLOAT, IMAGE_SYM_TYPE_INT, IMAGE_SYM_TYPE_LONG, IMAGE_SYM_TYPE_MOE,
    IMAGE_SYM_TYPE_NULL, IMAGE_SYM_TYPE_SHORT, IMAGE_SYM_TYPE_STRUCT, IMAGE_SYM_TYPE_UINT,
    IMAGE_SYM_TYPE_UNION, IMAGE_SYM_TYPE_VOID, IMAGE_SYM_TYPE_WORD, IMAGE_SYM_UNDEFINED,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
use serde_yml::with::singleton_map_optional;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CoffYamlSymbol {
    pub name: String,
    pub value: u32,

    #[serde(
        deserialize_with = "section_number_deserializer",
        serialize_with = "section_number_serializer"
    )]
    pub section_number: i32,

    #[serde(
        deserialize_with = "simple_type_deserializer",
        serialize_with = "simple_type_serializer"
    )]
    pub simple_type: u16,

    #[serde(
        deserialize_with = "complex_type_deserializer",
        serialize_with = "complex_type_serializer"
    )]
    pub complex_type: u16,

    #[serde(
        deserialize_with = "storage_class_deserializer",
        serialize_with = "storage_class_serializer"
    )]
    pub storage_class: u8,

    #[serde(
        default,
        with = "singleton_map_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub section_definition: Option<CoffYamlAuxSectionDefinition>,

    #[serde(
        default,
        with = "singleton_map_optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_definition: Option<CoffYamlAuxFunctionDefinition>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

fn section_number_deserializer<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    struct SectionNumberVisitor;

    impl Visitor<'_> for SectionNumberVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(
                "IMAGE_SYM_UNDEFINED, IMAGE_SYM_ABSOLUTE, IMAGE_SYM_DEBUG string or integer",
            )
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            i32::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            i32::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(match v {
                "IMAGE_SYM_UNDEFINED" => IMAGE_SYM_UNDEFINED,
                "IMAGE_SYM_ABSOLUTE" => IMAGE_SYM_ABSOLUTE,
                "IMAGE_SYM_DEBUG" => IMAGE_SYM_DEBUG,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid section number string {v}"
                    )));
                }
            })
        }
    }

    deserializer.deserialize_any(SectionNumberVisitor)
}

fn section_number_serializer<S>(section_number: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *section_number {
        IMAGE_SYM_UNDEFINED => serializer.serialize_str("IMAGE_SYM_UNDEFINED"),
        IMAGE_SYM_ABSOLUTE => serializer.serialize_str("IMAGE_SYM_ABSOLUTE"),
        IMAGE_SYM_DEBUG => serializer.serialize_str("IMAGE_SYM_DEBUG"),
        o if o >= 1 => serializer.serialize_i32(o),
        _ => Err(serde::ser::Error::custom(format!(
            "invalid section number {section_number}"
        ))),
    }
}

fn simple_type_deserializer<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    struct SimpleTypeVisitor;

    impl Visitor<'_> for SimpleTypeVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_SYM_TYPE_*' string or integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u16::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(match v {
                "IMAGE_SYM_TYPE_NULL" => IMAGE_SYM_TYPE_NULL,
                "IMAGE_SYM_TYPE_VOID" => IMAGE_SYM_TYPE_VOID,
                "IMAGE_SYM_TYPE_CHAR" => IMAGE_SYM_TYPE_CHAR,
                "IMAGE_SYM_TYPE_SHORT" => IMAGE_SYM_TYPE_SHORT,
                "IMAGE_SYM_TYPE_INT" => IMAGE_SYM_TYPE_INT,
                "IMAGE_SYM_TYPE_LONG" => IMAGE_SYM_TYPE_LONG,
                "IMAGE_SYM_TYPE_FLOAT" => IMAGE_SYM_TYPE_FLOAT,
                "IMAGE_SYM_TYPE_DOUBLE" => IMAGE_SYM_TYPE_DOUBLE,
                "IMAGE_SYM_TYPE_STRUCT" => IMAGE_SYM_TYPE_STRUCT,
                "IMAGE_SYM_TYPE_UNION" => IMAGE_SYM_TYPE_UNION,
                "IMAGE_SYM_TYPE_ENUM" => IMAGE_SYM_TYPE_ENUM,
                "IMAGE_SYM_TYPE_MOE" => IMAGE_SYM_TYPE_MOE,
                "IMAGE_SYM_TYPE_BYTE" => IMAGE_SYM_TYPE_BYTE,
                "IMAGE_SYM_TYPE_WORD" => IMAGE_SYM_TYPE_WORD,
                "IMAGE_SYM_TYPE_UINT" => IMAGE_SYM_TYPE_UINT,
                "IMAGE_SYM_TYPE_DWORD" => IMAGE_SYM_TYPE_DWORD,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid symbol simple type {v}"
                    )));
                }
            })
        }
    }

    deserializer.deserialize_any(SimpleTypeVisitor)
}

fn simple_type_serializer<S>(simple_type: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *simple_type {
        IMAGE_SYM_TYPE_NULL => serializer.serialize_str("IMAGE_SYM_TYPE_NULL"),
        IMAGE_SYM_TYPE_VOID => serializer.serialize_str("IMAGE_SYM_TYPE_VOID"),
        IMAGE_SYM_TYPE_CHAR => serializer.serialize_str("IMAGE_SYM_TYPE_CHAR"),
        IMAGE_SYM_TYPE_SHORT => serializer.serialize_str("IMAGE_SYM_TYPE_SHORT"),
        IMAGE_SYM_TYPE_INT => serializer.serialize_str("IMAGE_SYM_TYPE_INT"),
        IMAGE_SYM_TYPE_LONG => serializer.serialize_str("IMAGE_SYM_TYPE_LONG"),
        IMAGE_SYM_TYPE_FLOAT => serializer.serialize_str("IMAGE_SYM_TYPE_FLOAT"),
        IMAGE_SYM_TYPE_DOUBLE => serializer.serialize_str("IMAGE_SYM_TYPE_DOUBLE"),
        IMAGE_SYM_TYPE_STRUCT => serializer.serialize_str("IMAGE_SYM_TYPE_STRUCT"),
        IMAGE_SYM_TYPE_UNION => serializer.serialize_str("IMAGE_SYM_TYPE_UNION"),
        IMAGE_SYM_TYPE_ENUM => serializer.serialize_str("IMAGE_SYM_TYPE_ENUM"),
        IMAGE_SYM_TYPE_MOE => serializer.serialize_str("IMAGE_SYM_TYPE_MOE"),
        IMAGE_SYM_TYPE_BYTE => serializer.serialize_str("IMAGE_SYM_TYPE_BYTE"),
        IMAGE_SYM_TYPE_WORD => serializer.serialize_str("IMAGE_SYM_TYPE_WORD"),
        IMAGE_SYM_TYPE_UINT => serializer.serialize_str("IMAGE_SYM_TYPE_UINT"),
        IMAGE_SYM_TYPE_DWORD => serializer.serialize_str("IMAGE_SYM_TYPE_DWORD"),
        o => serializer.serialize_u16(o),
    }
}

fn complex_type_deserializer<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    struct ComplexTypeVisitor;

    impl Visitor<'_> for ComplexTypeVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_SYM_DTYPE_*' string or integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u16::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(match v {
                "IMAGE_SYM_DTYPE_NULL" => IMAGE_SYM_DTYPE_NULL,
                "IMAGE_SYM_DTYPE_POINTER" => IMAGE_SYM_DTYPE_POINTER,
                "IMAGE_SYM_DTYPE_FUNCTION" => IMAGE_SYM_DTYPE_FUNCTION,
                "IMAGE_SYM_DTYPE_ARRAY" => IMAGE_SYM_DTYPE_ARRAY,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid symbol complex type {v}"
                    )));
                }
            })
        }
    }

    deserializer.deserialize_any(ComplexTypeVisitor)
}

fn complex_type_serializer<S>(complex_type: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *complex_type {
        IMAGE_SYM_DTYPE_NULL => serializer.serialize_str("IMAGE_SYM_DTYPE_NULL"),
        IMAGE_SYM_DTYPE_POINTER => serializer.serialize_str("IMAGE_SYM_DTYPE_POINTER"),
        IMAGE_SYM_DTYPE_FUNCTION => serializer.serialize_str("IMAGE_SYM_DTYPE_FUNCTION"),
        IMAGE_SYM_DTYPE_ARRAY => serializer.serialize_str("IMAGE_SYM_DTYPE_ARRAY"),
        o => serializer.serialize_u16(o),
    }
}

fn storage_class_deserializer<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    struct StorageClassVisitor;

    impl Visitor<'_> for StorageClassVisitor {
        type Value = u8;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_SYM_CLASS_*' string or integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u8::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(i8::try_from(v).map_err(serde::de::Error::custom)? as u8)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(match v {
                "IMAGE_SYM_CLASS_END_OF_FUNCTION" => IMAGE_SYM_CLASS_END_OF_FUNCTION,
                "IMAGE_SYM_CLASS_NULL" => IMAGE_SYM_CLASS_NULL,
                "IMAGE_SYM_CLASS_AUTOMATIC" => IMAGE_SYM_CLASS_AUTOMATIC,
                "IMAGE_SYM_CLASS_EXTERNAL" => IMAGE_SYM_CLASS_EXTERNAL,
                "IMAGE_SYM_CLASS_STATIC" => IMAGE_SYM_CLASS_STATIC,
                "IMAGE_SYM_CLASS_REGISTER" => IMAGE_SYM_CLASS_REGISTER,
                "IMAGE_SYM_CLASS_EXTERNAL_DEF" => IMAGE_SYM_CLASS_EXTERNAL_DEF,
                "IMAGE_SYM_CLASS_LABEL" => IMAGE_SYM_CLASS_LABEL,
                "IMAGE_SYM_CLASS_UNDEFINED_LABEL" => IMAGE_SYM_CLASS_UNDEFINED_LABEL,
                "IMAGE_SYM_CLASS_MEMBER_OF_STRUCT" => IMAGE_SYM_CLASS_MEMBER_OF_STRUCT,
                "IMAGE_SYM_CLASS_ARGUMENT" => IMAGE_SYM_CLASS_ARGUMENT,
                "IMAGE_SYM_CLASS_STRUCT_TAG" => IMAGE_SYM_CLASS_STRUCT_TAG,
                "IMAGE_SYM_CLASS_MEMBER_OF_UNION" => IMAGE_SYM_CLASS_MEMBER_OF_UNION,
                "IMAGE_SYM_CLASS_UNION_TAG" => IMAGE_SYM_CLASS_UNION_TAG,
                "IMAGE_SYM_CLASS_TYPE_DEFINITION" => IMAGE_SYM_CLASS_TYPE_DEFINITION,
                "IMAGE_SYM_CLASS_UNDEFINED_STATIC" => IMAGE_SYM_CLASS_UNDEFINED_STATIC,
                "IMAGE_SYM_CLASS_ENUM_TAG" => IMAGE_SYM_CLASS_ENUM_TAG,
                "IMAGE_SYM_CLASS_MEMBER_OF_ENUM" => IMAGE_SYM_CLASS_MEMBER_OF_ENUM,
                "IMAGE_SYM_CLASS_REGISTER_PARAM" => IMAGE_SYM_CLASS_REGISTER_PARAM,
                "IMAGE_SYM_CLASS_BIT_FIELD" => IMAGE_SYM_CLASS_BIT_FIELD,
                "IMAGE_SYM_CLASS_BLOCK" => IMAGE_SYM_CLASS_BLOCK,
                "IMAGE_SYM_CLASS_FUNCTION" => IMAGE_SYM_CLASS_FUNCTION,
                "IMAGE_SYM_CLASS_END_OF_STRUCT" => IMAGE_SYM_CLASS_END_OF_STRUCT,
                "IMAGE_SYM_CLASS_FILE" => IMAGE_SYM_CLASS_FILE,
                "IMAGE_SYM_CLASS_SECTION" => IMAGE_SYM_CLASS_SECTION,
                "IMAGE_SYM_CLASS_WEAK_EXTERNAL" => IMAGE_SYM_CLASS_WEAK_EXTERNAL,
                "IMAGE_SYM_CLASS_CLR_TOKEN" => IMAGE_SYM_CLASS_CLR_TOKEN,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid symbol complex type {v}"
                    )));
                }
            })
        }
    }

    deserializer.deserialize_any(StorageClassVisitor)
}

fn storage_class_serializer<S>(storage_class: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *storage_class {
        IMAGE_SYM_CLASS_END_OF_FUNCTION => {
            serializer.serialize_str("IMAGE_SYM_CLASS_END_OF_FUNCTION")
        }
        IMAGE_SYM_CLASS_NULL => serializer.serialize_str("IMAGE_SYM_CLASS_NULL"),
        IMAGE_SYM_CLASS_AUTOMATIC => serializer.serialize_str("IMAGE_SYM_CLASS_AUTOMATIC"),
        IMAGE_SYM_CLASS_EXTERNAL => serializer.serialize_str("IMAGE_SYM_CLASS_EXTERNAL"),
        IMAGE_SYM_CLASS_STATIC => serializer.serialize_str("IMAGE_SYM_CLASS_STATIC"),
        IMAGE_SYM_CLASS_REGISTER => serializer.serialize_str("IMAGE_SYM_CLASS_REGISTER"),
        IMAGE_SYM_CLASS_EXTERNAL_DEF => serializer.serialize_str("IMAGE_SYM_CLASS_EXTERNAL_DEF"),
        IMAGE_SYM_CLASS_LABEL => serializer.serialize_str("IMAGE_SYM_CLASS_LABEL"),
        IMAGE_SYM_CLASS_UNDEFINED_LABEL => {
            serializer.serialize_str("IMAGE_SYM_CLASS_UNDEFINED_LABEL")
        }
        IMAGE_SYM_CLASS_MEMBER_OF_STRUCT => {
            serializer.serialize_str("IMAGE_SYM_CLASS_MEMBER_OF_STRUCT")
        }
        IMAGE_SYM_CLASS_ARGUMENT => serializer.serialize_str("IMAGE_SYM_CLASS_ARGUMENT"),
        IMAGE_SYM_CLASS_STRUCT_TAG => serializer.serialize_str("IMAGE_SYM_CLASS_STRUCT_TAG"),
        IMAGE_SYM_CLASS_MEMBER_OF_UNION => {
            serializer.serialize_str("IMAGE_SYM_CLASS_MEMBER_OF_UNION")
        }
        IMAGE_SYM_CLASS_UNION_TAG => serializer.serialize_str("IMAGE_SYM_CLASS_UNION_TAG"),
        IMAGE_SYM_CLASS_TYPE_DEFINITION => {
            serializer.serialize_str("IMAGE_SYM_CLASS_TYPE_DEFINITION")
        }
        IMAGE_SYM_CLASS_UNDEFINED_STATIC => {
            serializer.serialize_str("IMAGE_SYM_CLASS_UNDEFINED_STATIC")
        }
        IMAGE_SYM_CLASS_ENUM_TAG => serializer.serialize_str("IMAGE_SYM_CLASS_ENUM_TAG"),
        IMAGE_SYM_CLASS_MEMBER_OF_ENUM => {
            serializer.serialize_str("IMAGE_SYM_CLASS_MEMBER_OF_ENUM")
        }
        IMAGE_SYM_CLASS_REGISTER_PARAM => {
            serializer.serialize_str("IMAGE_SYM_CLASS_REGISTER_PARAM")
        }
        IMAGE_SYM_CLASS_BIT_FIELD => serializer.serialize_str("IMAGE_SYM_CLASS_BIT_FIELD"),
        IMAGE_SYM_CLASS_BLOCK => serializer.serialize_str("IMAGE_SYM_CLASS_BLOCK"),
        IMAGE_SYM_CLASS_FUNCTION => serializer.serialize_str("IMAGE_SYM_CLASS_FUNCTION"),
        IMAGE_SYM_CLASS_END_OF_STRUCT => serializer.serialize_str("IMAGE_SYM_CLASS_END_OF_STRUCT"),
        IMAGE_SYM_CLASS_FILE => serializer.serialize_str("IMAGE_SYM_CLASS_FILE"),
        IMAGE_SYM_CLASS_SECTION => serializer.serialize_str("IMAGE_SYM_CLASS_SECTION"),
        IMAGE_SYM_CLASS_WEAK_EXTERNAL => serializer.serialize_str("IMAGE_SYM_CLASS_WEAK_EXTERNAL"),
        IMAGE_SYM_CLASS_CLR_TOKEN => serializer.serialize_str("IMAGE_SYM_CLASS_CLR_TOKEN"),
        o => serializer.serialize_u8(o),
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CoffYamlAuxFunctionDefinition {
    pub tag_index: u32,
    pub total_size: u32,
    pub pointer_to_linenumber: u32,
    pub pointer_to_next_function: u32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CoffYamlAuxSectionDefinition {
    pub length: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub check_sum: u32,
    pub number: u16,

    #[serde(
        default,
        deserialize_with = "aux_section_comdat_selection_deserializer",
        serialize_with = "aux_section_comdat_selection_serializer"
    )]
    pub selection: u8,
}

fn aux_section_comdat_selection_deserializer<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    struct ComdatSelectionVisitor;

    impl<'de> Visitor<'de> for ComdatSelectionVisitor {
        type Value = u8;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("'IMAGE_COMDAT_SELECT_*' string or integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u8::try_from(v).map_err(serde::de::Error::custom)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(0)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(Self)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(match v {
                "IMAGE_COMDAT_SELECT_NODUPLICATES" => IMAGE_COMDAT_SELECT_NODUPLICATES,
                "IMAGE_COMDAT_SELECT_ANY" => IMAGE_COMDAT_SELECT_ANY,
                "IMAGE_COMDAT_SELECT_SAME_SIZE" => IMAGE_COMDAT_SELECT_SAME_SIZE,
                "IMAGE_COMDAT_SELECT_EXACT_MATCH" => IMAGE_COMDAT_SELECT_EXACT_MATCH,
                "IMAGE_COMDAT_SELECT_ASSOCIATIVE" => IMAGE_COMDAT_SELECT_ASSOCIATIVE,
                "IMAGE_COMDAT_SELECT_LARGEST" => IMAGE_COMDAT_SELECT_LARGEST,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid COMDAT selection type {v}"
                    )));
                }
            })
        }
    }

    deserializer.deserialize_any(ComdatSelectionVisitor)
}

fn aux_section_comdat_selection_serializer<S>(
    selection: &u8,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *selection {
        IMAGE_COMDAT_SELECT_NODUPLICATES => {
            serializer.serialize_str("IMAGE_COMDAT_SELECT_NODUPLICATES")
        }
        IMAGE_COMDAT_SELECT_ANY => serializer.serialize_str("IMAGE_COMDAT_SELECT_ANY"),
        IMAGE_COMDAT_SELECT_SAME_SIZE => serializer.serialize_str("IMAGE_COMDAT_SELECT_SAME_SIZE"),
        IMAGE_COMDAT_SELECT_EXACT_MATCH => {
            serializer.serialize_str("IMAGE_COMDAT_SELECT_EXACT_MATCH")
        }
        IMAGE_COMDAT_SELECT_ASSOCIATIVE => {
            serializer.serialize_str("IMAGE_COMDAT_SELECT_ASSOCIATIVE")
        }
        IMAGE_COMDAT_SELECT_LARGEST => serializer.serialize_str("IMAGE_COMDAT_SELECT_LARGEST"),
        o => serializer.serialize_u8(o),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CoffYamlAuxFunctionDefinition, CoffYamlSymbol, aux_section_comdat_selection_deserializer,
        complex_type_deserializer, section_number_deserializer, simple_type_deserializer,
        storage_class_deserializer,
    };
    use crate::testutils;
    use object::pe::{
        IMAGE_COMDAT_SELECT_ANY, IMAGE_COMDAT_SELECT_LARGEST, IMAGE_COMDAT_SELECT_NODUPLICATES,
        IMAGE_SYM_ABSOLUTE, IMAGE_SYM_CLASS_END_OF_FUNCTION, IMAGE_SYM_CLASS_EXTERNAL,
        IMAGE_SYM_CLASS_NULL, IMAGE_SYM_CLASS_STATIC, IMAGE_SYM_DEBUG, IMAGE_SYM_DTYPE_FUNCTION,
        IMAGE_SYM_DTYPE_NULL, IMAGE_SYM_TYPE_NULL, IMAGE_SYM_TYPE_VOID, IMAGE_SYM_UNDEFINED,
    };
    use serde::Deserialize;

    #[test]
    fn section_number_scalar_deserialize() {
        testutils::run_deserializer_tests(
            section_number_deserializer,
            [("0", 0), ("1", 1), ("-1", -1), ("-2", -2)],
        );
    }

    #[test]
    fn section_number_string_deserialize() {
        testutils::run_deserializer_tests(
            section_number_deserializer,
            [
                ("IMAGE_SYM_UNDEFINED", IMAGE_SYM_UNDEFINED),
                ("IMAGE_SYM_ABSOLUTE", IMAGE_SYM_ABSOLUTE),
                ("IMAGE_SYM_DEBUG", IMAGE_SYM_DEBUG),
            ],
        );
    }

    #[test]
    fn simple_type_scalar_deserialize() {
        testutils::run_deserializer_tests(
            simple_type_deserializer,
            [("0", IMAGE_SYM_TYPE_NULL), ("1", IMAGE_SYM_TYPE_VOID)],
        );
    }

    #[test]
    fn simple_type_string_deserialize() {
        testutils::run_deserializer_tests(
            simple_type_deserializer,
            [
                ("IMAGE_SYM_TYPE_NULL", IMAGE_SYM_TYPE_NULL),
                ("IMAGE_SYM_TYPE_VOID", IMAGE_SYM_TYPE_VOID),
            ],
        );
    }

    #[test]
    fn complex_type_scalar_deserialize() {
        testutils::run_deserializer_tests(
            complex_type_deserializer,
            [("0", IMAGE_SYM_DTYPE_NULL), ("2", IMAGE_SYM_DTYPE_FUNCTION)],
        );
    }

    #[test]
    fn complex_type_string_deserialize() {
        testutils::run_deserializer_tests(
            complex_type_deserializer,
            [
                ("IMAGE_SYM_DTYPE_NULL", IMAGE_SYM_DTYPE_NULL),
                ("IMAGE_SYM_DTYPE_FUNCTION", IMAGE_SYM_DTYPE_FUNCTION),
            ],
        );
    }

    #[test]
    fn storage_class_scalar_deserialize() {
        testutils::run_deserializer_tests(
            storage_class_deserializer,
            [
                ("-1", IMAGE_SYM_CLASS_END_OF_FUNCTION),
                ("0", IMAGE_SYM_CLASS_NULL),
                ("2", IMAGE_SYM_CLASS_EXTERNAL),
                ("3", IMAGE_SYM_CLASS_STATIC),
            ],
        );
    }

    #[test]
    fn storage_class_string_deserialize() {
        testutils::run_deserializer_tests(
            storage_class_deserializer,
            [
                (
                    "IMAGE_SYM_CLASS_END_OF_FUNCTION",
                    IMAGE_SYM_CLASS_END_OF_FUNCTION,
                ),
                ("IMAGE_SYM_CLASS_NULL", IMAGE_SYM_CLASS_NULL),
                ("IMAGE_SYM_CLASS_EXTERNAL", IMAGE_SYM_CLASS_EXTERNAL),
                ("IMAGE_SYM_CLASS_STATIC", IMAGE_SYM_CLASS_STATIC),
            ],
        );
    }

    #[test]
    fn aux_section_comdat_selection_scalar_deserialize() {
        testutils::run_deserializer_tests(
            aux_section_comdat_selection_deserializer,
            [
                ("1", IMAGE_COMDAT_SELECT_NODUPLICATES),
                ("2", IMAGE_COMDAT_SELECT_ANY),
                ("6", IMAGE_COMDAT_SELECT_LARGEST),
            ],
        );
    }

    #[test]
    fn aux_section_comdat_selection_string_deserialize() {
        testutils::run_deserializer_tests(
            aux_section_comdat_selection_deserializer,
            [
                (
                    "IMAGE_COMDAT_SELECT_NODUPLICATES",
                    IMAGE_COMDAT_SELECT_NODUPLICATES,
                ),
                ("IMAGE_COMDAT_SELECT_ANY", IMAGE_COMDAT_SELECT_ANY),
                ("IMAGE_COMDAT_SELECT_LARGEST", IMAGE_COMDAT_SELECT_LARGEST),
            ],
        );
    }

    #[test]
    fn symbol_no_aux_deserialize() {
        testutils::run_deserializer_tests(
            CoffYamlSymbol::deserialize,
            [
                (
                    r#"
                Name: all_scalars
                Value: 0
                SectionNumber: 1
                SimpleType: 2
                ComplexType: 3
                StorageClass: 4
                "#,
                    CoffYamlSymbol {
                        name: "all_scalars".into(),
                        value: 0,
                        section_number: 1,
                        simple_type: 2,
                        complex_type: 3,
                        storage_class: 4,
                        function_definition: None,
                        section_definition: None,
                        file: None,
                    },
                ),
                (
                    r#"
                Name: all_strings
                Value: 0
                SectionNumber: IMAGE_SYM_DEBUG
                SimpleType: IMAGE_SYM_TYPE_VOID
                ComplexType: IMAGE_SYM_DTYPE_FUNCTION
                StorageClass: IMAGE_SYM_CLASS_EXTERNAL
                "#,
                    CoffYamlSymbol {
                        name: "all_strings".into(),
                        value: 0,
                        section_number: IMAGE_SYM_DEBUG,
                        simple_type: IMAGE_SYM_TYPE_VOID,
                        complex_type: IMAGE_SYM_DTYPE_FUNCTION,
                        storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                        function_definition: None,
                        section_definition: None,
                        file: None,
                    },
                ),
            ],
        );
    }

    #[test]
    fn symbol_with_aux_deserialize() {
        testutils::run_deserializer_tests(
            CoffYamlSymbol::deserialize,
            [
                (
                    r#"
            Name: aux_section
            Value: 0
            SectionNumber: 1
            SimpleType: 2
            ComplexType: 3
            StorageClass: 3
            SectionDefinition:
              Length: 123
              NumberOfRelocations: 2
              NumberOfLinenumbers: 0
              CheckSum: 0
              Number: 0
            "#,
                    CoffYamlSymbol {
                        name: "aux_section".into(),
                        value: 0,
                        section_number: 1,
                        simple_type: 2,
                        complex_type: 3,
                        storage_class: 3,
                        section_definition: Some(super::CoffYamlAuxSectionDefinition {
                            length: 123,
                            number_of_relocations: 2,
                            number_of_linenumbers: 0,
                            check_sum: 0,
                            number: 0,
                            selection: 0,
                        }),
                        function_definition: None,
                        file: None,
                    },
                ),
                (
                    r#"
            Name: aux_function
            Value: 2
            SectionNumber: 23
            SimpleType: 4
            ComplexType: IMAGE_SYM_DTYPE_FUNCTION
            StorageClass: 2
            FunctionDefinition:
              TagIndex: 0
              TotalSize: 1
              PointerToLinenumber: 2
              PointerToNextFunction: 3
            "#,
                    CoffYamlSymbol {
                        name: "aux_function".into(),
                        value: 2,
                        section_number: 23,
                        simple_type: 4,
                        complex_type: IMAGE_SYM_DTYPE_FUNCTION,
                        storage_class: 2,
                        function_definition: Some(CoffYamlAuxFunctionDefinition {
                            tag_index: 0,
                            total_size: 1,
                            pointer_to_linenumber: 2,
                            pointer_to_next_function: 3,
                        }),
                        section_definition: None,
                        file: None,
                    },
                ),
                (
                    r#"
            Name: aux_file
            Value: 3
            SectionNumber: 11
            SimpleType: IMAGE_SYM_TYPE_VOID
            ComplexType: IMAGE_SYM_DTYPE_NULL
            StorageClass: 5
            File: test.c
            "#,
                    CoffYamlSymbol {
                        name: "aux_file".into(),
                        value: 3,
                        section_number: 11,
                        simple_type: IMAGE_SYM_TYPE_VOID,
                        complex_type: IMAGE_SYM_DTYPE_NULL,
                        storage_class: 5,
                        file: Some("test.c".into()),
                        section_definition: None,
                        function_definition: None,
                    },
                ),
            ],
        );
    }
}
