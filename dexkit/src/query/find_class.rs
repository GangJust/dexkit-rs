use crate::gen_flatbuffers::dexkit::schema::{
    FindClass as FBFindClass, FindClassArgs as FBFindClassArgs,
};
use crate::query::base::BaseQuery;
use crate::query::matchers::ClassMatcher;
use crate::result::ClassData;
use crate::result::base::BaseData;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct FindClass<'a> {
    search_packages: Option<Vec<String>>,
    exclude_packages: Option<Vec<String>>,
    ignore_packages_case: bool,
    search_classes: Option<Vec<ClassData<'a>>>,
    find_first: bool,
    matcher: Option<ClassMatcher>,
}

impl<'a> Default for FindClass<'a> {
    fn default() -> Self {
        Self {
            search_packages: None,
            exclude_packages: None,
            ignore_packages_case: false,
            search_classes: None,
            find_first: false,
            matcher: None,
        }
    }
}

impl<'a> From<FindClass<'a>> for Vec<u8> {
    fn from(value: FindClass) -> Self {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = value.inner_build(&mut fbb);
        fbb.finish(root, None);
        fbb.finished_data().to_vec()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBFindClass<'a>>> for FindClass<'a> {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBFindClass<'a>> {
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
                .map(|class| class.get_encode_id())
                .collect::<Vec<i64>>();
            fbb.create_vector(&ids)
        });

        let matcher = self.matcher.as_ref().map(|m| m.inner_build(fbb));

        FBFindClass::create(
            fbb,
            &FBFindClassArgs {
                search_packages,
                exclude_packages,
                ignore_packages_case: self.ignore_packages_case,
                in_classes,
                find_first: self.find_first,
                matcher,
            },
        )
    }
}

impl<'a> FindClass<'a> {
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

    pub fn set_search_classes(mut self, classes: Vec<ClassData<'a>>) -> Self {
        self.search_classes = Some(classes);
        self
    }

    pub fn set_find_first(mut self, find_first: bool) -> Self {
        self.find_first = find_first;
        self
    }

    pub fn set_matcher(mut self, matcher: ClassMatcher) -> Self {
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

    // extend matcher
    pub fn class_name<S: Into<String>>(mut self, class_name: S) -> Self {
        if self.matcher.is_none() {
            self.matcher = Some(ClassMatcher::create().set_class_name_str(class_name));
        } else {
            self.matcher = self.matcher.map(|m| m.set_class_name_str(class_name));
        }
        self
    }
}
