use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBFindMethod, FBFindMethodArgs};
use crate::query::base::BaseQuery;
use crate::query::matchers::MethodMatcher;
use crate::result::base::BaseData;
use crate::result::{ClassData, MethodData};

pub struct FindMethod<'a> {
    search_packages: Option<Vec<String>>,
    exclude_packages: Option<Vec<String>>,
    ignore_packages_case: bool,
    search_classes: Option<Vec<ClassData<'a>>>,
    search_methods: Option<Vec<MethodData<'a>>>,
    find_first: bool,
    matcher: Option<MethodMatcher>,
}

impl<'a> Default for FindMethod<'a> {
    fn default() -> Self {
        Self {
            search_packages: None,
            exclude_packages: None,
            ignore_packages_case: false,
            search_classes: None,
            search_methods: None,
            find_first: false,
            matcher: None,
        }
    }
}

impl<'a> From<FindMethod<'a>> for Vec<u8> {
    fn from(value: FindMethod) -> Self {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = value.inner_build(&mut fbb);
        fbb.finish(root, None);
        fbb.finished_data().to_vec()
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBFindMethod<'a>>> for FindMethod<'a> {
    fn inner_build(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> WIPOffset<FBFindMethod<'a>> {
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
        let in_methods = self.search_methods.as_ref().map(|methods| {
            let ids = methods
                .iter()
                .map(|method| method.get_mine_encode_id())
                .collect::<Vec<i64>>();
            fbb.create_vector(&ids)
        });
        let matcher = self.matcher.as_ref().map(|m| m.inner_build(fbb));

        FBFindMethod::create(
            fbb,
            &FBFindMethodArgs {
                search_packages,
                exclude_packages,
                ignore_packages_case: self.ignore_packages_case,
                in_classes,
                in_methods,
                find_first: self.find_first,
                matcher,
            },
        )
    }
}

impl<'a> FindMethod<'a> {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_search_packages<S>(mut self, packages: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        self.search_packages = Some(packages.into_iter().map(Into::into).collect());
        self
    }

    pub fn set_exclude_packages<S>(mut self, packages: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        self.exclude_packages = Some(packages.into_iter().map(Into::into).collect());
        self
    }

    pub fn set_ignore_packages_case(mut self, ignore: bool) -> Self {
        self.ignore_packages_case = ignore;
        self
    }

    pub fn set_search_classes<V>(mut self, classes: V) -> Self
    where
        V: Into<Vec<ClassData<'a>>>,
    {
        self.search_classes = Some(classes.into());
        self
    }

    pub fn set_search_methods<V>(mut self, fields: V) -> Self
    where
        V: Into<Vec<MethodData<'a>>>,
    {
        self.search_methods = Some(fields.into());
        self
    }

    pub fn set_find_first(mut self, find_first: bool) -> Self {
        self.find_first = find_first;
        self
    }

    pub fn set_matcher(mut self, matcher: MethodMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }

    // extend search_packages
    pub fn add_search_package<S>(mut self, package: S) -> Self
    where
        S: Into<String>,
    {
        self.search_packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }

    // extend exclude_packages
    pub fn add_exclude_package<S>(mut self, package: S) -> Self
    where
        S: Into<String>,
    {
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
    pub fn add_search_method(mut self, field: MethodData<'a>) -> Self {
        self.search_methods.get_or_insert_with(Vec::new).push(field);
        self
    }
}
