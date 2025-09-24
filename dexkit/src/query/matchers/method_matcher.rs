use crate::gen_flatbuffers::dexkit::schema::{
    MethodMatcher as FBMethodMatcher, MethodMatcherArgs as FBMethodMatcherArgs, Number as FBNumber,
    NumberWrapper as FBNumberWrapper, NumberWrapperArgs as FBNumberWrapperArgs,
};
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::matchers::base::AccessFlagsMatcher;
use crate::query::matchers::base::NumberEncodeValueMatcher;
use crate::query::matchers::base::OpCodesMatcher;
use crate::query::matchers::base::StringMatcher;
use crate::query::matchers::AnnotationsMatcher;
use crate::query::matchers::ClassMatcher;
use crate::query::matchers::MethodsMatcher;
use crate::query::matchers::ParametersMatcher;
use crate::query::matchers::UsingFieldMatcher;
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

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
}

impl Default for MethodMatcher {
    fn default() -> Self {
        MethodMatcher {
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
                FBNumberWrapper::create(
                    fbb,
                    &FBNumberWrapperArgs {
                        value_type,
                        value,
                        ..FBNumberWrapperArgs::default()
                    },
                )
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
            },
        )
    }
}

impl MethodMatcher {
    pub fn create() -> Self {
        MethodMatcher::default()
    }

    pub fn set_name_matcher(mut self, matcher: StringMatcher) -> Self {
        self.name_matcher = Some(matcher);
        self
    }

    pub fn set_modifiers_matcher(mut self, matcher: AccessFlagsMatcher) -> Self {
        self.modifiers_matcher = Some(matcher);
        self
    }

    pub fn set_class_matcher(mut self, matcher: ClassMatcher) -> Self {
        self.class_matcher = Some(matcher);
        self
    }

    pub fn set_proto_shorty_matcher<S: Into<String>>(mut self, proto: S) -> Self {
        self.proto_shorty_matcher = Some(proto.into());
        self
    }

    pub fn set_return_type_matcher(mut self, matcher: ClassMatcher) -> Self {
        self.return_type_matcher = Some(matcher);
        self
    }

    pub fn set_params_matcher(mut self, matcher: ParametersMatcher) -> Self {
        self.params_matcher = Some(matcher);
        self
    }

    pub fn set_annotations_matcher(mut self, matcher: AnnotationsMatcher) -> Self {
        self.annotations_matcher = Some(matcher);
        self
    }

    pub fn set_op_codes_matcher(mut self, matcher: OpCodesMatcher) -> Self {
        self.op_codes_matcher = Some(matcher);
        self
    }

    pub fn set_using_strings_matcher(mut self, matcher: Vec<StringMatcher>) -> Self {
        self.using_strings_matcher = Some(matcher);
        self
    }

    pub fn set_using_fields_matcher(mut self, matcher: Vec<UsingFieldMatcher>) -> Self {
        self.using_fields_matcher = Some(matcher);
        self
    }

    pub fn set_using_numbers_matcher(mut self, matcher: Vec<NumberEncodeValueMatcher>) -> Self {
        self.using_numbers_matcher = Some(matcher);
        self
    }

    pub fn set_invoke_methods_matcher(mut self, matcher: MethodsMatcher) -> Self {
        self.invoke_methods_matcher = Some(matcher);
        self
    }

    pub fn set_caller_methods_matcher(mut self, matcher: MethodsMatcher) -> Self {
        self.caller_methods_matcher = Some(matcher);
        self
    }

    pub fn set_method_name_str<S: Into<String>>(mut self, name: S) -> Self {
        self.name_matcher = Some(StringMatcher::create_string_str(name));
        self
    }
}
