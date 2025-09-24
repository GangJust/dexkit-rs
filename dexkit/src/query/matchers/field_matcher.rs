use crate::gen_flatbuffers::dexkit::schema::{
    FieldMatcher as FBFieldMatcher, FieldMatcherArgs as FBFieldMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::AnnotationsMatcher;
use crate::query::matchers::ClassMatcher;
use crate::query::matchers::MethodsMatcher;
use crate::query::matchers::base::AccessFlagsMatcher;
use crate::query::matchers::base::StringMatcher;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

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
    pub fn set_modifiers_u32<U: Into<u32>>(self, modifiers: U) -> Self {
        self.set_modifiers_matcher(AccessFlagsMatcher::default().set_modifiers(modifiers.into()))
    }
}
