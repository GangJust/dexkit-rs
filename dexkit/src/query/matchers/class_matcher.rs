use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBClassMatcher, FBClassMatcherArgs};
use crate::query::base::BaseQuery;
use crate::query::base::IAnnotationEncodeValue;
use crate::query::enums::MatchType;
use crate::query::enums::StringMatchType;
use crate::query::matchers::AccessFlagsMatcher;
use crate::query::matchers::AnnotationMatcher;
use crate::query::matchers::AnnotationsMatcher;
use crate::query::matchers::FieldMatcher;
use crate::query::matchers::FieldsMatcher;
use crate::query::matchers::MethodMatcher;
use crate::query::matchers::MethodsMatcher;
use crate::query::matchers::StringMatcher;
use crate::query::matchers::interfaces_matcher::InterfacesMatcher;

pub struct ClassMatcher {
    source_matcher: Option<StringMatcher>,
    class_name_matcher: Option<StringMatcher>,
    modifiers_matcher: Option<AccessFlagsMatcher>,
    super_class_name_matcher: Option<Box<ClassMatcher>>,
    interfaces_matcher: Option<InterfacesMatcher>,
    annotations_matcher: Option<AnnotationsMatcher>,
    fields_matcher: Option<FieldsMatcher>,
    methods_matcher: Option<MethodsMatcher>,
    using_strings_matcher: Option<Vec<StringMatcher>>,
    all_of_matcher: Option<Vec<ClassMatcher>>,
    any_of_matcher: Option<Vec<ClassMatcher>>,
    none_of_matcher: Option<Vec<ClassMatcher>>,
}

impl Default for ClassMatcher {
    fn default() -> Self {
        ClassMatcher {
            source_matcher: None,
            class_name_matcher: None,
            modifiers_matcher: None,
            super_class_name_matcher: None,
            interfaces_matcher: None,
            annotations_matcher: None,
            fields_matcher: None,
            methods_matcher: None,
            using_strings_matcher: None,
            all_of_matcher: None,
            any_of_matcher: None,
            none_of_matcher: None,
        }
    }
}

impl IAnnotationEncodeValue for ClassMatcher {
    fn inner_build_annotation_union(
        &self,
        fbb: &mut FlatBufferBuilder,
    ) -> WIPOffset<flatbuffers::UnionWIPOffset> {
        self.inner_build(fbb).as_union_value()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBClassMatcher<'a>>> for ClassMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBClassMatcher<'a>> {
        let smali_source = self.source_matcher.as_ref().map(|m| m.inner_build(fbb));
        let class_name = self.class_name_matcher.as_ref().map(|m| m.inner_build(fbb));
        let access_flags = self.modifiers_matcher.as_ref().map(|m| m.inner_build(fbb));
        let super_class = self
            .super_class_name_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let interfaces = self.interfaces_matcher.as_ref().map(|m| m.inner_build(fbb));
        let annotations = self
            .annotations_matcher
            .as_ref()
            .map(|m| m.inner_build(fbb));
        let fields = self.fields_matcher.as_ref().map(|m| m.inner_build(fbb));
        let methods = self.methods_matcher.as_ref().map(|m| m.inner_build(fbb));
        let using_strings = self.using_strings_matcher.as_ref().map(|v| {
            let vec: Vec<_> = v.iter().map(|m| m.inner_build(fbb)).collect();
            fbb.create_vector(&vec)
        });
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

        FBClassMatcher::create(
            fbb,
            &FBClassMatcherArgs {
                smali_source,
                class_name,
                access_flags,
                super_class,
                interfaces,
                annotations,
                fields,
                methods,
                using_strings,
                all_of,
                any_of,
                none_of,
            },
        )
    }
}

impl ClassMatcher {
    pub fn new() -> Self {
        ClassMatcher::default()
    }

    // base
    pub fn source(mut self, matcher: StringMatcher) -> Self {
        self.source_matcher = Some(matcher);
        self
    }

    pub fn class_name(mut self, matcher: StringMatcher) -> Self {
        self.class_name_matcher = Some(matcher);
        self
    }

