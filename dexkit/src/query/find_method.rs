use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::fb_codec::{ToFbBytes, finish_fb_bytes};
use crate::gen_flatbuffers::dexkit::fb::{FBFindMethod, FBFindMethodArgs};
use crate::query::base::BaseQuery;
use crate::query::matchers::MethodMatcher;
use crate::result::base::BaseData;
use crate::result::{ClassData, MethodData};

pub struct FindMethod {
    search_packages: Option<Vec<String>>,
    exclude_packages: Option<Vec<String>>,
    ignore_packages_case: bool,
    search_classes: Option<Vec<ClassData>>,
    search_methods: Option<Vec<MethodData>>,
    find_first: bool,
    matcher: Option<MethodMatcher>,
}

impl Default for FindMethod {
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

impl ToFbBytes for FindMethod {
    fn to_fb_bytes(&self) -> Vec<u8> {
        let mut fbb = FlatBufferBuilder::with_capacity(1024);
        let root = self.inner_build(&mut fbb);
        finish_fb_bytes(fbb, root)
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBFindMethod<'a>>> for FindMethod {
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

impl FindMethod {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FindMethod {
    pub fn search_packages<I, S>(mut self, packages: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.search_packages = Some(packages.into_iter().map(Into::into).collect());
        self
    }

    pub fn exclude_packages<I, S>(mut self, packages: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude_packages = Some(packages.into_iter().map(Into::into).collect());
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

    pub fn search_methods<I>(mut self, methods: I) -> Self
    where
        I: IntoIterator<Item = MethodData>,
    {
        self.search_methods = Some(methods.into_iter().collect());
        self
    }

    pub fn first_only(mut self) -> Self {
        self.find_first = true;
        self
    }

    pub fn matcher(mut self, matcher: MethodMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }
}

impl FindMethod {
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

impl FindMethod {
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

impl FindMethod {
    pub fn add_search_class(mut self, class: ClassData) -> Self {
        self.search_classes.get_or_insert_with(Vec::new).push(class);
        self
    }
}

impl FindMethod {
    pub fn add_search_method(mut self, method: MethodData) -> Self {
        self.search_methods
            .get_or_insert_with(Vec::new)
            .push(method);
        self
    }
}
