use crate::DexkitBridge;
use crate::gen_flatbuffers::dexkit::schema::{
    UsingFieldMeta as FBUsingFieldMeta, UsingFieldMetaArrayHolder as FBUsingFieldMetaArrayHolder,
};
use crate::result::{FieldData, FieldUsingType};

#[derive(Debug, Clone)]
pub struct UsingFieldData<'a> {
    field: FieldData<'a>,
    using_type: FieldUsingType,
}

impl<'a> UsingFieldData<'a> {
    /// The field being used
    pub fn field(&self) -> FieldData<'a> {
        self.field.clone()
    }

    /// The type of usage (read or write)
    pub fn using_type(&self) -> FieldUsingType {
        self.using_type.clone()
    }

    /// ...
    pub(crate) fn with_meta(bridge: &'a DexkitBridge, meta: FBUsingFieldMeta<'a>) -> Self {
        let field = meta
            .field()
            .map(|f| FieldData::with_meta(bridge, f))
            .unwrap();
        let using_type = FieldUsingType::from(meta.using_type());

        Self { field, using_type }
    }

    /// ...
    pub(crate) fn with_using_field_meta_array_raw(
        bridge: &'a DexkitBridge,
        data: &'a [u8],
    ) -> Vec<Self> {
        flatbuffers::root::<FBUsingFieldMetaArrayHolder<'a>>(data)
            .map(|array| Self::with_using_field_meta_array(bridge, array))
            .unwrap_or_default()
    }

    /// ...
    pub(crate) fn with_using_field_meta_array(
        bridge: &'a DexkitBridge,
        array: FBUsingFieldMetaArrayHolder<'a>,
    ) -> Vec<Self> {
        array
            .items()
            .map(|items| {
                items
                    .iter()
                    .map(|meta| Self::with_meta(bridge, meta))
                    .collect()
            })
            .unwrap_or_default()
    }
}
