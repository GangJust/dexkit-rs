use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{
    FBMethodMatcher, FBMethodMatcherArgs, FBNumber, FBNumberUnion, FBNumberUnionArgs,
};
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::enums::MatchType;
use crate::query::matchers::{
    AccessFlagsMatcher, AnnotationsMatcher, ClassMatcher, MethodsMatcher, NumberEncodeValueMatcher,
    OpCodesMatcher, ParametersMatcher, StringMatcher, UsingFieldMatcher,
};

pub struct MethodMatcher {
    name_matcher: Option<StringMatcher>,
    modifiers_matcher: Option<AccessFlagsMatcher>,
    class_matcher: Option<ClassMatcher>,
    proto_shorty_matcher: Option<String>,
    return_type_matcher: Option<ClassMatcher>,
    params_matcher: Option<ParametersMatcher>,
    annotations_matcher: Option<AnnotationsMatcher>,
    op_codes_matcher: Option<OpCodesMatcher>,
    using_strings_matcher: Option<Vec<StringMatcher>>,
    using_fields_matcher: Option<Vec<UsingFieldMatcher>>,
    using_numbers_matcher: Option<Vec<NumberEncodeValueMatcher>>,
    invoke_methods_matcher: Option<MethodsMatcher>,
    caller_methods_matcher: Option<MethodsMatcher>,
    all_of_matcher: Option<Vec<MethodMatcher>>,
    any_of_matcher: Option<Vec<MethodMatcher>>,
    none_of_matcher: Option<Vec<MethodMatcher>>,
}

impl Default for MethodMatcher {
    fn default() -> Self {
        Self {
            name_matcher: None,
            modifiers_matcher: None,
            class_matcher: None,
            proto_shorty_matcher: None,
            return_type_matcher: None,
            params_matcher: None,
            annotations_matcher: None,
            op_codes_matcher: None,
            using_strings_matcher: None,
            using_fields_matcher: None,
            using_numbers_matcher: None,
            invoke_methods_matcher: None,
            caller_methods_matcher: None,
            all_of_matcher: None,
            any_of_matcher: None,
            none_of_matcher: None,
        }
    }
}

impl IAnnotationEncodeValue for MethodMatcher {
    fn inner_build_annotation_union(
        &self,
        fbb: &mut FlatBufferBuilder,
    ) -> WIPOffset<UnionWIPOffset> {
        self.inner_build(fbb).as_union_value()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBMethodMatcher<'a>>> for MethodMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBMethodMatcher<'a>> {
        let method_name = self.name_matcher.as_ref().map(|m| m.inner_build(fbb));
        let access_flags = self.modifiers_matcher.as_ref().map(|m| m.inner_build(fbb));
        let declaring_class = self.class_matcher.as_ref().map(|m| m.inner_build(fbb));
        let return_type = self
            .return_type_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let parameters = self.params_matcher.as_ref().map(|m| m.inner_build(fbb));
        let annotations = self
            .annotations_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let op_codes = self.op_codes_matcher.as_ref().map(|m| m.inner_build(fbb));
        let using_strings = self.using_strings_matcher.as_ref().map(|vec| {
            let built_vec: Vec<_> = vec.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&built_vec)
        });
        let using_fields = self.using_fields_matcher.as_ref().map(|vec| {
            let built_vec: Vec<_> = vec.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&built_vec)
        });
        let using_numbers = self.using_numbers_matcher.as_ref().map(|vec| {
            let vec = vec.iter().map(|n| {
                let value_type: FBNumber = n.into();
                let value = n.inner_build(fbb);
                FBNumberUnion::create(fbb, &FBNumberUnionArgs { value_type, value })
            });
            let built_vec: Vec<_> = vec.collect();
            fbb.create_vector(&built_vec)
        });
        let invoking_methods = self
            .invoke_methods_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let method_callers = self
            .caller_methods_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let all_of = self.all_of_matcher.as_ref().map(|vec| {
            let built_vec: Vec<_> = vec.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&built_vec)
        });
        let any_of = self.any_of_matcher.as_ref().map(|vec| {
            let built_vec: Vec<_> = vec.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&built_vec)
        });
        let none_of = self.none_of_matcher.as_ref().map(|vec| {
            let built_vec: Vec<_> = vec.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&built_vec)
        });
        let proto_shorty = self
            .proto_shorty_matcher
            .as_ref()
            .map(|s| fbb.create_string(s));

        FBMethodMatcher::create(
            fbb,
            &FBMethodMatcherArgs {
                method_name,
                access_flags,
                declaring_class,
                return_type,
                parameters,
                annotations,
                op_codes,
                using_strings,
                using_fields,
                using_numbers,
                invoking_methods,
                method_callers,
                proto_shorty,
                all_of,
                any_of,
                none_of,
            },
        )
    }
}

