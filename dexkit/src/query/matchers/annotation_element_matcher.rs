use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{
    FBAnnotationElementMatcher, FBAnnotationElementMatcherArgs, FBAnnotationEncodeValueMatcher,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::{
    AnnotationEncodeArrayMatcher, AnnotationEncodeValueMatcher, AnnotationMatcher, ClassMatcher,
    FieldMatcher, MethodMatcher, StringMatcher,
};

pub struct AnnotationElementMatcher {
    name_matcher: Option<StringMatcher>,
    value_matcher: Option<AnnotationEncodeValueMatcher>,
}

impl Default for AnnotationElementMatcher {
    fn default() -> Self {
        Self {
            name_matcher: None,
            value_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBAnnotationElementMatcher<'a>>> for AnnotationElementMatcher {
    fn inner_build(
        &self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBAnnotationElementMatcher<'a>> {
        let name = self.name_matcher.as_ref().map(|m| m.inner_build(fbb));
        let value_type: FBAnnotationEncodeValueMatcher = if let Some(matcher) = &self.value_matcher
        {
            matcher.into()
        } else {
            FBAnnotationEncodeValueMatcher::NONE
        };
        let value = self
            .value_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb))
            .unwrap_or(None);

        FBAnnotationElementMatcher::create(
            fbb,
            &FBAnnotationElementMatcherArgs {
                name,
                value_type,
                value,
            },
        )
    }
}

impl AnnotationElementMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AnnotationElementMatcher {
    pub fn name(mut self, matcher: StringMatcher) -> Self {
        self.name_matcher = Some(matcher);
        self
    }

    pub fn value(mut self, matcher: AnnotationEncodeValueMatcher) -> Self {
        self.value_matcher = Some(matcher);
        self
    }
}

impl AnnotationElementMatcher {
    pub fn byte_value(mut self, value: i8) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::byte(value));
        self
    }

    pub fn short_value(mut self, value: i16) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::short(value));
        self
    }

    pub fn char_value(mut self, value: char) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::char(value));
        self
    }

    pub fn int_value(mut self, value: i32) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::int(value));
        self
    }

    pub fn long_value(mut self, value: i64) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::long(value));
        self
    }

    pub fn float_value(mut self, value: f32) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::float(value));
        self
    }

    pub fn double_value(mut self, value: f64) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::double(value));
        self
    }

    pub fn string_value(mut self, value: StringMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::string(value));
        self
    }

    pub fn class_value(mut self, value: ClassMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::class(value));
        self
    }

    pub fn method_value(mut self, value: MethodMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::method(value));
        self
    }

    pub fn enum_value(mut self, value: FieldMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::enum_value(value));
        self
    }

    pub fn array_value(mut self, value: AnnotationEncodeArrayMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::array(value));
        self
    }

    pub fn annotation_value(mut self, value: AnnotationMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::annotation(value));
        self
    }

    pub fn null_value(mut self) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::null());
        self
    }

    pub fn bool_value(mut self, value: bool) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::bool(value));
        self
    }
}
