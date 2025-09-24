use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub trait IAnnotationEncodeValue {
    fn inner_build_annotation_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset>;
}
