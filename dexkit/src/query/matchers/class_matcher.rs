use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBClassMatcher, FBClassMatcherArgs};
use crate::query::base::BaseQuery;
use crate::query::base::IAnnotationEncodeValue;
use crate::query::enums::MatchType;
use crate::query::matchers::interfaces_matcher::InterfacesMatcher;
use crate::query::matchers::{
    AccessFlagsMatcher, AnnotationsMatcher, FieldsMatcher, MethodsMatcher, StringMatcher,
};

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
        Self {
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
        Self::default()
    }
}

impl ClassMatcher {
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

    pub fn interfaces(mut self, matcher: InterfacesMatcher) -> Self {
        self.interfaces_matcher = Some(matcher);
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

    pub fn using_strings<I>(mut self, using_strings_matcher: I) -> Self
    where
        I: IntoIterator<Item = StringMatcher>,
    {
        self.using_strings_matcher = Some(using_strings_matcher.into_iter().collect());
        self
    }
}

impl ClassMatcher {
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

impl ClassMatcher {
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
}

impl ClassMatcher {
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
}
