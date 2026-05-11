use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::fb_codec::{ToFbBytes, finish_fb_bytes};
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

impl ToFbBytes for BatchFindClassUsingStrings {
    fn to_fb_bytes(&self) -> Vec<u8> {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = self.inner_build(&mut fbb);
        finish_fb_bytes(fbb, root)
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
}

impl BatchFindClassUsingStrings {
    pub fn search_packages<I, S>(mut self, packages: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.search_packages = Some(packages.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exclude_packages<I, S>(mut self, packages: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude_packages = Some(packages.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn ignore_packages_case(mut self, ignore: bool) -> Self {
        self.ignore_packages_case = ignore;
        self
    }

    pub fn search_classes<I>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = ClassData>,
    {
        self.search_classes = Some(classes.into_iter().collect());
        self
    }

    pub fn groups<I>(mut self, groups: I) -> Self
    where
        I: IntoIterator<Item = StringMatchersGroup>,
    {
        self.search_groups = Some(groups.into_iter().collect());
        self
    }
}

impl BatchFindClassUsingStrings {
    pub fn add_search_package<S>(mut self, package: S) -> Self
    where
        S: Into<String>,
    {
        self.search_packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }
}

impl BatchFindClassUsingStrings {
    pub fn add_exclude_package<S>(mut self, package: S) -> Self
    where
        S: Into<String>,
    {
        self.exclude_packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }
}

impl BatchFindClassUsingStrings {
    pub fn add_search_class(mut self, class: ClassData) -> Self {
        self.search_classes.get_or_insert_with(Vec::new).push(class);
        self
    }
}

impl BatchFindClassUsingStrings {
    pub fn add_group(mut self, group: StringMatchersGroup) -> Self {
        self.search_groups.get_or_insert_with(Vec::new).push(group);
        self
    }
}
