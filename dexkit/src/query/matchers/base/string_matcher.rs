use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBStringMatchType, FBStringMatcher, FBStringMatcherArgs};
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::enums::StringMatchType;

pub struct StringMatcher {
    value: Option<String>,
    match_type: StringMatchType,
    ignore_case: bool,
}

impl Default for StringMatcher {
    fn default() -> Self {
        Self {
            value: None,
            match_type: StringMatchType::Contains,
            ignore_case: false,
        }
    }
}

impl IAnnotationEncodeValue for StringMatcher {
    fn inner_build_annotation_union(
        &self,
        fbb: &mut FlatBufferBuilder,
    ) -> WIPOffset<flatbuffers::UnionWIPOffset> {
        self.inner_build(fbb).as_union_value()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBStringMatcher<'a>>> for StringMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBStringMatcher<'a>> {
        let value = self.value.as_ref().map(|v| fbb.create_string(v));
        let match_type: FBStringMatchType = self.match_type.into();

        let root = FBStringMatcher::create(
            fbb,
            &FBStringMatcherArgs {
                value,
                match_type,
                ignore_case: self.ignore_case,
            },
        );
        root
    }
}

impl StringMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StringMatcher {
    pub fn contains<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: Some(value.into()),
            match_type: StringMatchType::Contains,
            ignore_case: false,
        }
    }

    pub fn equals<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: Some(value.into()),
            match_type: StringMatchType::Equals,
            ignore_case: false,
        }
    }
}

impl StringMatcher {
    pub fn value<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.value = Some(value.into());
        self
    }

    pub fn match_type(mut self, match_type: StringMatchType) -> Self {
        self.match_type = match_type;
        self
    }

    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = ignore_case;
        self
    }
}
