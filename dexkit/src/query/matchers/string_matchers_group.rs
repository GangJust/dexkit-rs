use crate::gen_flatbuffers::dexkit::schema::{
    BatchUsingStringsMatcher as FBBatchUsingStringsMatcher,
    BatchUsingStringsMatcherArgs as FBBatchUsingStringsMatcherArgs,
};
use crate::query::{base::BaseQuery, matchers::base::StringMatcher};

pub struct StringMatchersGroup {
    group_name: Option<String>,
    string_matchers: Vec<StringMatcher>,
}

impl From<StringMatchersGroup> for Vec<u8> {
    fn from(value: StringMatchersGroup) -> Self {
        let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(256);
        let root = value.inner_build(&mut fbb);
        fbb.finish(root, None);
        fbb.finished_data().to_vec()
    }
}

impl<'a> BaseQuery<'a, flatbuffers::WIPOffset<FBBatchUsingStringsMatcher<'a>>>
    for StringMatchersGroup
{
    fn inner_build(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<FBBatchUsingStringsMatcher<'a>> {
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
    pub fn create<S: Into<String>>(group_name: S) -> Self {
        Self {
            group_name: Some(group_name.into()),
            string_matchers: Vec::new(),
        }
    }

    // base
    pub fn set_group_name<S: Into<String>>(mut self, name: S) -> Self {
        self.group_name = Some(name.into());
        self
    }

    pub fn set_string_matchers(mut self, matchers: Vec<StringMatcher>) -> Self {
        self.string_matchers = matchers;
        self
    }

    /// extend string_matchers
    pub fn add_string_matcher(mut self, matcher: StringMatcher) -> Self {
        self.string_matchers.push(matcher);
        self
    }

    pub fn add_string_matchers(mut self, matchers: Vec<StringMatcher>) -> Self {
        self.string_matchers.extend(matchers);
        self
    }

    pub fn add_string_matcher_str<S: Into<String>>(mut self, matcher: S) -> Self {
        self.string_matchers
            .push(StringMatcher::create().set_value(matcher));
        self
    }

    pub fn add_string_matchers_str<S: Into<String>>(mut self, matchers: Vec<S>) -> Self {
        self.string_matchers.extend(
            matchers
                .into_iter()
                .map(|m| StringMatcher::create().set_value(m)),
        );
        self
    }
}
