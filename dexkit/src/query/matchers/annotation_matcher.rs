use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationMatcher as FBAnnotationMatcher, AnnotationMatcherArgs as FBAnnotationMatcherArgs,
    RetentionPolicyType as FBRetentionPolicyType,
};
use crate::query::base::BaseQuery;
use crate::query::base::IAnnotationEncodeValue;
use crate::query::enums::RetentionPolicyType;
use crate::query::matchers::base::StringMatcher;
use crate::query::matchers::base::TargetElementTypesMatcher;
use crate::query::matchers::AnnotationElementsMatcher;
use crate::query::matchers::ClassMatcher;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct AnnotationMatcher {
    type_matcher: Option<ClassMatcher>,
    target_element_types_matcher: Option<TargetElementTypesMatcher>,
    using_strings_matcher: Option<StringMatcher>,
    policy: Option<RetentionPolicyType>,
    elements_matcher: Option<AnnotationElementsMatcher>,
}

impl Default for AnnotationMatcher {
    fn default() -> Self {
        AnnotationMatcher {
            type_matcher: None,
            target_element_types_matcher: None,
            using_strings_matcher: None,
            policy: None,
            elements_matcher: None,
        }
    }
}

impl IAnnotationEncodeValue for AnnotationMatcher {
    fn inner_build_annotation_union(
        &self,
        fbb: &mut FlatBufferBuilder,
    ) -> WIPOffset<flatbuffers::UnionWIPOffset> {
        self.inner_build(fbb).as_union_value()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBAnnotationMatcher<'a>>> for AnnotationMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBAnnotationMatcher<'a>> {
        let type_ = self.type_matcher.as_ref().map(|m| m.inner_build(fbb));
        let target_element_types = self
            .target_element_types_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let policy: FBRetentionPolicyType = self
            .policy
            .map(|p| p.into())
            .unwrap_or(FBRetentionPolicyType::Any);
        let elements = self.elements_matcher.as_ref().map(|m| m.inner_build(fbb));
        let using_strings = self.using_strings_matcher.as_ref().map(|m| {
            let matcher = m.inner_build(fbb);
            fbb.create_vector(&[matcher])
        });

        FBAnnotationMatcher::create(
            fbb,
            &FBAnnotationMatcherArgs {
                type_,
                target_element_types,
                policy,
                elements,
                using_strings,
            },
        )
    }
}

impl AnnotationMatcher {
    pub(crate) fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_type_matcher(mut self, matcher: ClassMatcher) -> Self {
        self.type_matcher = Some(matcher);
        self
    }

    pub fn set_target_element_types_matcher(mut self, matcher: TargetElementTypesMatcher) -> Self {
        self.target_element_types_matcher = Some(matcher);
        self
    }

    pub fn set_using_strings_matcher(mut self, matcher: StringMatcher) -> Self {
        self.using_strings_matcher = Some(matcher);
        self
    }

    pub fn set_policy(mut self, policy: RetentionPolicyType) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn set_elements_matcher(mut self, matcher: AnnotationElementsMatcher) -> Self {
        self.elements_matcher = Some(matcher);
        self
    }

    // extend type_matcher
    pub fn set_type_class_name<S: Into<String>>(mut self, class_name: S) -> Self {
        self.type_matcher = Some(
            ClassMatcher::create()
                .set_class_name_matcher(StringMatcher::create_string_str(class_name)),
        );
        self
    }

    // extend target_element_types_matcher
}
