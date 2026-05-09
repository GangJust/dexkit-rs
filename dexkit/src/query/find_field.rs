use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::fb_codec::{ToFbBytes, finish_fb_bytes};
use crate::gen_flatbuffers::dexkit::fb::{FBFindField, FBFindFieldArgs};
use crate::query::base::BaseQuery;
use crate::query::matchers::FieldMatcher;
use crate::result::base::BaseData;
use crate::result::{ClassData, FieldData};

pub struct FindField {
    search_packages: Option<Vec<String>>,
    exclude_packages: Option<Vec<String>>,
    ignore_packages_case: bool,
    search_classes: Option<Vec<ClassData>>,
    search_fields: Option<Vec<FieldData>>,
    find_first: bool,
    matcher: Option<FieldMatcher>,
}

impl Default for FindField {
    fn default() -> Self {
        Self {
            search_packages: None,
            exclude_packages: None,
            ignore_packages_case: false,
            search_classes: None,
            search_fields: None,
            find_first: false,
            matcher: None,
        }
    }
}

impl ToFbBytes for FindField {
    fn to_fb_bytes(&self) -> Vec<u8> {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = self.inner_build(&mut fbb);
        finish_fb_bytes(fbb, root)
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBFindField<'a>>> for FindField {
    fn inner_build(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBFindField<'a>> {
        let search_packages = self.search_packages.as_ref().map(|packages| {
            let packages_offsets: Vec<_> = packages.iter().map(|p| fbb.create_string(p)).collect();
            fbb.create_vector(&packages_offsets)
        });
        let exclude_packages = self.exclude_packages.as_ref().map(|packages| {
            let packages_offsets: Vec<_> = packages.iter().map(|p| fbb.create_string(p)).collect();
            fbb.create_vector(&packages_offsets)
        });
        let in_classes = self.search_classes.as_ref().map(|classes| {
            let ids = classes
                .iter()
                .map(|class| class.get_mine_encode_id())
                .collect::<Vec<i64>>();
            fbb.create_vector(&ids)
        });
        let in_fields = self.search_fields.as_ref().map(|fields| {
            let ids = fields
                .iter()
                .map(|field| field.get_mine_encode_id())
                .collect::<Vec<i64>>();
            fbb.create_vector(&ids)
        });

        let matcher = self.matcher.as_ref().map(|m| m.inner_build(fbb));

        let root = FBFindField::create(
            fbb,
            &FBFindFieldArgs {
                search_packages,
                exclude_packages,
                ignore_packages_case: self.ignore_packages_case,
                in_classes,
                in_fields,
                find_first: self.find_first,
                matcher,
            },
        );
        root
    }
}

impl FindField {
    pub fn new() -> Self {
        Self::default()
    }

    // base
    pub fn search_packages<S>(mut self, packages: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        self.search_packages = Some(packages.into_iter().map(Into::into).collect());
        self
    }

    pub fn exclude_packages<S>(mut self, packages: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        self.exclude_packages = Some(packages.into_iter().map(Into::into).collect());
        self
    }

    pub fn ignore_packages_case(mut self, ignore: bool) -> Self {
        self.ignore_packages_case = ignore;
        self
    }

    pub fn search_classes<V>(mut self, classes: V) -> Self
    where
        V: Into<Vec<ClassData>>,
    {
        self.search_classes = Some(classes.into());
        self
    }

    pub fn search_fields<V>(mut self, fields: V) -> Self
    where
        V: Into<Vec<FieldData>>,
    {
        self.search_fields = Some(fields.into());
        self
    }

    pub fn first_only(mut self) -> Self {
        self.find_first = true;
        self
    }

    pub fn matcher(mut self, matcher: FieldMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }

    // extend search_packages
    pub fn search_package<S>(mut self, package: S) -> Self
    where
        S: Into<String>,
    {
        self.search_packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }

    // extend exclude_packages
    pub fn exclude_package<S>(mut self, package: S) -> Self
    where
        S: Into<String>,
    {
        self.exclude_packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }

    // extend search_classes
    pub fn search_class(mut self, class: ClassData) -> Self {
        self.search_classes.get_or_insert_with(Vec::new).push(class);
        self
    }

    // extend search_fields
    pub fn search_field(mut self, field: FieldData) -> Self {
        self.search_fields.get_or_insert_with(Vec::new).push(field);
        self
    }
}
