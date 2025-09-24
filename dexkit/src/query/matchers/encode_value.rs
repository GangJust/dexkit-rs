use crate::gen_flatbuffers::dexkit::schema::{
    EncodeValueBoolean as FBEncodeValueBoolean, EncodeValueBooleanArgs as FBEncodeValueBooleanArgs,
    EncodeValueByte as FBEncodeValueByte, EncodeValueByteArgs as FBEncodeValueByteArgs,
    EncodeValueChar as FBEncodeValueChar, EncodeValueCharArgs as FBEncodeValueCharArgs,
    EncodeValueDouble as FBEncodeValueDouble, EncodeValueDoubleArgs as FBEncodeValueDoubleArgs,
    EncodeValueFloat as FBEncodeValueFloat, EncodeValueFloatArgs as FBEncodeValueFloatArgs,
    EncodeValueInt as FBEncodeValueInt, EncodeValueIntArgs as FBEncodeValueIntArgs,
    EncodeValueLong as FBEncodeValueLong, EncodeValueLongArgs as FBEncodeValueLongArgs,
    EncodeValueNull as FBEncodeValueNull, EncodeValueNullArgs as FBEncodeValueNullArgs,
    EncodeValueShort as FBEncodeValueShort, EncodeValueShortArgs as FBEncodeValueShortArgs,
};

use crate::query::base::IAnnotationEncodeValue;
use crate::query::base::INumberEncodeValue;
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub struct EncodeValueByte(pub i8);
impl IAnnotationEncodeValue for EncodeValueByte {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueByte::create(fbb, &FBEncodeValueByteArgs { value: self.0 });
        value.as_union_value()
    }
}
impl INumberEncodeValue for EncodeValueByte {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        self.inner_build_annotation_union(fbb)
    }
}

pub struct EncodeValueShort(pub i16);
impl IAnnotationEncodeValue for EncodeValueShort {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueChar::create(fbb, &FBEncodeValueCharArgs { value: self.0 });
        value.as_union_value()
    }
}
impl INumberEncodeValue for EncodeValueShort {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        self.inner_build_annotation_union(fbb)
    }
}

pub struct EncodeValueChar(pub i16);
impl IAnnotationEncodeValue for EncodeValueChar {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueShort::create(fbb, &FBEncodeValueShortArgs { value: self.0 });
        value.as_union_value()
    }
}
impl INumberEncodeValue for EncodeValueChar {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        self.inner_build_annotation_union(fbb)
    }
}

pub struct EncodeValueInt(pub i32);
impl IAnnotationEncodeValue for EncodeValueInt {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueInt::create(fbb, &FBEncodeValueIntArgs { value: self.0 });
        value.as_union_value()
    }
}
impl INumberEncodeValue for EncodeValueInt {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        self.inner_build_annotation_union(fbb)
    }
}

pub struct EncodeValueLong(pub i64);
impl IAnnotationEncodeValue for EncodeValueLong {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueLong::create(fbb, &FBEncodeValueLongArgs { value: self.0 });
        value.as_union_value()
    }
}
impl INumberEncodeValue for EncodeValueLong {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        self.inner_build_annotation_union(fbb)
    }
}

pub struct EncodeValueFloat(pub f32);
impl IAnnotationEncodeValue for EncodeValueFloat {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueFloat::create(fbb, &FBEncodeValueFloatArgs { value: self.0 });
        value.as_union_value()
    }
}
impl INumberEncodeValue for EncodeValueFloat {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        self.inner_build_annotation_union(fbb)
    }
}

pub struct EncodeValueDouble(pub f64);
impl IAnnotationEncodeValue for EncodeValueDouble {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueDouble::create(fbb, &FBEncodeValueDoubleArgs { value: self.0 });
        value.as_union_value()
    }
}
impl INumberEncodeValue for EncodeValueDouble {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        self.inner_build_annotation_union(fbb)
    }
}

pub struct EncodeValueString(pub i32);

pub struct EncodeValueNull;
impl IAnnotationEncodeValue for EncodeValueNull {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueNull::create(fbb, &FBEncodeValueNullArgs { value: 0 });
        value.as_union_value()
    }
}

pub struct EncodeValueBoolean(pub bool);
impl IAnnotationEncodeValue for EncodeValueBoolean {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        let value = FBEncodeValueBoolean::create(fbb, &FBEncodeValueBooleanArgs { value: self.0 });
        value.as_union_value()
    }
}
