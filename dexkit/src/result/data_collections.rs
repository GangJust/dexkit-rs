use std::collections::HashMap;

use crate::DexkitBridge;
use crate::gen_flatbuffers::dexkit::fb::{
    FBBatchClassMetaArrayHolder, FBBatchMethodMetaArrayHolder, FBClassMetaArrayHolder,
    FBFieldMetaArrayHolder, FBMethodMetaArrayHolder,
};
use crate::query::{FindClass, FindField, FindMethod};
use crate::result::base::BaseData;
use crate::result::{ClassData, FieldData, MethodData};

pub(crate) trait BaseDataList<T> {
    fn size(&self) -> usize;

    fn single(&self) -> Option<&T>;

    fn single_where(&self, predicate: impl Fn(&T) -> bool) -> Option<&T>;
}

/// ClassDataList represents a collection of ClassData objects.
#[derive(Debug, Clone)]
pub struct ClassDataList {
    classes: Vec<ClassData>,
}

impl std::ops::Deref for ClassDataList {
    type Target = Vec<ClassData>;

    fn deref(&self) -> &Self::Target {
        &self.classes
    }
}

impl From<ClassDataList> for Vec<ClassData> {
    fn from(value: ClassDataList) -> Self {
        value.classes
    }
}

impl AsRef<[ClassData]> for ClassDataList {
    fn as_ref(&self) -> &[ClassData] {
        &self.classes
    }
}

impl IntoIterator for ClassDataList {
    type Item = ClassData;
    type IntoIter = std::vec::IntoIter<ClassData>;

    fn into_iter(self) -> Self::IntoIter {
        self.classes.into_iter()
    }
}

impl<'a> IntoIterator for &'a ClassDataList {
    type Item = &'a ClassData;
    type IntoIter = std::slice::Iter<'a, ClassData>;

    fn into_iter(self) -> Self::IntoIter {
        self.classes.iter()
    }
}

impl BaseDataList<ClassData> for ClassDataList {
    fn size(&self) -> usize {
        self.classes.len()
    }

    fn single(&self) -> Option<&ClassData> {
        if self.classes.len() == 1 {
            Some(&self.classes[0])
        } else {
            None
        }
    }

    fn single_where(&self, predicate: impl Fn(&ClassData) -> bool) -> Option<&ClassData> {
        let filtered: Vec<&ClassData> = self.classes.iter().filter(|&c| predicate(c)).collect();
        if filtered.len() == 1 {
            Some(filtered[0])
        } else {
            None
        }
    }
}

impl ClassDataList {
    pub(crate) fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, class_data: ClassData) {
        self.classes.push(class_data);
    }

    pub fn len(&self) -> usize {
        self.classes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ClassData> {
        self.classes.iter()
    }

    pub fn single(&self) -> Option<&ClassData> {
        BaseDataList::single(self)
    }

    pub fn single_where(&self, predicate: impl Fn(&ClassData) -> bool) -> Option<&ClassData> {
        BaseDataList::single_where(self, predicate)
    }

    pub fn find_class(&self, find_class: FindClass) -> ClassDataList {
        if self.classes.is_empty() {
            return ClassDataList::new();
        }

        let first = &self.classes[0];
        let bridge = first.bridge();
        let find_class = find_class.search_classes(self.clone());
        bridge.find_class(find_class)
    }

    pub fn find_method(&self, find_method: FindMethod) -> MethodDataList {
        if self.classes.is_empty() {
            return MethodDataList::new();
        }

        let first = &self.classes[0];
        let bridge = first.bridge();
        let find_method = find_method.search_classes(self.clone());
        bridge.find_method(find_method)
    }

    pub fn find_field(&self, find_field: FindField) -> FieldDataList {
        if self.classes.is_empty() {
            return FieldDataList::new();
        }

        let first = &self.classes[0];
        let bridge = first.bridge();
        let find_field = find_field.search_classes(self.clone());
        bridge.find_field(find_field)
    }

    /// ...
    pub(crate) fn from_data(bridge: &DexkitBridge, data: &[u8]) -> ClassDataList {
        // println!("Class data list vector of length: {}", data.len());
        let class_meta_list = flatbuffers::root::<FBClassMetaArrayHolder>(&data).unwrap();
        // println!("Class meta list: {:#?}", class_meta_list);

        let mut class_data_list = Self::new();
        for classes in class_meta_list.classes().iter() {
            for class_meta in classes {
                class_data_list.add(ClassData::with_meta(bridge, class_meta));
            }
        }
        class_data_list
    }

    /// ...
    pub(crate) fn from_batch_data(
        bridge: &DexkitBridge,
        data: &[u8],
    ) -> HashMap<String, ClassDataList> {
        // println!("Batch class data list vector of length: {}", data.len());
        let batch_class_meta_list =
            flatbuffers::root::<FBBatchClassMetaArrayHolder>(&data).unwrap();
        // println!("Batch class meta list: {:#?}", batch_class_meta_list);

        batch_class_meta_list
            .items()
            .iter()
            .flat_map(|class_meta_list| {
                class_meta_list.iter().map(|class_meta| {
                    let union_key = class_meta.union_key();
                    let mut class_data_list = ClassDataList::new();
                    for class_meta in class_meta.classes().iter() {
                        for class_meta in class_meta {
                            class_data_list.add(ClassData::with_meta(bridge, class_meta));
                        }
                    }
                    (union_key.unwrap_or_default().to_string(), class_data_list)
                })
            })
            .collect::<HashMap<_, _>>()
    }
}

