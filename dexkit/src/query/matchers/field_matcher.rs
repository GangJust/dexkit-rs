use crate::gen_flatbuffers::dexkit::schema::{
    FieldMatcher as FBFieldMatcher, FieldMatcherArgs as FBFieldMatcherArgs,
};
use crate::query::base::{BaseQuery, IAnnotationEncodeValue};
use crate::query::matchers::MethodsMatcher;
use crate::query::matchers::base::AccessFlagsMatcher;
use crate::query::matchers::base::StringMatcher;
use crate::query::matchers::{AnnotationMatcher, AnnotationsMatcher};
use crate::query::matchers::{ClassMatcher, MethodMatcher};
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub struct FieldMatcher {
    name_matcher: Option<StringMatcher>,
    modifiers_matcher: Option<AccessFlagsMatcher>,
    class_matcher: Option<ClassMatcher>,
    type_matcher: Option<ClassMatcher>,
    annotations_matcher: Option<AnnotationsMatcher>,
    get_methods_matcher: Option<MethodsMatcher>,
    put_methods_matcher: Option<MethodsMatcher>,
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
            },
        )
    }
}

impl FieldMatcher {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_field_name_matcher(mut self, matcher: StringMatcher) -> Self {
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

    pub fn set_type_matcher(mut self, matcher: ClassMatcher) -> Self {
        self.type_matcher = Some(matcher);
        self
    }

    pub fn set_annotations_matcher(mut self, matcher: AnnotationsMatcher) -> Self {
        self.annotations_matcher = Some(matcher);
        self
    }

    pub fn set_get_methods_matcher(mut self, matcher: MethodsMatcher) -> Self {
        self.get_methods_matcher = Some(matcher);
        self
    }

    pub fn set_put_methods_matcher(mut self, matcher: MethodsMatcher) -> Self {
        self.put_methods_matcher = Some(matcher);
        self
    }

    // extend name_matcher
    pub fn set_field_name_str<S: Into<String>>(self, name: S) -> Self {
        self.set_field_name_matcher(StringMatcher::create_string_str(name))
    }

    // extend modifiers_matcher
    pub fn set_modifiers<U: Into<u32>>(mut self, modifiers: U) -> Self {
        self.modifiers_matcher = Some(AccessFlagsMatcher::create().set_modifiers(modifiers.into()));
        self
    }

    pub fn or_modifiers<U: Into<u32>>(mut self, modifiers: U) -> Self {
        if self.modifiers_matcher.is_none() {
            self.modifiers_matcher =
                Some(AccessFlagsMatcher::create().set_modifiers(modifiers.into()));
        } else {
            self.modifiers_matcher = self
                .modifiers_matcher
                .map(|mm| mm.or_modifiers(modifiers.into()));
        }
        self
    }

    // extend class_matcher
    pub fn set_class_name_str<S: Into<String>>(mut self, class_name: S) -> Self {
        self.class_matcher = Some(ClassMatcher::create().set_class_name_str(class_name));
        self
    }

    // extend type_matcher
    pub fn set_type_name_str<S: Into<String>>(mut self, type_name: S) -> Self {
        self.type_matcher = Some(ClassMatcher::create().set_class_name_str(type_name));
        self
    }

    // extend annotations_matcher
    pub fn add_annotations(mut self, annotations: Vec<AnnotationMatcher>) -> Self {
        for annotation in annotations {
            self = self.add_annotation(annotation);
        }
        self
    }

    pub fn add_annotation(mut self, annotation: AnnotationMatcher) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher =
                Some(AnnotationsMatcher::create().add_annotation_matcher(annotation));
        } else {
            self.annotations_matcher = self
                .annotations_matcher
                .map(|am| am.add_annotation_matcher(annotation));
        }
        self
    }

    pub fn add_annotation_strs<S: Into<String>>(mut self, annotations: Vec<S>) -> Self {
        if self.annotations_matcher.is_none() {
            self.add_annotations(
                annotations
                    .into_iter()
                    .map(|s| AnnotationMatcher::create().set_type_class_name(s))
                    .collect(),
            )
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| {
                am.add_annotation_matchers(
                    annotations
                        .into_iter()
                        .map(|s| AnnotationMatcher::create().set_type_class_name(s))
                        .collect(),
                )
            });
            self
        }
    }

    pub fn add_annotation_str<S: Into<String>>(mut self, annotation: S) -> Self {
        if self.annotations_matcher.is_none() {
            self.add_annotation(AnnotationMatcher::create().set_type_class_name(annotation))
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| {
                am.add_annotation_matcher(
                    AnnotationMatcher::create().set_type_class_name(annotation),
                )
            });
            self
        }
    }

    pub fn annotation_count(mut self, count: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::create().count(count));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count(count));
        }
        self
    }

    pub fn annotation_count_range(mut self, min: u32, max: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::create().count_range(min, max));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count_range(min, max));
        }
        self
    }

    pub fn annotation_count_min(mut self, min: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::create().count_min(min));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count_min(min));
        }
        self
    }

    pub fn annotation_count_max(mut self, max: u32) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::create().count_max(max));
        } else {
            self.annotations_matcher = self.annotations_matcher.map(|am| am.count_max(max));
        }
        self
    }

    // extend get_methods_matcher
    pub fn add_get_method(mut self, method: MethodMatcher) -> Self {
        if self.get_methods_matcher.is_none() {
            self.get_methods_matcher = Some(MethodsMatcher::create().add_method_matcher(method));
        } else {
            self.get_methods_matcher = self
                .get_methods_matcher
                .map(|mm| mm.add_method_matcher(method));
        }
        self
    }

    // extend put_methods_matcher
    pub fn add_put_method(mut self, method: MethodMatcher) -> Self {
        if self.put_methods_matcher.is_none() {
            self.put_methods_matcher = Some(MethodsMatcher::create().add_method_matcher(method));
        } else {
            self.put_methods_matcher = self
                .put_methods_matcher
                .map(|mm| mm.add_method_matcher(method));
        }
        self
    }
}
