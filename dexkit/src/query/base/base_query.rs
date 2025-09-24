use flatbuffers::FlatBufferBuilder;

pub(crate) trait BaseQuery<'a, T> {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> T;
}
