use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBFieldMatcher, FBFieldMatcherArgs};
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::enums::MatchType;
use crate::query::matchers::AccessFlagsMatcher;
use crate::query::matchers::MethodsMatcher;
use crate::query::matchers::{AnnotationMatcher, AnnotationsMatcher};
use crate::query::matchers::{ClassMatcher, MethodMatcher, StringMatcher};

pub struct FieldMatcher {
    name_matcher: Option<StringMatcher>,
    modifiers_matcher: Option<AccessFlagsMatcher>,
    class_matcher: Option<ClassMatcher>,
    type_matcher: Option<ClassMatcher>,
    annotations_matcher: Option<AnnotationsMatcher>,
    get_methods_matcher: Option<MethodsMatcher>,
    put_methods_matcher: Option<MethodsMatcher>,
    all_of_matcher: Option<Vec<FieldMatcher>>,
    any_of_matcher: Option<Vec<FieldMatcher>>,
    none_of_matcher: Option<Vec<FieldMatcher>>,
}

impl Default for FieldMatcher {
    fn default() -> Self {
        FieldMatcher {
            name_matcher: None,
            modifiers_matcher: None,
            class_matcher: None,
            type_matcher: None,
            annotations_matcher: None,
            get_methods_matcher: None,
            put_methods_matcher: None,
            all_of_matcher: None,
            any_of_matcher: None,
            none_of_matcher: None,
        }
    }
}

impl IAnnotationEncodeValue for FieldMatcher {
    fn inner_build_annotation_union(
        &self,
        fbb: &mut FlatBufferBuilder,
    ) -> WIPOffset<UnionWIPOffset> {
        self.inner_build(fbb).as_union_value()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBFieldMatcher<'a>>> for FieldMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBFieldMatcher<'a>> {
        let field_name = self.name_matcher.as_ref().map(|m| m.inner_build(fbb));
        let access_flags = self.modifiers_matcher.as_ref().map(|m| m.inner_build(fbb));
        let declaring_class = self.class_matcher.as_ref().map(|m| m.inner_build(fbb));
        let type_class = self.type_matcher.as_ref().map(|m| m.inner_build(fbb));
        let annotations = self
            .annotations_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let get_methods = self
            .get_methods_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let put_methods = self
            .put_methods_matcher
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

        FBFieldMatcher::create(
            fbb,
            &FBFieldMatcherArgs {
                field_name,
                access_flags,
                declaring_class,
                type_class,
                annotations,
                get_methods,
                put_methods,
                all_of,
                any_of,
                none_of,
            },
        )
    }
}

impl FieldMatcher {
    pub fn new() -> Self {
        Self::default()
    }

    // base
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

    pub fn field_type(mut self, matcher: ClassMatcher) -> Self {
        self.type_matcher = Some(matcher);
        self
    }

    pub fn annotations(mut self, matcher: AnnotationsMatcher) -> Self {
        self.annotations_matcher = Some(matcher);
        self
    }

    pub fn get_methods(mut self, matcher: MethodsMatcher) -> Self {
        self.get_methods_matcher = Some(matcher);
        self
    }

    pub fn put_methods(mut self, matcher: MethodsMatcher) -> Self {
        self.put_methods_matcher = Some(matcher);
        self
    }

    // extend name_matcher
    pub fn name_contains<S>(self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name(StringMatcher::contains(name))
    }

    // extend modifiers_matcher
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

    // extend class_matcher
    pub fn class_name<S>(mut self, class_name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_matcher = Some(ClassMatcher::new().class_name_equals(class_name));
        self
    }

    // extend type_matcher
    pub fn type_name<S>(mut self, type_name: S) -> Self
    where
        S: Into<String>,
    {
        self.type_matcher = Some(ClassMatcher::new().class_name_equals(type_name));
        self
    }

    // extend annotations_matcher
    pub fn extend_annotations(mut self, annotations: Vec<AnnotationMatcher>) -> Self {
        for annotation in annotations {
            self = self.annotation(annotation);
        }
        self
    }

    pub fn annotation(mut self, annotation: AnnotationMatcher) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::new().annotation(annotation));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.annotation(annotation));
        }
        self
    }

    pub fn annotation_names<S>(mut self, annotations: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        if self.annotations_matcher.is_none() {
            self.extend_annotations(
                annotations
                    .into_iter()
                    .map(|s| AnnotationMatcher::new().type_name_contains(s))
                    .collect(),
            )
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| {
                am.extend_annotations(
                    annotations
                        .into_iter()
                        .map(|s| AnnotationMatcher::new().type_name_contains(s))
                        .collect(),
                )
            });
            self
        }
    }

    pub fn annotation_name<S>(mut self, annotation: S) -> Self
    where
        S: Into<String>,
    {
        if self.annotations_matcher.is_none() {
            self.annotation(AnnotationMatcher::new().type_name_contains(annotation))
        } else {
            self.annotations_matcher = self
                .annotations_matcher
                .map(|am| am.annotation(AnnotationMatcher::new().type_name_contains(annotation)));
            self
        }
    }

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
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count_range(min, max));
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

    // extend get_methods_matcher
    pub fn get_method(mut self, method: MethodMatcher) -> Self {
        if self.get_methods_matcher.is_none() {
            self.get_methods_matcher = Some(MethodsMatcher::new().method(method));
        } else {
            self.get_methods_matcher = self.get_methods_matcher.map(|mm| mm.method(method));
        }
        self
    }

    // extend put_methods_matcher
    pub fn put_method(mut self, method: MethodMatcher) -> Self {
        if self.put_methods_matcher.is_none() {
            self.put_methods_matcher = Some(MethodsMatcher::new().method(method));
        } else {
            self.put_methods_matcher = self.put_methods_matcher.map(|mm| mm.method(method));
        }
        self
    }
}
