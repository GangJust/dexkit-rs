use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub(crate) trait INumberEncodeValue {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset>;
}
