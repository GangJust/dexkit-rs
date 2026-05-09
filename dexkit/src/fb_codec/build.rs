use flatbuffers::FlatBufferBuilder;

pub(crate) trait ToFbBytes {
    fn to_fb_bytes(&self) -> Vec<u8>;
}

pub(crate) fn finish_fb_bytes<'a, Root>(
    mut fbb: FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Root>,
) -> Vec<u8> {
    fbb.finish(root, None);
    fbb.finished_data().to_vec()
}
