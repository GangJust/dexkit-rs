use crate::uitls::DexSignature;

pub struct DexMethod {
    pub descriptor: String,
    pub class_name: String,
    pub method_name: String,
    pub return_type: String,
    pub parameter_types: Vec<String>,
}

impl DexMethod {
    /// Create a new DexMethod from a method descriptor. e.g. "Lcom/example/MyClass;->myMethod(I)V"
    /// # Arguments
    /// * `descriptor` - The method descriptor to create the DexMethod from. e.g. "Lcom/example/MyClass;->myMethod(I)V"
    /// # Returns
    /// * `Option<DexMethod>` - The DexMethod if the descriptor is valid, otherwise None
    /// # Examples
    /// ```
    /// let dex_method = DexMethod::deserialize("Lcom/example/MyClass;->myMethod(I)V").unwrap();
    /// assert_eq!(dex_method.class_name(), "com.example.MyClass");
    /// assert_eq!(dex_method.method_name(), "myMethod");
    /// assert_eq!(dex_method.return_type(), "void");
    /// assert_eq!(dex_method.parameter_types(), vec!["int"]);
    /// assert_eq!(dex_method.descriptor(), "Lcom/example/MyClass;->myMethod(I)V");
    /// ```
    pub fn deserialize<T>(descriptor: T) -> Option<Self>
    where
        T: AsRef<str>,
    {
        let desc = descriptor.as_ref();
        let idx1 = desc.find("->")?;
        let idx2 = desc.find('(')?;
        let idx3 = desc.find(')')?;
        if idx1 < 1 || idx2 < idx1 + 3 || idx3 < idx2 + 1 {
            return None;
        }
        let class_name = DexSignature::get_type_name(&desc[0..idx1])?;
        let method_name = desc[idx1 + 2..idx2].to_string();
        let return_type = DexSignature::get_type_name(&desc[idx3 + 1..])?;
        let parameter_types = DexSignature::get_parameter_types(&desc[idx2..=idx3])?;
        Some(Self {
            descriptor: desc.to_string(),
            class_name,
            method_name,
            return_type,
            parameter_types,
        })
    }

    /// Get the class name. e.g. "com.example.MyClass"
    pub fn class_name(&self) -> String {
        self.class_name.clone()
    }

    /// Get the method name. e.g. "myMethod"
    pub fn method_name(&self) -> String {
        self.method_name.clone()
    }

    /// Get the return type. e.g. "void"
    pub fn return_type(&self) -> String {
        self.return_type.clone()
    }

    /// Get the parameter types. e.g. vec!["int", "java.lang.String"]
    pub fn parameter_types(&self) -> Vec<String> {
        self.parameter_types.clone()
    }

    /// Get the original method descriptor. e.g. "Lcom/example/MyClass;->myMethod(I)V"
    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }
}

impl PartialEq for DexMethod {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}
