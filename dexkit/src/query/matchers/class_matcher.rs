use crate::gen_flatbuffers::dexkit::schema::{
    ClassMatcher as FBClassMatcher, ClassMatcherArgs as FBClassMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::base::IAnnotationEncodeValue;
use crate::query::enums::{MatchType, StringMatchType};
use crate::query::matchers::AnnotationMatcher;
use crate::query::matchers::AnnotationsMatcher;
use crate::query::matchers::FieldMatcher;
use crate::query::matchers::FieldsMatcher;
use crate::query::matchers::MethodMatcher;
use crate::query::matchers::MethodsMatcher;
use crate::query::matchers::base::AccessFlagsMatcher;
use crate::query::matchers::base::StringMatcher;
use crate::query::matchers::interfaces_matcher::InterfacesMatcher;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

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
            },
        )
    }
}

impl ClassMatcher {
    pub fn create() -> Self {
        ClassMatcher::default()
    }

    // base
    pub fn set_source_matcher(mut self, matcher: StringMatcher) -> Self {
        self.source_matcher = Some(matcher);
        self
    }

    pub fn set_class_name_matcher(mut self, matcher: StringMatcher) -> Self {
        self.class_name_matcher = Some(matcher);
        self
    }

    pub fn set_modifiers_matcher(mut self, modifiers_matcher: AccessFlagsMatcher) -> Self {
        self.modifiers_matcher = Some(modifiers_matcher);
        self
    }

    pub fn set_super_class_name_matcher(mut self, super_class_name_matcher: ClassMatcher) -> Self {
        self.super_class_name_matcher = Some(Box::new(super_class_name_matcher));
        self
    }

    pub fn set_interfaces_matcher(mut self, interfaces_matcher: InterfacesMatcher) -> Self {
        self.interfaces_matcher = Some(interfaces_matcher);
        self
    }

    pub fn set_annotations_matcher(mut self, annotations_matcher: AnnotationsMatcher) -> Self {
        self.annotations_matcher = Some(annotations_matcher);
        self
    }

    pub fn set_fields_matcher(mut self, fields_matcher: FieldsMatcher) -> Self {
        self.fields_matcher = Some(fields_matcher);
        self
    }

    pub fn set_methods_matcher(mut self, methods_matcher: MethodsMatcher) -> Self {
        self.methods_matcher = Some(methods_matcher);
        self
    }

    pub fn set_using_strings_matcher(mut self, using_strings_matcher: Vec<StringMatcher>) -> Self {
        self.using_strings_matcher = Some(using_strings_matcher);
        self
    }

    // extend source
    pub fn set_source_str<S: Into<String>>(self, source: S) -> Self {
        self.set_source_matcher(StringMatcher::create_string_str(source))
    }

    // extend class_name
    pub fn set_class_name_str<S: Into<String>>(self, class_name: S) -> Self {
        self.set_class_name_matcher(
            StringMatcher::create()
                .set_value(class_name)
                .set_match_type(StringMatchType::Equals),
        )
    }

    // extend modifiers
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

    // extend super_class_name
    pub fn set_super_class_name_str<S: Into<String>>(self, super_class_name: S) -> Self {
        let matcher = ClassMatcher::create().set_class_name_str(super_class_name);
        self.set_super_class_name_matcher(matcher)
    }

    // extend interfaces
    pub fn set_interfaces(mut self, interfaces: Vec<ClassMatcher>) -> Self {
        self.interfaces_matcher =
            Some(InterfacesMatcher::create().set_interface_name_matcher(interfaces));
        self
    }

    pub fn set_interfaces_strs(mut self, interfaces: Vec<String>) -> Self {
        self.interfaces_matcher =
            Some(InterfacesMatcher::create().set_interface_name_strs(interfaces));
        self
    }

