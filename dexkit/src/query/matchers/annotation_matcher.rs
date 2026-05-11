use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{
    FBAnnotationMatcher, FBAnnotationMatcherArgs, FBRetentionPolicyType,
};
use crate::query::base::BaseQuery;
use crate::query::base::IAnnotationEncodeValue;
use crate::query::enums::RetentionPolicyType;
use crate::query::matchers::{
    AnnotationElementMatcher, AnnotationElementsMatcher, ClassMatcher, StringMatcher,
    TargetElementTypesMatcher,
};

pub struct AnnotationMatcher {
    type_matcher: Option<ClassMatcher>,
    target_element_types_matcher: Option<TargetElementTypesMatcher>,
    using_strings_matcher: Option<Vec<StringMatcher>>,
    policy: Option<RetentionPolicyType>,
    elements_matcher: Option<AnnotationElementsMatcher>,
}

impl Default for AnnotationMatcher {
    fn default() -> Self {
        Self {
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
        let using_strings = self.using_strings_matcher.as_ref().map(|matchers| {
            let offsets: Vec<_> = matchers.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&offsets)
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
    pub fn new() -> Self {
        Self::default()
    }
}

impl AnnotationMatcher {
    pub fn annotation_type(mut self, matcher: ClassMatcher) -> Self {
        self.type_matcher = Some(matcher);
        self
    }

    pub fn target_element_types(mut self, matcher: TargetElementTypesMatcher) -> Self {
        self.target_element_types_matcher = Some(matcher);
        self
    }

    pub fn using_strings<I>(mut self, matcher: I) -> Self
    where
        I: IntoIterator<Item = StringMatcher>,
    {
        self.using_strings_matcher = Some(matcher.into_iter().collect());
        self
    }

    pub fn policy(mut self, policy: RetentionPolicyType) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn elements(mut self, matcher: AnnotationElementsMatcher) -> Self {
        self.elements_matcher = Some(matcher);
        self
    }
}

impl AnnotationMatcher {
    pub fn add_using_string(mut self, matcher: StringMatcher) -> Self {
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.push(matcher);
        } else {
            self.using_strings_matcher = Some(vec![matcher]);
        }
        self
    }
}

impl AnnotationMatcher {
    pub fn element_count(mut self, count: u32) -> Self {
        self.elements_matcher = Some(
            self.elements_matcher
                .unwrap_or_else(AnnotationElementsMatcher::new)
                .count(count),
        );
        self
    }

    pub fn element_count_range(mut self, min: u32, max: u32) -> Self {
        self.elements_matcher = Some(
            self.elements_matcher
                .unwrap_or_else(AnnotationElementsMatcher::new)
                .count_range(min, max),
        );
        self
    }

    pub fn element_count_min(mut self, min: u32) -> Self {
        self.elements_matcher = Some(
            self.elements_matcher
                .unwrap_or_else(AnnotationElementsMatcher::new)
                .count_min(min),
        );
        self
    }

    pub fn element_count_max(mut self, max: u32) -> Self {
        self.elements_matcher = Some(
            self.elements_matcher
                .unwrap_or_else(AnnotationElementsMatcher::new)
                .count_max(max),
        );
        self
    }
}
