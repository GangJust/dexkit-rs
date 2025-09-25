use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationElementMatcher as FBAnnotationElementMatcher,
    AnnotationElementMatcherArgs as FBAnnotationElementMatcherArgs,
    AnnotationEncodeValueMatcher as FBAnnotationEncodeValueMatcher,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::base::AnnotationEncodeValueMatcher;
use crate::query::matchers::base::StringMatcher;
use crate::query::matchers::{
    AnnotationEncodeArrayMatcher, AnnotationMatcher, ClassMatcher, FieldMatcher, MethodMatcher,
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct AnnotationElementMatcher {
    name_matcher: Option<StringMatcher>,
    value_matcher: Option<AnnotationEncodeValueMatcher>,
}

impl Default for AnnotationElementMatcher {
    fn default() -> Self {
        AnnotationElementMatcher {
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
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_name_matcher(mut self, matcher: StringMatcher) -> Self {
        self.name_matcher = Some(matcher);
        self
    }

    pub fn set_value_matcher(mut self, matcher: AnnotationEncodeValueMatcher) -> Self {
        self.value_matcher = Some(matcher);
        self
    }

    // extended methods
    pub fn byte_value(mut self, value: i8) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_number_byte(value));
        self
    }

    pub fn short_value(mut self, value: i16) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_number_short(value));
        self
    }

    pub fn int_value(mut self, value: i32) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_number_int(value));
        self
    }

    pub fn long_value(mut self, value: i64) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_number_long(value));
        self
    }

    pub fn float_value(mut self, value: f32) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_number_float(value));
        self
    }

    pub fn double_value(mut self, value: f64) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_number_double(value));
        self
    }

    pub fn string_value(mut self, value: StringMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_string(value));
        self
    }

    pub fn class_value(mut self, value: ClassMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_class(value));
        self
    }

    pub fn method_value(mut self, value: MethodMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_method(value));
        self
    }

    pub fn enum_value(mut self, value: FieldMatcher) -> Self {
        let enum_matcher = AnnotationEncodeValueMatcher::create_enum(value);
        self.value_matcher = Some(enum_matcher);
        self
    }

    pub fn array_value(mut self, value: AnnotationEncodeArrayMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_array(value));
        self
    }

    pub fn annotation_value(mut self, value: AnnotationMatcher) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_annotation(value));
        self
    }

    pub fn null_value(mut self) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_null());
        self
    }

    pub fn bool_value(mut self, value: bool) -> Self {
        self.value_matcher = Some(AnnotationEncodeValueMatcher::create_bool(value));
        self
    }
}
