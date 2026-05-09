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

    // base
    pub fn annotation_type(mut self, matcher: ClassMatcher) -> Self {
        self.type_matcher = Some(matcher);
        self
    }

    pub fn target_element_types(mut self, matcher: TargetElementTypesMatcher) -> Self {
        self.target_element_types_matcher = Some(matcher);
        self
    }

    pub fn using_strings(mut self, matcher: Vec<StringMatcher>) -> Self {
        self.using_strings_matcher = Some(matcher);
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

    // extend type_matcher
    pub fn type_name_contains<S>(mut self, class_name: S) -> Self
    where
        S: Into<String>,
    {
        self.type_matcher =
            Some(ClassMatcher::new().class_name(StringMatcher::contains(class_name)));
        self
    }

    pub fn type_name_equals<S>(mut self, class_name: S) -> Self
    where
        S: Into<String>,
    {
        self.type_matcher = Some(ClassMatcher::new().class_name(StringMatcher::equals(class_name)));
        self
    }

    // extend add_element_matcher
    pub fn element(mut self, matcher: AnnotationElementMatcher) -> Self {
        self.elements_matcher = Some(
            self.elements_matcher
                .unwrap_or_else(AnnotationElementsMatcher::new)
                .element(matcher),
        );
        self
    }

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

    // extend using_strings_matcher
    pub fn extend_using_strings(mut self, matchers: Vec<StringMatcher>) -> Self {
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.extend(matchers);
        } else {
            self.using_strings_matcher = Some(matchers);
        }
        self
    }

    pub fn using_string(mut self, matcher: StringMatcher) -> Self {
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.push(matcher);
        } else {
            self.using_strings_matcher = Some(vec![matcher]);
        }
        self
    }

    pub fn using_string_contains_all<S>(mut self, ss: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        let matchers: Vec<StringMatcher> = ss.into_iter().map(StringMatcher::contains).collect();
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.extend(matchers);
        } else {
            self.using_strings_matcher = Some(matchers);
        }
        self
    }

    pub fn using_string_equals_all<S>(mut self, ss: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        let matchers: Vec<StringMatcher> = ss.into_iter().map(StringMatcher::equals).collect();
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.extend(matchers);
        } else {
            self.using_strings_matcher = Some(matchers);
        }
        self
    }

    pub fn using_string_contains<S>(mut self, s: S) -> Self
    where
        S: Into<String>,
    {
        let matcher = StringMatcher::contains(s);
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.push(matcher);
        } else {
            self.using_strings_matcher = Some(vec![matcher]);
        }
        self
    }

    pub fn using_string_equals<S>(mut self, s: S) -> Self
    where
        S: Into<String>,
    {
        let matcher = StringMatcher::equals(s);
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.push(matcher);
        } else {
            self.using_strings_matcher = Some(vec![matcher]);
        }
        self
    }
}