/// MethodDataList represents a collection of MethodData objects.
#[derive(Debug, Clone)]
pub struct MethodDataList {
    methods: Vec<MethodData>,
}

impl std::ops::Deref for MethodDataList {
    type Target = Vec<MethodData>;

    fn deref(&self) -> &Self::Target {
        &self.methods
    }
}

impl From<MethodDataList> for Vec<MethodData> {
    fn from(value: MethodDataList) -> Self {
        value.methods
    }
}

impl AsRef<[MethodData]> for MethodDataList {
    fn as_ref(&self) -> &[MethodData] {
        &self.methods
    }
}

impl IntoIterator for MethodDataList {
    type Item = MethodData;
    type IntoIter = std::vec::IntoIter<MethodData>;

    fn into_iter(self) -> Self::IntoIter {
        self.methods.into_iter()
    }
}

impl<'a> IntoIterator for &'a MethodDataList {
    type Item = &'a MethodData;
    type IntoIter = std::slice::Iter<'a, MethodData>;

    fn into_iter(self) -> Self::IntoIter {
        self.methods.iter()
    }
}

impl BaseDataList<MethodData> for MethodDataList {
    fn size(&self) -> usize {
        self.methods.len()
    }

    fn single(&self) -> Option<&MethodData> {
        if self.methods.len() == 1 {
            Some(&self.methods[0])
        } else {
            None
        }
    }

    fn single_where(&self, predicate: impl Fn(&MethodData) -> bool) -> Option<&MethodData> {
        let filtered: Vec<&MethodData> =
            self.methods.iter().filter(|&c| predicate(c)).collect();
        if filtered.len() == 1 {
            Some(filtered[0])
        } else {
            None
        }
    }
}

