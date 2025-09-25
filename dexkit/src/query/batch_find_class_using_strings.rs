use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::schema::{
    BatchFindClassUsingStrings as FBBatchFindClassUsingStrings,
    BatchFindClassUsingStringsArgs as FBBatchFindClassUsingStringsArgs,
};
use crate::{
    query::{base::BaseQuery, matchers::StringMatchersGroup},
    result::{ClassData, base::BaseData},
};

pub struct BatchFindClassUsingStrings<'a> {
    search_packages: Option<Vec<String>>,
    exclude_packages: Option<Vec<String>>,
    ignore_packages_case: bool,
    search_classes: Option<Vec<ClassData<'a>>>,
    search_groups: Option<Vec<StringMatchersGroup>>,
}

impl<'a> Default for BatchFindClassUsingStrings<'a> {
    fn default() -> Self {
        Self {
            search_packages: None,
            exclude_packages: None,
            ignore_packages_case: false,
            search_classes: None,
            search_groups: None,
        }
    }
}

impl<'a> From<BatchFindClassUsingStrings<'a>> for Vec<u8> {
    fn from(value: BatchFindClassUsingStrings) -> Self {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = value.inner_build(&mut fbb);
        fbb.finish(root, None);
        fbb.finished_data().to_vec()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBBatchFindClassUsingStrings<'a>>>
    for BatchFindClassUsingStrings<'a>
{
    fn inner_build(
        &self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBBatchFindClassUsingStrings<'a>> {
        let search_packages = self.search_packages.as_ref().map(|packages| {
            let packages_offsets: Vec<_> = packages.iter().map(|p| fbb.create_string(p)).collect();
            fbb.create_vector(&packages_offsets)
        });
        let exclude_packages = self.exclude_packages.as_ref().map(|packages| {
            let packages_offsets: Vec<_> = packages.iter().map(|p| fbb.create_string(p)).collect();
            fbb.create_vector(&packages_offsets)
        });
        let ignore_packages_case = self.ignore_packages_case;
        let in_classes = self.search_classes.as_ref().map(|classes| {
            let ids = classes
                .iter()
                .map(|class| class.get_mine_encode_id())
                .collect::<Vec<i64>>();
            fbb.create_vector(&ids)
        });
        let matchers = self.search_groups.as_ref().map(|groups| {
            let groups_offsets: Vec<_> = groups.iter().map(|g| g.inner_build(fbb)).collect();
            fbb.create_vector(&groups_offsets)
        });

        FBBatchFindClassUsingStrings::create(
            fbb,
            &FBBatchFindClassUsingStringsArgs {
                search_packages,
                exclude_packages,
                ignore_packages_case,
                in_classes,
                matchers,
            },
        )
    }
}

impl<'a> BatchFindClassUsingStrings<'a> {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_search_packages<S: Into<String>>(mut self, packages: Vec<S>) -> Self {
        self.search_packages = Some(packages.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn set_exclude_packages<S: Into<String>>(mut self, packages: Vec<S>) -> Self {
        self.exclude_packages = Some(packages.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn set_ignore_packages_case(mut self, ignore: bool) -> Self {
        self.ignore_packages_case = ignore;
        self
    }

    pub fn set_search_classes(mut self, classes: Vec<ClassData<'a>>) -> Self {
        self.search_classes = Some(classes);
        self
    }

    pub fn set_groups(mut self, groups: Vec<StringMatchersGroup>) -> Self {
        self.search_groups = Some(groups);
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

    // extend groups
    pub fn add_group(mut self, group: StringMatchersGroup) -> Self {
        self.search_groups.get_or_insert_with(Vec::new).push(group);
        self
    }

    pub fn add_groups(mut self, groups: Vec<StringMatchersGroup>) -> Self {
        self.search_groups
            .get_or_insert_with(Vec::new)
            .extend(groups);
        self
    }
}