impl MethodMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MethodMatcher {
    pub fn name(mut self, matcher: StringMatcher) -> Self {
        self.name_matcher = Some(matcher);
        self
    }

    pub fn modifiers(mut self, matcher: AccessFlagsMatcher) -> Self {
        self.modifiers_matcher = Some(matcher);
        self
    }

    pub fn class(mut self, matcher: ClassMatcher) -> Self {
        self.class_matcher = Some(matcher);
        self
    }

    pub fn proto_shorty<S>(mut self, proto: S) -> Self
    where
        S: Into<String>,
    {
        self.proto_shorty_matcher = Some(proto.into());
        self
    }

    pub fn return_type(mut self, matcher: ClassMatcher) -> Self {
        self.return_type_matcher = Some(matcher);
        self
    }

    pub fn params(mut self, matcher: ParametersMatcher) -> Self {
        self.params_matcher = Some(matcher);
        self
    }

    pub fn annotations(mut self, matcher: AnnotationsMatcher) -> Self {
        self.annotations_matcher = Some(matcher);
        self
    }

    pub fn op_codes(mut self, matcher: OpCodesMatcher) -> Self {
        self.op_codes_matcher = Some(matcher);
        self
    }

    pub fn using_strings<I>(mut self, matcher: I) -> Self
    where
        I: IntoIterator<Item = StringMatcher>,
    {
        self.using_strings_matcher = Some(matcher.into_iter().collect());
        self
    }

    pub fn using_fields<I>(mut self, matcher: I) -> Self
    where
        I: IntoIterator<Item = UsingFieldMatcher>,
    {
        self.using_fields_matcher = Some(matcher.into_iter().collect());
        self
    }

    pub fn using_numbers<I>(mut self, matcher: I) -> Self
    where
        I: IntoIterator<Item = NumberEncodeValueMatcher>,
    {
        self.using_numbers_matcher = Some(matcher.into_iter().collect());
        self
    }

    pub fn invoke_methods(mut self, matcher: MethodsMatcher) -> Self {
        self.invoke_methods_matcher = Some(matcher);
        self
    }

    pub fn caller_methods(mut self, matcher: MethodsMatcher) -> Self {
        self.caller_methods_matcher = Some(matcher);
        self
    }
}

impl MethodMatcher {
    pub fn modifiers_value<U>(mut self, modifiers: U) -> Self
    where
        U: Into<u32>,
    {
        self.modifiers_matcher = Some(AccessFlagsMatcher::new(
            modifiers.into(),
            MatchType::default(),
        ));
        self
    }

    pub fn or_modifiers<U>(mut self, modifiers: U) -> Self
    where
        U: Into<u32>,
    {
        if self.modifiers_matcher.is_none() {
            self.modifiers_matcher = Some(AccessFlagsMatcher::new(
                modifiers.into(),
                MatchType::default(),
            ));
        } else {
            self.modifiers_matcher = self
                .modifiers_matcher
                .map(|mm| mm.or_modifiers(modifiers.into()));
        }
        self
    }

    pub fn and_modifiers<U>(mut self, modifiers: U) -> Self
    where
        U: Into<u32>,
    {
        if self.modifiers_matcher.is_none() {
            self.modifiers_matcher = Some(AccessFlagsMatcher::new(
                modifiers.into(),
                MatchType::default(),
            ));
        } else {
            self.modifiers_matcher = self
                .modifiers_matcher
                .map(|mm| mm.and_modifiers(modifiers.into()));
        }
        self
    }
}

impl MethodMatcher {
    pub fn add_using_string(mut self, matcher: StringMatcher) -> Self {
        self.using_strings_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
        self
    }

    pub fn add_using_field(mut self, matcher: UsingFieldMatcher) -> Self {
        self.using_fields_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
        self
    }

    pub fn add_using_number(mut self, matcher: NumberEncodeValueMatcher) -> Self {
        self.using_numbers_matcher
            .get_or_insert_with(Vec::new)
            .push(matcher);
        self
    }
}

impl MethodMatcher {
    pub fn annotation_count(mut self, count: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::new().count(count));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count(count));
        }
        self
    }

    pub fn annotation_count_range(mut self, min: u32, max: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::new().count_range(min, max));
        } else {
            self.annotations_matcher = self
                .annotations_matcher
                .map(|am| am.count_range(min, max));
        }
        self
    }

    pub fn annotation_count_min(mut self, min: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::new().count_min(min));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count_min(min));
        }
        self
    }

    pub fn annotation_count_max(mut self, max: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::new().count_max(max));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count_max(max));
        }
        self
    }
}