impl MethodDataList {
    pub(crate) fn new() -> Self {
        Self {
            methods: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, method_data: MethodData) {
        self.methods.push(method_data);
    }

    pub fn len(&self) -> usize {
        self.methods.len()
    }

    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MethodData> {
        self.methods.iter()
    }

    pub fn single(&self) -> Option<&MethodData> {
        BaseDataList::single(self)
    }

    pub fn single_where(&self, predicate: impl Fn(&MethodData) -> bool) -> Option<&MethodData> {
        BaseDataList::single_where(self, predicate)
    }

    pub fn find_method(&self, find_method: FindMethod) -> MethodDataList {
        if self.methods.is_empty() {
            return MethodDataList::new();
        }

        let first = &self.methods[0];
        let bridge = first.bridge();
        let find_method = find_method.search_methods(self.clone());
        bridge.find_method(find_method)
    }

    pub(crate) fn form_data(bridge: &DexkitBridge, vec: &[u8]) -> MethodDataList {
        // println!("Method data list vector of length: {}", vec.len());
        let method_meta_array = flatbuffers::root::<FBMethodMetaArrayHolder>(&vec).unwrap();
        // println!("Method meta array: {:#?}", method_meta_array);

        let mut method_data_list = Self::new();
        for methods in method_meta_array.methods().iter() {
            for method_meta in methods {
                method_data_list.add(MethodData::with_meta(bridge, method_meta));
            }
        }

        method_data_list
    }

    pub(crate) fn from_batch_data(
        bridge: &DexkitBridge,
        data: &[u8],
    ) -> HashMap<String, MethodDataList> {
        // println!("Batch method data list vector of length: {}", data.len());
        let batch_method_meta_list =
            flatbuffers::root::<FBBatchMethodMetaArrayHolder>(&data).unwrap();
        // println!("Batch method meta list: {:#?}", batch_method_meta_list);

        batch_method_meta_list
            .items()
            .iter()
            .flat_map(|method_meta_list| {
                method_meta_list.iter().map(|method_meta| {
                    let union_key = method_meta.union_key();
                    let mut method_data_list = MethodDataList::new();
                    for method_meta in method_meta.methods().iter() {
                        for method_meta in method_meta {
                            method_data_list.add(MethodData::with_meta(bridge, method_meta));
                        }
                    }
                    (union_key.unwrap_or_default().to_string(), method_data_list)
                })
            })
            .collect::<HashMap<_, _>>()
    }
}

/// FieldDataList represents a collection of FieldData objects.
#[derive(Debug, Clone)]
pub struct FieldDataList {
    fields: Vec<FieldData>,
}

impl std::ops::Deref for FieldDataList {
    type Target = Vec<FieldData>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl From<FieldDataList> for Vec<FieldData> {
    fn from(value: FieldDataList) -> Self {
        value.fields
    }
}

impl AsRef<[FieldData]> for FieldDataList {
    fn as_ref(&self) -> &[FieldData] {
        &self.fields
    }
}

impl IntoIterator for FieldDataList {
    type Item = FieldData;
    type IntoIter = std::vec::IntoIter<FieldData>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl<'a> IntoIterator for &'a FieldDataList {
    type Item = &'a FieldData;
    type IntoIter = std::slice::Iter<'a, FieldData>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}

impl BaseDataList<FieldData> for FieldDataList {
    fn size(&self) -> usize {
        self.fields.len()
    }

    fn single(&self) -> Option<&FieldData> {
        if self.fields.len() == 1 {
            Some(&self.fields[0])
        } else {
            None
        }
    }

    fn single_where(&self, predicate: impl Fn(&FieldData) -> bool) -> Option<&FieldData> {
        let filtered: Vec<&FieldData> = self.fields.iter().filter(|&c| predicate(c)).collect();
        if filtered.len() == 1 {
            Some(filtered[0])
        } else {
            None
        }
    }
}

impl FieldDataList {
    pub(crate) fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub(crate) fn add(&mut self, field_data: FieldData) {
        self.fields.push(field_data);
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FieldData> {
        self.fields.iter()
    }

    pub fn single(&self) -> Option<&FieldData> {
        BaseDataList::single(self)
    }

    pub fn single_where(&self, predicate: impl Fn(&FieldData) -> bool) -> Option<&FieldData> {
        BaseDataList::single_where(self, predicate)
    }

    pub fn find_field(&self, find_field: FindField) -> FieldDataList {
        if self.fields.is_empty() {
            return FieldDataList::new();
        }

        let first = &self.fields[0];
        let bridge = first.bridge();
        let find_field = find_field.search_fields(self.clone());
        bridge.find_field(find_field)
    }

    pub(crate) fn form_data(dexkit_bridge: &DexkitBridge, vec: &[u8]) -> FieldDataList {
        // println!("Field data list vector of length: {}", vec.len());
        let field_meta_array = flatbuffers::root::<FBFieldMetaArrayHolder>(&vec).unwrap();
        // println!("Field meta array: {:#?}", field_meta_array);

        let mut field_data_list = Self::new();
        for fields in field_meta_array.fields().iter() {
            for field_meta in fields {
                field_data_list.add(FieldData::with_meta(dexkit_bridge, field_meta));
            }
        }

        field_data_list
    }
}
