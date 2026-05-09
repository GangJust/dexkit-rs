use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{
    FBBatchFindClassUsingStrings, FBBatchFindClassUsingStringsArgs,
};
use crate::{
    query::{base::BaseQuery, matchers::StringMatchersGroup},
    result::{ClassData, base::BaseData},
};

pub struct BatchFindClassUsingStrings {
    search_packages: Option<Vec<String>>,
    exclude_packages: Option<Vec<String>>,
    ignore_packages_case: bool,
    search_classes: Option<Vec<ClassData>>,
    search_groups: Option<Vec<StringMatchersGroup>>,
}

impl Default for BatchFindClassUsingStrings {
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

impl From<BatchFindClassUsingStrings> for Vec<u8> {
    fn from(value: BatchFindClassUsingStrings) -> Self {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = value.inner_build(&mut fbb);
        fbb.finish(root, None);
        fbb.finished_data().to_vec()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBBatchFindClassUsingStrings<'a>>> for BatchFindClassUsingStrings {
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

impl BatchFindClassUsingStrings {
    pub fn new() -> Self {
        Self::default()
    }

    // base
    pub fn search_packages<S>(mut self, packages: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        self.search_packages = Some(packages.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exclude_packages<S>(mut self, packages: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        self.exclude_packages = Some(packages.into_iter().map(|s| s.into()).collect());
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

    pub fn groups(mut self, groups: Vec<StringMatchersGroup>) -> Self {
        self.search_groups = Some(groups);
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

    // extend groups
    pub fn group(mut self, group: StringMatchersGroup) -> Self {
        self.search_groups.get_or_insert_with(Vec::new).push(group);
        self
    }
}
