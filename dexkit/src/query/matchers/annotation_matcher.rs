use crate::gen_flatbuffers::dexkit::schema::{
    AnnotationMatcher as FBAnnotationMatcher, AnnotationMatcherArgs as FBAnnotationMatcherArgs,
    RetentionPolicyType as FBRetentionPolicyType,
};
use crate::query::base::BaseQuery;
use crate::query::base::IAnnotationEncodeValue;
use crate::query::enums::{RetentionPolicyType, StringMatchType, TargetElementType};
use crate::query::matchers::ClassMatcher;
use crate::query::matchers::base::StringMatcher;
use crate::query::matchers::base::TargetElementTypesMatcher;
use crate::query::matchers::{AnnotationElementMatcher, AnnotationElementsMatcher};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

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

    pub fn set_using_strings_matcher(mut self, matcher: Vec<StringMatcher>) -> Self {
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

    pub fn set_eq_type_class_name<S: Into<String>>(mut self, class_name: S) -> Self {
        self.type_matcher = Some(
            ClassMatcher::create()
                .set_class_name_matcher(StringMatcher::create_eq_string_str(class_name)),
        );
        self
    }

    // extend add_element_matcher
    pub fn add_element_matcher(mut self, matcher: AnnotationElementMatcher) -> Self {
        self.elements_matcher = if let Some(mut existing) = self.elements_matcher {
            existing = existing.add_element_matcher(matcher);
            Some(existing)
        } else {
            self.elements_matcher
        };
        self
    }

    pub fn element_count(mut self, count: u32) -> Self {
        self.elements_matcher = if let Some(mut existing) = self.elements_matcher {
            existing = existing.count(count);
            Some(existing)
        } else {
            self.elements_matcher
        };
        self
    }

    pub fn element_count_range(mut self, min: u32, max: u32) -> Self {
        self.elements_matcher = if let Some(mut existing) = self.elements_matcher {
            existing = existing.count_range(min, max);
            Some(existing)
        } else {
            self.elements_matcher
        };
        self
    }

    pub fn element_count_min(mut self, min: u32) -> Self {
        self.elements_matcher = if let Some(mut existing) = self.elements_matcher {
            existing = existing.count_min(min);
            Some(existing)
        } else {
            self.elements_matcher
        };
        self
    }

    pub fn element_count_max(mut self, max: u32) -> Self {
        self.elements_matcher = if let Some(mut existing) = self.elements_matcher {
            existing = existing.count_max(max);
            Some(existing)
        } else {
            self.elements_matcher
        };
        self
    }

    // extend using_strings_matcher
    pub fn add_using_string_matchers(mut self, matchers: Vec<StringMatcher>) -> Self {
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.extend(matchers);
        } else {
            self.using_strings_matcher = Some(matchers);
        }
        self
    }

    pub fn add_using_string_matcher(mut self, matcher: StringMatcher) -> Self {
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.push(matcher);
        } else {
            self.using_strings_matcher = Some(vec![matcher]);
        }
        self
    }

    pub fn add_using_string_strs<S: Into<String>>(mut self, ss: Vec<S>) -> Self {
        let matchers: Vec<StringMatcher> = ss
            .into_iter()
            .map(|s| StringMatcher::create_string_str(s))
            .collect();
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.extend(matchers);
        } else {
            self.using_strings_matcher = Some(matchers);
        }
        self
    }

    pub fn add_eq_using_string_strs<S: Into<String>>(mut self, ss: Vec<S>) -> Self {
        let matchers: Vec<StringMatcher> = ss
            .into_iter()
            .map(|s| StringMatcher::create_eq_string_str(s))
            .collect();
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.extend(matchers);
        } else {
            self.using_strings_matcher = Some(matchers);
        }
        self
    }

    pub fn add_using_string_str<S: Into<String>>(mut self, s: S) -> Self {
        let matcher = StringMatcher::create_string_str(s);
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.push(matcher);
        } else {
            self.using_strings_matcher = Some(vec![matcher]);
        }
        self
    }

    pub fn add_eq_using_string_str<S: Into<String>>(mut self, s: S) -> Self {
        let matcher = StringMatcher::create_eq_string_str(s);
        if let Some(ref mut vec) = self.using_strings_matcher {
            vec.push(matcher);
        } else {
            self.using_strings_matcher = Some(vec![matcher]);
        }
        self
    }
}
