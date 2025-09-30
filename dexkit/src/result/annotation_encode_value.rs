use std::any::Any;
use std::fmt::Debug;
use std::str::{from_utf8, from_utf8_unchecked};

use flatbuffers::{UOffsetT, Verifier, VerifierOptions, read_scalar_at};

use crate::DexkitBridge;
use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationEncodeValueMeta as FBAnnotationEncodeValueMeta,
    AnnotationEncodeValueType as FBAnnotationEncodeValueType,
    EncodeValueString as FBEncodeValueString,
};
use crate::query::enums::AnnotationEncodeValueType;
use crate::query::matchers::EncodeValue;
use crate::result::{AnnotationData, AnnotationEncodeArrayData, ClassData, FieldData, MethodData};
use crate::uitls::{MUtf8, StringUnicodeEncoderDecoder};

#[derive(Debug, Clone)]
pub enum EncodeValueData<'a> {
    Byte(i8),
    Short(i16),
    Char(char),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Type(ClassData<'a>),
    Method(MethodData<'a>),
    Enum(FieldData<'a>),
    Array(AnnotationEncodeArrayData<'a>),
    Annotation(AnnotationData<'a>),
    Null,
    Bool(bool),
}

#[derive(Clone)]
pub struct AnnotationEncodeValue<'a> {
    value: EncodeValueData<'a>,
    value_type: AnnotationEncodeValueType,
}

impl<'a> Debug for AnnotationEncodeValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotationEncodeValue")
            .field("value_type", &self.value_type)
            .field("value", &self.value)
            .finish()
    }
}

impl<'a> Default for AnnotationEncodeValue<'a> {
    fn default() -> Self {
        Self {
            value: EncodeValueData::Null,
            value_type: AnnotationEncodeValueType::NullValue,
        }
    }
}

impl<'a> AnnotationEncodeValue<'a> {
    /// Get the type of the encoded value.
    pub fn value_type(&self) -> AnnotationEncodeValueType {
        self.value_type
    }

    /// Get the encoded value.
    pub fn value(&self) -> &EncodeValueData<'a> {
        &self.value
    }

    /// If the encoded value is of type `Byte`, return the byte value; otherwise, return `None`.
    pub fn byte_value(&self) -> Option<i8> {
        match &self.value {
            EncodeValueData::Byte(b) => Some(*b),
            _ => None,
        }
    }

    /// If the encoded value is of type `Short`, return the short value; otherwise, return `None`.
    pub fn short_value(&self) -> Option<i16> {
        match &self.value {
            EncodeValueData::Short(s) => Some(*s),
            _ => None,
        }
    }

    /// If the encoded value is of type `Char`, return the character value; otherwise, return `None`.
    pub fn char_value(&self) -> Option<char> {
        match &self.value {
            EncodeValueData::Char(c) => Some(*c),
            _ => None,
        }
    }

    /// If the encoded value is of type `Int`, return the integer value; otherwise, return `None`.
    pub fn int_value(&self) -> Option<i32> {
        match &self.value {
            EncodeValueData::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// If the encoded value is of type `Long`, return the long value; otherwise, return `None`.
    pub fn long_value(&self) -> Option<i64> {
        match &self.value {
            EncodeValueData::Long(l) => Some(*l),
            _ => None,
        }
    }

    /// If the encoded value is of type `Float`, return the float value; otherwise, return `None`.
    pub fn float_value(&self) -> Option<f32> {
        match &self.value {
            EncodeValueData::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// If the encoded value is of type `Double`, return the double value; otherwise, return `None`.
    pub fn double_value(&self) -> Option<f64> {
        match &self.value {
            EncodeValueData::Double(d) => Some(*d),
            _ => None,
        }
    }

    /// If the encoded value is of type `String`, return the string value; otherwise, return `None`.
    pub fn string_value(&self) -> Option<&str> {
        match &self.value {
            EncodeValueData::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// If the encoded value is of type `Type`, return the class data; otherwise, return `None`.
    pub fn type_value(&self) -> Option<ClassData<'a>> {
        match &self.value {
            EncodeValueData::Type(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// If the encoded value is of type `Method`, return the method data; otherwise, return `None`.
    pub fn method_value(&self) -> Option<MethodData<'a>> {
        match &self.value {
            EncodeValueData::Method(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// If the encoded value is of type `Enum`, return the field data; otherwise, return `None`.
    pub fn enum_value(&self) -> Option<FieldData<'a>> {
        match &self.value {
            EncodeValueData::Enum(e) => Some(e.clone()),
            _ => None,
        }
    }

    /// If the encoded value is of type `Array`, return the annotation encode array data; otherwise, return `None`.
    pub fn array_value(&self) -> Option<AnnotationEncodeArrayData<'a>> {
        match &self.value {
            EncodeValueData::Array(a) => Some(a.clone()),
            _ => None,
        }
    }

    /// If the encoded value is of type `Annotation`, return the annotation data; otherwise, return `None`.
    pub fn annotation_value(&self) -> Option<AnnotationData<'a>> {
        match &self.value {
            EncodeValueData::Annotation(a) => Some(a.clone()),
            _ => None,
        }
    }

    /// If the encoded value is of type `Bool`, return the boolean value; otherwise, return `None`.
    pub fn bool_value(&self) -> Option<bool> {
        match &self.value {
            EncodeValueData::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Check if the encoded value is `Null`.
    pub fn is_null_value(&self) -> bool {
        matches!(self.value, EncodeValueData::Null)
    }

    /// ...
    pub(crate) fn with_meta(
        bridge: &'a DexkitBridge,
        meta: FBAnnotationEncodeValueMeta<'a>,
    ) -> AnnotationEncodeValue<'a> {
        let meta_value_tpye: AnnotationEncodeValueType = meta.type_().into();
        let value = match meta_value_tpye {
            AnnotationEncodeValueType::ByteValue => {
                let byte_value = meta
                    .value_as_encode_value_byte()
                    .map(|v| v.value())
                    .unwrap_or(0);
                EncodeValueData::Byte(byte_value)
            }

            AnnotationEncodeValueType::ShortValue => {
                let short_value = meta
                    .value_as_encode_value_short()
                    .map(|v| v.value())
                    .unwrap_or(0);
                EncodeValueData::Short(short_value)
            }

            AnnotationEncodeValueType::CharValue => {
                let char_value = meta
                    .value_as_encode_value_char()
                    .map(|v| v.value())
                    .unwrap_or(0);
                EncodeValueData::Char(char::from_u32(char_value as u32).unwrap_or('\0'))
            }

            AnnotationEncodeValueType::IntValue => {
                let int_value = meta
                    .value_as_encode_value_int()
                    .map(|v| v.value())
                    .unwrap_or(0);
                EncodeValueData::Int(int_value)
            }

            AnnotationEncodeValueType::LongValue => {
                let long_value = meta
                    .value_as_encode_value_long()
                    .map(|v| v.value())
                    .unwrap_or(0);
                EncodeValueData::Long(long_value)
            }

            AnnotationEncodeValueType::FloatValue => {
                let float_value = meta
                    .value_as_encode_value_float()
                    .map(|v| v.value())
                    .unwrap_or(0.0);
                EncodeValueData::Float(float_value)
            }

            AnnotationEncodeValueType::DoubleValue => {
                let double_value = meta
                    .value_as_encode_value_double()
                    .map(|v| v.value())
                    .unwrap_or(0.0);
                EncodeValueData::Double(double_value)
            }

            AnnotationEncodeValueType::StringValue => {
                // let string_value = meta
                //     .value_as_encode_value_string()
                //     .map(|encode_value| encode_value.value())
                //     .map(|s| s.map(|s| s).unwrap_or_default())
                //     .unwrap_or_default();
                // EncodeValueData::String(string_value.to_string())

                // --------------------------------
                // The above code cannot correctly parse the protocol in `@kotlin.Metadata`, such as: `\u0000\u00e2\u0002\n\u0002...`
                // --------------------------------

                // custom parse string. details logic see below: `encode_value.value()`
                let encode_value_string = meta
                    .value_as_encode_value_string()
                    .map(|encode_value| encode_value._tab)
                    .map(|tab| {
                        let o = tab.vtable().get(FBEncodeValueString::VT_VALUE) as usize;
                        if o == 0 {
                            return String::new();
                        }

                        let buf = tab.buf();
                        let loc = tab.loc() + o;
                        unsafe {
                            // `ForwardsUOffset.follow()` details logic:
                            // https://github.com/google/flatbuffers/blob/27325e002a36b3de2999906a77ff13a14fb09471/rust/flatbuffers/src/primitives.rs#L174
                            let slice = &buf[loc..loc + flatbuffers::SIZE_UOFFSET];
                            let off = flatbuffers::read_scalar::<u32>(slice) as usize;

                            // `Follow.from_utf8_unchecked()` details logic:
                            // https://github.com/google/flatbuffers/blob/27325e002a36b3de2999906a77ff13a14fb09471/rust/flatbuffers/src/vector.rs#L152
                            let loc = loc + off;
                            let len = flatbuffers::read_scalar_at::<flatbuffers::UOffsetT>(buf, loc)
                                as usize;
                            let slice = &buf[loc + flatbuffers::SIZE_UOFFSET
                                ..loc + flatbuffers::SIZE_UOFFSET + len];

                            match from_utf8(slice) {
                                Ok(s) => s.to_string(),
                                Err(_) => match MUtf8::decode(slice) {
                                    Ok(decode) => StringUnicodeEncoderDecoder::encode_string_to_unicode_sequence(&decode),
                                    Err(_) => "Error Decode MUtf8.".to_string(),
                                },
                            }
                        }
                    })
                    .map(|s| EncodeValueData::String(s))
                    .unwrap_or(EncodeValueData::String(String::new()));
                encode_value_string
            }

            AnnotationEncodeValueType::TypeValue => {
                let type_value = meta
                    .value_as_class_meta()
                    .map(|class_meta| ClassData::with_meta(bridge, class_meta))
                    .unwrap(); // should not be None, if it is, just panic
                EncodeValueData::Type(type_value)
            }

            AnnotationEncodeValueType::MethodValue => {
                let method_value = meta
                    .value_as_method_meta()
                    .map(|method_meta| MethodData::with_meta(bridge, method_meta))
                    .unwrap(); // should not be None, if it is, just panic
                EncodeValueData::Method(method_value)
            }

            AnnotationEncodeValueType::EnumValue => {
                let enum_value = meta
                    .value_as_field_meta()
                    .map(|field_meta| FieldData::with_meta(bridge, field_meta))
                    .unwrap(); // should not be None, if it is, just panic
                EncodeValueData::Enum(enum_value)
            }

            AnnotationEncodeValueType::ArrayValue => {
                let array_value = meta
                    .value_as_annotation_encode_array()
                    .map(|array_meta| AnnotationEncodeArrayData::with_meta(bridge, array_meta))
                    .unwrap(); // should not be None, if it is, just panic
                EncodeValueData::Array(array_value)
            }

            AnnotationEncodeValueType::AnnotationValue => {
                let annotation_value = meta
                    .value_as_annotation_meta()
                    .map(|annotation_meta| AnnotationData::with_meta(bridge, annotation_meta))
                    .unwrap(); // should not be None, if it is, just panic
                EncodeValueData::Annotation(annotation_value)
            }

            AnnotationEncodeValueType::BoolValue => {
                let bool_value = meta
                    .value_as_encode_value_boolean()
                    .map(|v| v.value())
                    .unwrap_or(false);
                EncodeValueData::Bool(bool_value)
            }

            _ => EncodeValueData::Null,
        };

        Self {
            value_type: meta_value_tpye.into(),
            value,
        }
    }
}
