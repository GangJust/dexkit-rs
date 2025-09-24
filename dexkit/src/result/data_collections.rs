use crate::DexkitBridge;
use crate::gen_flatbuffers::dexkit::schema::{
    ClassMetaArrayHolder as FBClassMetaArrayHolder, FieldMetaArrayHolder as FBFieldMetaArrayHolder,
    MethodMetaArrayHolder as FBMethodMetaArrayHolder,
};
use crate::result::{ClassData, FieldData, MethodData};

/// ClassDataList represents a collection of ClassData objects.
#[derive(Debug)]
pub struct ClassDataList<'a> {
    classes: Vec<ClassData<'a>>,
}

impl<'a> std::ops::Deref for ClassDataList<'a> {
    type Target = Vec<ClassData<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.classes
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

    pub fn size(&self) -> usize {
        self.classes.len()
    }

    pub(crate) fn form_data(dexkit_bridge: &'a DexkitBridge, data: &'a [u8]) -> ClassDataList<'a> {
        // println!("Class data list vector of length: {}", data.len());
        let class_meta_list = flatbuffers::root::<FBClassMetaArrayHolder>(&data).unwrap();
        // println!("Class meta list: {:#?}", class_meta_list);

        let mut class_data_list = Self::new();
        for classes in class_meta_list.classes().iter() {
            for class_meta in classes {
                class_data_list.add(ClassData::with_meta(dexkit_bridge, class_meta));
            }
        }
        class_data_list
    }
}

/// MethodDataList represents a collection of MethodData objects.
#[derive(Debug)]
pub struct MethodDataList<'a> {
    methods: Vec<MethodData<'a>>,
}

impl<'a> std::ops::Deref for MethodDataList<'a> {
    type Target = Vec<MethodData<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.methods
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

    pub fn size(&self) -> usize {
        self.methods.len()
    }

    pub(crate) fn form_data(dexkit_bridge: &'a DexkitBridge, vec: &'a [u8]) -> MethodDataList<'a> {
        // println!("Method data list vector of length: {}", vec.len());
        let method_meta_array = flatbuffers::root::<FBMethodMetaArrayHolder>(&vec).unwrap();
        // println!("Method meta array: {:#?}", method_meta_array);

        let mut method_data_list = Self::new();
        for methods in method_meta_array.methods().iter() {
            for method_meta in methods {
                method_data_list.add(MethodData::with_meta(dexkit_bridge, method_meta));
            }
        }

        method_data_list
    }
}

/// FieldDataList represents a collection of FieldData objects.
#[derive(Debug)]
pub struct FieldDataList<'a> {
    fields: Vec<FieldData<'a>>,
}

impl<'a> std::ops::Deref for FieldDataList<'a> {
    type Target = Vec<FieldData<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl<'a> FieldDataList<'a> {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add(&mut self, field_data: FieldData<'a>) {
        self.fields.push(field_data);
    }

    pub fn size(&self) -> usize {
        self.fields.len()
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