    pub fn modifiers(mut self, modifiers_matcher: AccessFlagsMatcher) -> Self {
        self.modifiers_matcher = Some(modifiers_matcher);
        self
    }

    pub fn super_class(mut self, super_class_name_matcher: ClassMatcher) -> Self {
        self.super_class_name_matcher = Some(Box::new(super_class_name_matcher));
        self
    }

    pub fn interfaces_matcher(mut self, interfaces_matcher: InterfacesMatcher) -> Self {
        self.interfaces_matcher = Some(interfaces_matcher);
        self
    }

    pub fn annotations(mut self, annotations_matcher: AnnotationsMatcher) -> Self {
        self.annotations_matcher = Some(annotations_matcher);
        self
    }

    pub fn fields(mut self, fields_matcher: FieldsMatcher) -> Self {
        self.fields_matcher = Some(fields_matcher);
        self
    }

    pub fn methods(mut self, methods_matcher: MethodsMatcher) -> Self {
        self.methods_matcher = Some(methods_matcher);
        self
    }

    pub fn using_strings(mut self, using_strings_matcher: Vec<StringMatcher>) -> Self {
        self.using_strings_matcher = Some(using_strings_matcher);
        self
    }

    // extend source
    pub fn source_contains<S>(self, source: S) -> Self
    where
        S: Into<String>,
    {
        self.source(StringMatcher::contains(source))
    }

    // extend class_name
    pub fn class_name_equals<S>(self, class_name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_name(
            StringMatcher::new()
                .value(class_name)
                .match_type(StringMatchType::Equals),
        )
    }

    // extend modifiers
    pub fn modifiers_value<U>(mut self, modifiers: U) -> Self
    where
        U: Into<u32>,
    {
        self.modifiers_matcher = Some(AccessFlagsMatcher::new(modifiers.into(), MatchType::default()));
        self
    }

    pub fn or_modifiers<U>(mut self, modifiers: U) -> Self
    where
        U: Into<u32>,
    {
        if self.modifiers_matcher.is_none() {
            self.modifiers_matcher =
                Some(AccessFlagsMatcher::new(modifiers.into(), MatchType::default()));
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
            self.modifiers_matcher =
                Some(AccessFlagsMatcher::new(modifiers.into(), MatchType::default()));
        } else {
            self.modifiers_matcher = self
                .modifiers_matcher
                .map(|mm| mm.and_modifiers(modifiers.into()));
        }
        self
    }

    // extend super_class_name
    pub fn super_class_name<S>(self, super_class_name: S) -> Self
    where
        S: Into<String>,
    {
        let matcher = ClassMatcher::new().class_name_equals(super_class_name);
        self.super_class(matcher)
    }

    // extend interfaces
    pub fn interface_names(mut self, interfaces: Vec<String>) -> Self {
        self.interfaces_matcher = Some(InterfacesMatcher::new().interface_names(interfaces));
        self
    }

