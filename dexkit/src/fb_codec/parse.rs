use crate::errors::Error;

pub(crate) fn parse_fb_root<'a, T>(
    data: &'a [u8],
) -> Result<<T as flatbuffers::Follow<'a>>::Inner, Error>
where
    T: flatbuffers::Follow<'a> + flatbuffers::Verifiable + 'a,
{
    flatbuffers::root::<T>(data).map_err(|_| Error::BridgeOperation("invalid flatbuffer response"))
}

pub(crate) unsafe fn parse_fb_root_unchecked<'a, T>(
    data: &'a [u8],
) -> <T as flatbuffers::Follow<'a>>::Inner
where
    T: flatbuffers::Follow<'a> + 'a,
{
    unsafe { flatbuffers::root_unchecked::<T>(data) }
}
