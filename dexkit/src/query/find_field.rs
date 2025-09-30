use crate::gen_flatbuffers::dexkit::schema::{
    FindField as FBFindField, FindFieldArgs as FBFindFieldArgs,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::FieldMatcher;
use crate::result::base::BaseData;
use crate::result::{ClassData, FieldData};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct FindField<'a> {
    search_packages: Option<Vec<String>>,
    exclude_packages: Option<Vec<String>>,
    ignore_packages_case: bool,
    search_classes: Option<Vec<ClassData<'a>>>,
    search_fields: Option<Vec<FieldData<'a>>>,
    find_first: bool,
    matcher: Option<FieldMatcher>,
}

impl<'a> Default for FindField<'a> {
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

impl<'a> From<FindField<'a>> for Vec<u8> {
    fn from(value: FindField) -> Self {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = value.inner_build(&mut fbb);
        fbb.finish(root, None);
        fbb.finished_data().to_vec()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBFindField<'a>>> for FindField<'a> {
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

impl<'a> FindField<'a> {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_search_packages<S: Into<String>>(mut self, packages: Vec<S>) -> Self {
        self.search_packages = Some(packages.into_iter().map(Into::into).collect());
        self
    }

    pub fn set_exclude_packages<S: Into<String>>(mut self, packages: Vec<S>) -> Self {
        self.exclude_packages = Some(packages.into_iter().map(Into::into).collect());
        self
    }

    pub fn set_ignore_packages_case(mut self, ignore: bool) -> Self {
        self.ignore_packages_case = ignore;
        self
    }

    pub fn set_search_classes<V: Into<Vec<ClassData<'a>>>>(mut self, classes: V) -> Self {
        self.search_classes = Some(classes.into());
        self
    }

    pub fn set_search_fields<V: Into<Vec<FieldData<'a>>>>(mut self, fields: V) -> Self {
        self.search_fields = Some(fields.into());
        self
    }

    pub fn set_find_first(mut self, find_first: bool) -> Self {
        self.find_first = find_first;
        self
    }

    pub fn set_matcher(mut self, matcher: FieldMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }

    // extend search_packages
    pub fn add_search_package<S: Into<String>>(mut self, package: S) -> Self {
        self.search_packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }

    // extend exclude_packages
    pub fn add_exclude_package<S: Into<String>>(mut self, package: S) -> Self {
        self.exclude_packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }

    // extend search_classes
    pub fn add_search_class(mut self, class: ClassData<'a>) -> Self {
        self.search_classes.get_or_insert_with(Vec::new).push(class);
        self
    }

    // extend search_fields
    pub fn add_search_field(mut self, field: FieldData<'a>) -> Self {
        self.search_fields.get_or_insert_with(Vec::new).push(field);
        self
    }
}
