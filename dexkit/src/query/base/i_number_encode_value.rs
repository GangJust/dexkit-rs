use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub trait INumberEncodeValue {
    fn inner_build_number_union(&self, fbb: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset>;
}
