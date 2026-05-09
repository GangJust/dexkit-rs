static PRIMITIVE_TYPES_SIGNATURE: &[(&str, &str)] = &[
    ("void", "V"),
    ("boolean", "Z"),
    ("byte", "B"),
    ("short", "S"),
    ("char", "C"),
    ("int", "I"),
    ("long", "J"),
    ("float", "F"),
    ("double", "D"),
];

static PRIMITIVE_TYPES_NAMES: &[(&str, &str)] = &[
    ("V", "void"),
    ("Z", "boolean"),
    ("B", "byte"),
    ("S", "short"),
    ("C", "char"),
    ("I", "int"),
    ("J", "long"),
    ("F", "float"),
    ("D", "double"),
];

pub struct DexSignature;

impl DexSignature {
    /// Get the primitive type signature from the type name. e.g. "int" -> "I"
    /// # Arguments
    /// * `signature` - The type name to get the signature for. e.g. "int"
    /// # Returns
    /// * `Option<String>` - The primitive type signature if found, otherwise None
    pub fn primitive_type_name<T>(type_name: T) -> Option<String>
    where
        T: AsRef<str>,
    {
        let name = type_name.as_ref();

        PRIMITIVE_TYPES_SIGNATURE
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, sig)| sig.to_string())
    }

    /// Get the primitive type name from the type signature. e.g. "I" -> "int"
    /// # Arguments
    /// * `signature` - The type signature to get the name for. e.g. "I"
    /// # Returns
    /// * `Option<String>` - The primitive type name if found, otherwise None
    pub fn primitive_type_signature<T>(signature: T) -> Option<String>
    where
        T: AsRef<str>,
    {
        let sig = signature.as_ref();
        if sig.len() != 1 {
            return None;
        }

        PRIMITIVE_TYPES_NAMES
            .iter()
            .find(|(s, _)| *s == sig)
            .map(|(_, name)| name.to_string())
    }

    /// Get the type name from the type signature. e.g. "Ljava/lang/String;" -> "java.lang.String"
    /// # Arguments
    /// * `type_signature` - The type signature to get the name for. e.g. "Ljava/lang/String;"
    /// # Returns
    /// * `Option<String>` - The type name if found, otherwise None
    pub fn get_type_name<T>(type_signature: T) -> Option<String>
    where
        T: AsRef<str>,
    {
        let sig = type_signature.as_ref();
        if sig.is_empty() {
            return None;
        }

        // array type
        if sig.starts_with('[') {
            let mut dim = 0;
            for c in sig.chars() {
                if c == '[' {
                    dim += 1;
                } else {
                    break;
                }
            }
            let base_type = &sig[dim..];
            let base_type_name = Self::get_type_name(base_type)?;
            return Some(format!("{}{}", base_type_name, "[]".repeat(dim)));
        }

        // primitive type
        if let Some(prim_name) = Self::primitive_type_signature(sig) {
            return Some(prim_name);
        }

        // object type
        if sig.starts_with('L') && sig.ends_with(';') {
            let internal_name = &sig[1..sig.len() - 1];
            return Some(internal_name.replace('/', "."));
        }

        None
    }

    /// Get the parameter type names from the method signature. e.g. "(ILjava/lang/String;)V" -> ["int", "java.lang.String"]
    /// # Arguments
    /// * `method_signature` - The method signature to get the parameter type names for. e.g. "(ILjava/lang/String;)V"
    /// # Returns
    /// * `Option<Vec<String>>` - The parameter type names if found, otherwise None
    pub fn get_parameter_types<T>(method_signature: T) -> Option<Vec<String>>
    where
        T: AsRef<str>,
    {
        let sig = method_signature.as_ref();
        if !sig.starts_with('(') {
            return None;
        }
        let end_params_index = sig.find(')')?;
        let params_sig = &sig[1..end_params_index];
        let mut params = Vec::new();
        let mut i = 0;
        while i < params_sig.len() {
            let c = params_sig.chars().nth(i)?;
            if c == 'L' {
                // object type
                let semicolon_index = params_sig[i..].find(';')?;
                let type_sig = &params_sig[i..i + semicolon_index + 1];
                let type_name = Self::get_type_name(type_sig)?;
                params.push(type_name);
                i += semicolon_index + 1;
            } else if c == '[' {
                // array type
                let mut dim = 0;
                while i < params_sig.len() && params_sig.chars().nth(i)? == '[' {
                    dim += 1;
                    i += 1;
                }
                if i >= params_sig.len() {
                    return None;
                }
                let base_c = params_sig.chars().nth(i)?;
                if base_c == 'L' {
                    let semicolon_index = params_sig[i..].find(';')?;
                    let type_sig = &params_sig[i - dim..i + semicolon_index + 1];
                    let type_name = Self::get_type_name(type_sig)?;
                    params.push(type_name);
                    i += semicolon_index + 1;
                } else {
                    let type_sig = &params_sig[i - dim..=i];
                    let type_name = Self::get_type_name(type_sig)?;
                    params.push(type_name);
                    i += 1;
                }
            } else {
                // primitive type
                let type_sig = &params_sig[i..=i];
                let type_name = Self::get_type_name(type_sig)?;
                params.push(type_name);
                i += 1;
            }
        }
        Some(params)
    }

    /// Get the type signature from the type name. e.g. "java.lang.String" -> "Ljava/lang/String;"
    /// # Arguments
    /// * `type_name` - The type name to get the signature for. e.g. "java.lang.String"
    /// # Returns
    /// * `Option<String>` - The type signature if found, otherwise None
    pub fn get_type_signature<T>(type_name: T) -> Option<String>
    where
        T: AsRef<str>,
    {
        let name = type_name.as_ref();
        if name.is_empty() {
            return None;
        }

        // array type
        if name.ends_with("[]") {
            let mut dim = 0;
            let mut base_name = name;
            while base_name.ends_with("[]") {
                dim += 1;
                base_name = &base_name[..base_name.len() - 2];
            }
            let base_type_sig = Self::get_type_signature(base_name)?;
            return Some(format!("{}{}", "[".repeat(dim), base_type_sig));
        }

        // primitive type
        if let Some(prim_sig) = Self::primitive_type_name(name) {
            return Some(prim_sig);
        }

        // object type
        let internal_name = name.replace('.', "/");
        Some(format!("L{};", internal_name))
    }
}
