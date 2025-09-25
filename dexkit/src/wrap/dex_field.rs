use crate::uitls::DexSignature;

#[derive(Debug, Clone)]
pub struct DexField {
    descriptor: String,
    class_name: String,
    field_name: String,
    type_name: String,
}

impl DexField {
    /// Parse a field descriptor string into a DexField struct.
    /// The descriptor should be in the format "Lcom/example/MyClass;->fieldName:Type;"
    /// # Arguments
    /// * `descriptor` - A string slice that holds the field descriptor. e.g. "Lcom/example/MyClass;->myField:I
    /// # Returns
    /// * `Option<DexField>` - Returns Some(DexField) if parsing is successful, otherwise None.
    /// # Examples
    /// ```
    /// let field = DexField::deserialize("Lcom/example/MyClass;->myField:I").unwrap();
    /// assert_eq!(field.class_name(), "com.example.MyClass");
    /// assert_eq!(field.field_name(), "myField");
    /// assert_eq!(field.type_name(), "int");
    /// assert_eq!(field.get_type_signature().unwrap(), "I");
    /// assert_eq!(field.descriptor(), "Lcom/example/MyClass;->myField:I");
    /// ```
    pub fn deserialize<T>(descriptor: T) -> Option<Self>
    where
        T: AsRef<str>,
    {
        let desc = descriptor.as_ref();
        let idx1 = desc.find("->")?;
        let idx2 = desc[idx1 + 2..].find(':')? + idx1 + 2;

        if idx1 == 0 || idx2 <= idx1 + 2 || idx2 >= desc.len() - 1 {
            return None;
        }

        let class_name = DexSignature::get_type_name(&desc[0..idx1])?;
        let field_name = desc[idx1 + 2..idx2].to_string();
        let type_name = DexSignature::get_type_name(&desc[idx2 + 1..])?;
        Some(Self {
            descriptor: desc.to_string(),
            class_name,
            field_name,
            type_name,
        })
    }

    /// Get the original field descriptor string.
    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    /// Get the class name where the field is declared.
    pub fn class_name(&self) -> String {
        self.class_name.clone()
    }

    /// Get the field name.
    pub fn field_name(&self) -> String {
        self.field_name.clone()
    }

    /// Get the type name of the field.
    pub fn type_name(&self) -> String {
        self.type_name.clone()
    }

    /// Get the class name where the field is declared.
    pub fn declared_class_name(&self) -> String {
        self.class_name.clone()
    }

    /// Get the type signature of the field. e.g. "int" -> "I"
    pub fn type_signature(&self) -> Option<String> {
        DexSignature::get_type_signature(&self.type_name)
    }
}

impl PartialEq for DexField {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}
