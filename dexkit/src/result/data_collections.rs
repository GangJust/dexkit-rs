use std::collections::HashMap;

use crate::DexkitBridge;
use crate::gen_flatbuffers::dexkit::schema::{
    BatchClassMetaArrayHolder as FBBatchClassMetaArrayHolder,
    BatchMethodMetaArrayHolder as FBBatchMethodMetaArrayHolder,
    ClassMetaArrayHolder as FBClassMetaArrayHolder, FieldMetaArrayHolder as FBFieldMetaArrayHolder,
    MethodMetaArrayHolder as FBMethodMetaArrayHolder,
};
use crate::query::{FindClass, FindField, FindMethod};
use crate::result::base::BaseData;
use crate::result::{ClassData, FieldData, MethodData};

pub trait BaseDataList<'a, T> {
    fn size(&self) -> usize;

    fn single(&self) -> Option<&T>;

    fn single_where(&self, predicate: impl Fn(&T) -> bool) -> Option<&T>;
}

/// ClassDataList represents a collection of ClassData objects.
#[derive(Debug, Clone)]
pub struct ClassDataList<'a> {
    classes: Vec<ClassData<'a>>,
}

impl<'a> std::ops::Deref for ClassDataList<'a> {
    type Target = Vec<ClassData<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.classes
    }
}

impl<'a> From<ClassDataList<'a>> for Vec<ClassData<'a>> {
    fn from(value: ClassDataList<'a>) -> Self {
        value.classes
    }
}

impl<'a> BaseDataList<'a, ClassData<'a>> for ClassDataList<'a> {
    fn size(&self) -> usize {
        self.classes.len()
    }

    fn single(&self) -> Option<&ClassData<'a>> {
        if self.classes.len() == 1 {
            Some(&self.classes[0])
        } else {
            None
        }
    }

    fn single_where(&self, predicate: impl Fn(&ClassData<'a>) -> bool) -> Option<&ClassData<'a>> {
        let filtered: Vec<&ClassData<'a>> = self.classes.iter().filter(|&c| predicate(c)).collect();
        if filtered.len() == 1 {
            Some(filtered[0])
        } else {
            None
        }
    }
}

impl<'a> ClassDataList<'a> {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    pub fn add(&mut self, class_data: ClassData<'a>) {
        self.classes.push(class_data);
    }

    pub fn find_class(&self, find_class: FindClass<'a>) -> ClassDataList<'_> {
        if self.classes.is_empty() {
            return ClassDataList::new();
        }

        let first = &self.classes[0];
        let bridge = first.bridge();
        let find_class = find_class.set_search_classes(self.clone());
        bridge.find_class(find_class)
    }

    pub fn find_method(&self, find_method: FindMethod<'a>) -> MethodDataList<'_> {
        if self.classes.is_empty() {
            return MethodDataList::new();
        }

        let first = &self.classes[0];
        let bridge = first.bridge();
        let find_method = find_method.set_search_classes(self.clone());
        bridge.find_method(find_method)
    }

    pub fn find_field(&self, find_field: FindField<'a>) -> FieldDataList<'_> {
        if self.classes.is_empty() {
            return FieldDataList::new();
        }

        let first = &self.classes[0];
        let bridge = first.bridge();
        let find_field = find_field.set_search_classes(self.clone());
        bridge.find_field(find_field)
    }

    /// ...
    pub(crate) fn from_data(bridge: &'a DexkitBridge, data: &'a [u8]) -> ClassDataList<'a> {
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
        bridge: &'a DexkitBridge,
        data: &'a [u8],
    ) -> HashMap<String, ClassDataList<'a>> {
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
pub struct MethodDataList<'a> {
    methods: Vec<MethodData<'a>>,
}

impl<'a> std::ops::Deref for MethodDataList<'a> {
    type Target = Vec<MethodData<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.methods
    }
}

impl<'a> From<MethodDataList<'a>> for Vec<MethodData<'a>> {
    fn from(value: MethodDataList<'a>) -> Self {
        value.methods
    }
}

impl<'a> BaseDataList<'a, MethodData<'a>> for MethodDataList<'a> {
    fn size(&self) -> usize {
        self.methods.len()
    }

    fn single(&self) -> Option<&MethodData<'a>> {
        if self.methods.len() == 1 {
            Some(&self.methods[0])
        } else {
            None
        }
    }

    fn single_where(&self, predicate: impl Fn(&MethodData<'a>) -> bool) -> Option<&MethodData<'a>> {
        let filtered: Vec<&MethodData<'a>> =
            self.methods.iter().filter(|&c| predicate(c)).collect();
        if filtered.len() == 1 {
            Some(filtered[0])
        } else {
            None
        }
    }
}

impl<'a> MethodDataList<'a> {
    pub fn new() -> Self {
        Self {
            methods: Vec::new(),
        }
    }

    pub fn add(&mut self, method_data: MethodData<'a>) {
        self.methods.push(method_data);
    }

    pub fn find_method(&self, find_method: FindMethod<'a>) -> MethodDataList<'_> {
        if self.methods.is_empty() {
            return MethodDataList::new();
        }

        let first = &self.methods[0];
        let bridge = first.bridge();
        let find_method = find_method.set_search_methods(self.clone());
        bridge.find_method(find_method)
    }

    pub(crate) fn form_data(bridge: &'a DexkitBridge, vec: &'a [u8]) -> MethodDataList<'a> {
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
        bridge: &'a DexkitBridge,
        data: &'a [u8],
    ) -> HashMap<String, MethodDataList<'a>> {
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
pub struct FieldDataList<'a> {
    fields: Vec<FieldData<'a>>,
}

impl<'a> std::ops::Deref for FieldDataList<'a> {
    type Target = Vec<FieldData<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl<'a> From<FieldDataList<'a>> for Vec<FieldData<'a>> {
    fn from(value: FieldDataList<'a>) -> Self {
        value.fields
    }
}

impl<'a> BaseDataList<'a, FieldData<'a>> for FieldDataList<'a> {
    fn size(&self) -> usize {
        self.fields.len()
    }

    fn single(&self) -> Option<&FieldData<'a>> {
        if self.fields.len() == 1 {
            Some(&self.fields[0])
        } else {
            None
        }
    }

    fn single_where(&self, predicate: impl Fn(&FieldData<'a>) -> bool) -> Option<&FieldData<'a>> {
        let filtered: Vec<&FieldData<'a>> = self.fields.iter().filter(|&c| predicate(c)).collect();
        if filtered.len() == 1 {
            Some(filtered[0])
        } else {
            None
        }
    }
}

impl<'a> FieldDataList<'a> {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add(&mut self, field_data: FieldData<'a>) {
        self.fields.push(field_data);
    }

    pub fn find_field(&self, find_field: FindField<'a>) -> FieldDataList<'_> {
        if self.fields.is_empty() {
            return FieldDataList::new();
        }

        let first = &self.fields[0];
        let bridge = first.bridge();
        let find_field = find_field.set_search_fields(self.clone());
        bridge.find_field(find_field)
    }

    pub(crate) fn form_data(dexkit_bridge: &'a DexkitBridge, vec: &'a [u8]) -> FieldDataList<'a> {
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
