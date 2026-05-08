use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub(crate) trait IAnnotationEncodeValue {
    fn inner_build_annotation_union(
        &self,
        fbb: &mut FlatBufferBuilder,
    ) -> WIPOffset<UnionWIPOffset>;
}