    pub fn add_interface(mut self, interface: ClassMatcher) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher =
                Some(InterfacesMatcher::create().add_interface_name_matcher(interface));
        } else {
            self.interfaces_matcher = self
                .interfaces_matcher
                .map(|im| im.add_interface_name_matcher(interface));
        }
        self
    }

    pub fn add_interface_strs<S: Into<String>>(mut self, interfaces: Vec<S>) -> Self {
        if self.interfaces_matcher.is_none() {
            self.set_interfaces(
                interfaces
                    .into_iter()
                    .map(|s| ClassMatcher::create().set_class_name_str(s))
                    .collect(),
            )
        } else {
            self.interfaces_matcher = self
                .interfaces_matcher
                .map(|im| im.add_interface_name_strs(interfaces));
            self
        }
    }

    pub fn add_interfaces_str<S: Into<String>>(self, interface: S) -> Self {
        self.add_interface(ClassMatcher::create().set_class_name_str(interface))
    }

    pub fn interface_count(mut self, count: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::create().count(count));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count(count));
        }
        self
    }

    pub fn interface_count_range(mut self, min: u32, max: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::create().count_range(min, max));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count_range(min, max));
        }
        self
    }

    pub fn interface_count_min(mut self, min: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::create().count_min(min));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count_min(min));
        }
        self
    }

    pub fn interface_count_max(mut self, max: u32) -> Self {
        if self.interfaces_matcher.is_none() {
            self.interfaces_matcher = Some(InterfacesMatcher::create().count_max(max));
        } else {
            self.interfaces_matcher = self.interfaces_matcher.map(|im| im.count_max(max));
        }
        self
    }

    // extend annotations
    pub fn add_annotations(mut self, annotations: Vec<AnnotationMatcher>) -> Self {
        if self.annotations_matcher.is_none() {
            self.annotations_matcher =
                Some(AnnotationsMatcher::create().set_annotations_matcher(annotations));
        } else {
            self.annotations_matcher = self
                .annotations_matcher
                .map(|am| am.add_annotation_matchers(annotations));
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

    pub fn add_annotations_strs<S: Into<String>>(mut self, annotations: Vec<S>) -> Self {
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

    pub fn add_annotations_str<S: Into<String>>(mut self, annotation: S) -> Self {
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

    // extend fields
    pub fn add_field(mut self, field: FieldMatcher) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::create().add_field_matcher(field));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.add_field_matcher(field));
        }
        self
    }

    pub fn add_field_strs<S: Into<String>>(mut self, field_names: Vec<S>) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::create().add_field_names_strs(field_names));
        } else {
            self.fields_matcher = self
                .fields_matcher
                .map(|fm| fm.add_field_names_strs(field_names));
        }
        self
    }

    pub fn add_field_str<S: Into<String>>(self, field_name: S) -> Self {
        self.add_field(FieldMatcher::create().set_field_name_str(field_name))
    }

    pub fn field_count(mut self, count: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::create().count(count));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count(count));
        }
        self
    }

    pub fn field_count_range(mut self, min: u32, max: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::create().count_range(min, max));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count_range(min, max));
        }
        self
    }

    pub fn field_count_min(mut self, min: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::create().count_min(min));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count_min(min));
        }
        self
    }

    pub fn field_count_max(mut self, max: u32) -> Self {
        if self.fields_matcher.is_none() {
            self.fields_matcher = Some(FieldsMatcher::create().count_max(max));
        } else {
            self.fields_matcher = self.fields_matcher.map(|fm| fm.count_max(max));
        }
        self
    }

    // extend methods
    pub fn add_method(mut self, method: MethodMatcher) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::create().add_method_matcher(method));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.add_method_matcher(method));
        }
        self
    }

    pub fn add_method_strs<S: Into<String>>(mut self, method_names: Vec<S>) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher =
                Some(MethodsMatcher::create().add_method_names_strs(method_names));
        } else {
            self.methods_matcher = self
                .methods_matcher
                .map(|mm| mm.add_method_names_strs(method_names));
        }
        self
    }

    pub fn add_method_str<S: Into<String>>(self, method_name: S) -> Self {
        self.add_method(MethodMatcher::create().set_method_name_str(method_name))
    }

    pub fn method_count(mut self, count: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::create().count(count));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count(count));
        }
        self
    }

    pub fn method_count_range(mut self, min: u32, max: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::create().count_range(min, max));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count_range(min, max));
        }
        self
    }

    pub fn method_count_min(mut self, min: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::create().count_min(min));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count_min(min));
        }
        self
    }

    pub fn method_count_max(mut self, max: u32) -> Self {
        if self.methods_matcher.is_none() {
            self.methods_matcher = Some(MethodsMatcher::create().count_max(max));
        } else {
            self.methods_matcher = self.methods_matcher.map(|mm| mm.count_max(max));
        }
        self
    }

    // extend using_strings
    pub fn add_using_strings(mut self, using_strings: Vec<StringMatcher>) -> Self {
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

    pub fn add_using_string(mut self, using_string: StringMatcher) -> Self {
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

    pub fn add_using_strings_strs<S: Into<String>>(mut self, using_strings: Vec<S>) -> Self {
        let matchers: Vec<StringMatcher> = using_strings
            .into_iter()
            .map(|s| StringMatcher::create_string_str(s))
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

    pub fn add_using_string_str<S: Into<String>>(self, using_string: S) -> Self {
        self.add_using_string(StringMatcher::create_string_str(using_string))
    }

    pub fn add_eq_using_string_str<S: Into<String>>(self, using_string: S) -> Self {
        self.add_using_string(
            StringMatcher::create()
                .set_value(using_string)
                .set_match_type(StringMatchType::Equals),
        )
    }

    pub fn add_contains_using_string_str<S: Into<String>>(self, using_string: S) -> Self {
        self.add_using_string(
            StringMatcher::create()
                .set_value(using_string)
                .set_match_type(StringMatchType::Contains),
        )
    }
}
