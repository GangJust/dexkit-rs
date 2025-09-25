use crate::uitls::DexSignature;

#[derive(Debug, Clone)]
pub struct DexClass {
    descriptor: String,
    type_name: String,
}

impl DexClass {
    /// Create a new DexClass from a class descriptor. e.g. "Lcom/example/MyClass;"
    /// # Arguments
    /// * `descriptor` - The class descriptor to create the DexClass from. e.g. "Lcom/example/MyClass;"
    /// # Returns
    /// * `Option<DexClass>` - The DexClass if the descriptor is valid, otherwise None
    /// # Examples
    /// ```
    /// let dex_class = DexClass::deserialize("Lcom/example/MyClass;").unwrap();
    /// assert_eq!(dex_class.class_name(), "com.example.MyClass");
    /// assert_eq!(dex_class.simple_name(), "MyClass");
    /// assert_eq!(dex_class.is_array(), false);
    /// assert_eq!(dex_class.descriptor(), "Lcom/example/MyClass;");
    /// ```
    pub fn deserialize<T>(descriptor: T) -> Option<Self>
    where
        T: AsRef<str>,
    {
        let desc = descriptor.as_ref();
        DexSignature::get_type_name(desc).map(|name| Self {
            descriptor: desc.to_string(),
            type_name: name,
        })
    }

    /// Get the original class descriptor. e.g. "Lcom/example/MyClass;"
    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    /// Get the full type name. e.g. "com.example.MyClass"
    pub fn type_name(&self) -> String {
        self.type_name.clone()
    }

    /// Get the class name. e.g. "com.example.MyClass"
    pub fn class_name(&self) -> String {
        self.type_name.clone()
    }

    /// Get the simple class name. e.g. "MyClass"
    pub fn simple_name(&self) -> String {
        self.type_name
            .rsplit('.')
            .next()
            .map(|s| s.to_string())
            .unwrap_or_default()
    }

    /// Check if the class is an array type. e.g. "com.example.MyClass[]" -> true
    pub fn is_array(&self) -> bool {
        self.type_name.ends_with("[]")
    }
}

impl PartialEq for DexClass {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}
