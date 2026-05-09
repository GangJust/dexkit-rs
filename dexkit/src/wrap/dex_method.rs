use crate::utils::DexSignature;
use std::cell::OnceCell;

#[derive(Debug, Clone)]
pub struct DexMethod {
    descriptor: String,
    class_name: String,
    method_name: String,
    return_type: String,
    parameter_type_names: Vec<String>,
    // Lazy loaded fields
    method_signature: OnceCell<String>,
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
    /// assert_eq!(dex_method.return_type_name(), "void");
    /// assert_eq!(dex_method.param_type_names(), vec!["int"]);
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
            parameter_type_names: parameter_types,
            method_signature: OnceCell::new(),
        })
    }

    /// Get the original method descriptor. e.g. "Lcom/example/MyClass;->myMethod(I)V"
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    /// method class name, e.g. "com.example.MyClass"
    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    /// method class name, e.g. "com.example.MyClass"
    pub fn declared_class_name(&self) -> &str {
        self.class_name()
    }

    /// method name, e.g. "myMethod"
    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    /// method name, e.g. "myMethod"
    pub fn name(&self) -> &str {
        self.method_name()
    }

    /// parameter type names, e.g. vec!["int", "java.lang.String"]
    pub fn param_type_names(&self) -> &[String] {
        &self.parameter_type_names
    }

    /// return type name, e.g. "void"
    pub fn return_type_name(&self) -> &str {
        &self.return_type
    }

    /// is constructor method
    pub fn is_constructor(&self) -> bool {
        self.method_name == "<init>"
    }

    /// is static initializer method
    pub fn is_static_initializer(&self) -> bool {
        self.method_name == "<clinit>"
    }

    /// is method
    pub fn is_method(&self) -> bool {
        !self.is_static_initializer() && !self.is_constructor()
    }

    /// Get the method signature. e.g. "myMethod(ILjava/lang/String;)V"
    pub fn method_signature(&self) -> &str {
        self.method_signature
            .get_or_init(|| {
                let params = self
                    .parameter_type_names
                    .iter()
                    .map(|m| DexSignature::get_type_signature(m).unwrap_or_default())
                    .collect::<Vec<String>>()
                    .join("");
                let return_type =
                    DexSignature::get_type_signature(&self.return_type).unwrap_or_default();
                format!("{}({}){}", self.method_name, params, return_type)
            })
            .as_str()
    }
}

impl PartialEq for DexMethod {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}
