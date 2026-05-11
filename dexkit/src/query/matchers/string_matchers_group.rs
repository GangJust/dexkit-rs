use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::fb_codec::{ToFbBytes, finish_fb_bytes};
use crate::gen_flatbuffers::dexkit::fb::{
    FBBatchUsingStringsMatcher, FBBatchUsingStringsMatcherArgs,
};
use crate::query::{base::BaseQuery, matchers::base::StringMatcher};

pub struct StringMatchersGroup {
    group_name: Option<String>,
    string_matchers: Vec<StringMatcher>,
}

impl ToFbBytes for StringMatchersGroup {
    fn to_fb_bytes(&self) -> Vec<u8> {
        let mut fbb = FlatBufferBuilder::with_capacity(256);
        let root = self.inner_build(&mut fbb);
        finish_fb_bytes(fbb, root)
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBBatchUsingStringsMatcher<'a>>> for StringMatchersGroup {
    fn inner_build(
        &self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBBatchUsingStringsMatcher<'a>> {
        let union_key = self.group_name.as_ref().map(|name| fbb.create_string(name));
        let using_strings = if !self.string_matchers.is_empty() {
            let matchers_offsets: Vec<_> = self
                .string_matchers
                .iter()
                .map(|m| m.inner_build(fbb))
                .collect();
            Some(fbb.create_vector(&matchers_offsets))
        } else {
            None
        };

        FBBatchUsingStringsMatcher::create(
            fbb,
            &FBBatchUsingStringsMatcherArgs {
                union_key,
                using_strings,
            },
        )
    }
}

impl StringMatchersGroup {
    pub fn new<S>(group_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            group_name: Some(group_name.into()),
            string_matchers: Vec::new(),
        }
    }
}

impl StringMatchersGroup {
    pub fn group_name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.group_name = Some(name.into());
        self
    }

    pub fn string_matchers<I>(mut self, matchers: I) -> Self
    where
        I: IntoIterator<Item = StringMatcher>,
    {
        self.string_matchers = matchers.into_iter().collect();
        self
    }
}

impl StringMatchersGroup {
    pub fn add_string_matcher(mut self, matcher: StringMatcher) -> Self {
        self.string_matchers.push(matcher);
        self
    }
}