    pub fn interface(mut self, interface: ClassMatcher) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::new().interface(interface));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.interface(interface));
        }
        self
    }

    pub fn extend_interface_names<S>(mut self, interfaces: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::new().interfaces(
                interfaces
                    .into_iter()
                    .map(|s| ClassMatcher::new().class_name_equals(s))
                    .collect(),
            ));
            self
        } else {
            self.interfaces_matcher = self
                .interfaces_matcher
                .map(|im| im.extend_interface_names(interfaces));
            self
        }
    }

    pub fn interface_name<S>(self, interface: S) -> Self
    where
        S: Into<String>,
    {
        self.interface(ClassMatcher::new().class_name_equals(interface))
    }

    pub fn interface_count(mut self, count: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::new().count(count));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count(count));
        }
        self
    }

    pub fn interface_count_range(mut self, min: u32, max: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::new().count_range(min, max));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count_range(min, max));
        }
        self
    }

    pub fn interface_count_min(mut self, min: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::new().count_min(min));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count_min(min));
        }
        self
    }

    pub fn interface_count_max(mut self, max: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::new().count_max(max));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count_max(max));
        }
        self
    }

    // extend annotations
    pub fn extend_annotations(mut self, annotations: Vec<AnnotationMatcher>) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::new().annotations(annotations));
        } else {
            self.annotations_matcher = self
                .annotations_matcher
                .map(|am| am.extend_annotations(annotations));
        }
        self
    }

    pub fn annotation(mut self, annotation: AnnotationMatcher) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher = Some(AnnotationsMatcher::new().annotation(annotation));
        } else {
            self.annotations_matcher = self
                .annotations_matcher
                .map(|am| am.annotation(annotation));
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
            self.annotations_matcher = self.annotations_matcher.map(|am| {
                am.annotation(AnnotationMatcher::new().type_name_contains(annotation))
            });
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

    // extend fields
    pub fn field(mut self, field: FieldMatcher) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::new().field(field));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.field(field));
        }
        self
    }

    pub fn extend_field_names<S>(mut self, field_names: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::new().extend_field_names(field_names));
        } else {
            self.fields_matcher = self
                .fields_matcher
                .map(|fm| fm.extend_field_names(field_names));
        }
        self
    }

    pub fn field_name<S>(self, field_name: S) -> Self
    where
        S: Into<String>,
    {
        self.field(FieldMatcher::new().name_contains(field_name))
    }

    pub fn field_count(mut self, count: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::new().count(count));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count(count));
        }
        self
    }

    pub fn field_count_range(mut self, min: u32, max: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::new().count_range(min, max));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count_range(min, max));
        }
        self
    }

    pub fn field_count_min(mut self, min: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::new().count_min(min));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count_min(min));
        }
        self
    }

    pub fn field_count_max(mut self, max: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::new().count_max(max));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count_max(max));
        }
        self
    }

    // extend methods
    pub fn method(mut self, method: MethodMatcher) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::new().method(method));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.method(method));
        }
        self
    }

    pub fn extend_method_names<S>(mut self, method_names: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::new().method_names(method_names));
        } else {
            self.methods_matcher = self
                .methods_matcher
                .map(|mm| mm.method_names(method_names));
        }
        self
    }

    pub fn method_name<S>(self, method_name: S) -> Self
    where
        S: Into<String>,
    {
        self.method(MethodMatcher::new().name_contains(method_name))
    }

    pub fn method_count(mut self, count: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::new().count(count));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count(count));
        }
        self
    }

    pub fn method_count_range(mut self, min: u32, max: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::new().count_range(min, max));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count_range(min, max));
        }
        self
    }

    pub fn method_count_min(mut self, min: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::new().count_min(min));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count_min(min));
        }
        self
    }

    pub fn method_count_max(mut self, max: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::new().count_max(max));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count_max(max));
        }
        self
    }

    // extend using_strings
    pub fn extend_using_strings(mut self, using_strings: Vec<StringMatcher>) -> Self {
        if self.using_strings_matcher.is_none() {
            self.using_strings_matcher = Some(using_strings);
        } else {
            self.using_strings_matcher = self.using_strings_matcher.map(|mut v| {
                v.extend(using_strings);
                v
            });
        }
        self
    }

    pub fn using_string(mut self, using_string: StringMatcher) -> Self {
        if self.using_strings_matcher.is_none() {
            self.using_strings_matcher = Some(vec![using_string]);
        } else {
            self.using_strings_matcher = self.using_strings_matcher.map(|mut v| {
                v.push(using_string);
                v
            });
        }
        self
    }

    pub fn using_string_contains_all<S>(mut self, using_strings: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        let matchers: Vec<StringMatcher> = using_strings
            .into_iter()
            .map(StringMatcher::contains)
            .collect();
        if self.using_strings_matcher.is_none() {
            self.using_strings_matcher = Some(matchers);
        } else {
            self.using_strings_matcher = self.using_strings_matcher.map(|mut v| {
                v.extend(matchers);
                v
            });
        }
        self
    }

    pub fn using_string_contains<S>(self, using_string: S) -> Self
    where
        S: Into<String>,
    {
        self.using_string(StringMatcher::contains(using_string))
    }

    pub fn using_string_equals<S>(self, using_string: S) -> Self
    where
        S: Into<String>,
    {
        self.using_string(
            StringMatcher::new()
                .value(using_string)
                .match_type(StringMatchType::Equals),
        )
    }
}
