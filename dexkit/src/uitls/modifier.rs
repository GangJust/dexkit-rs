use std::fmt::Debug;

pub struct Modifier(u32);

impl Modifier {
    pub const PUBLIC: Self = Self(1);
    pub const PRIVATE: Self = Self(2);
    pub const PROTECTED: Self = Self(4);
    pub const STATIC: Self = Self(8);
    pub const FINAL: Self = Self(16);
    pub const SYNCHRONIZED: Self = Self(32);
    pub const VOLATILE: Self = Self(64);
    pub const TRANSIENT: Self = Self(128);
    pub const NATIVE: Self = Self(256);
    pub const INTERFACE: Self = Self(512);
    pub const ABSTRACT: Self = Self(1024);
    pub const STRICT: Self = Self(2048);
    pub const BRIDGE: Self = Self(64);
    pub const VARARGS: Self = Self(128);
    pub const SYNTHETIC: Self = Self(4096);
    pub const ANNOTATION: Self = Self(8192);
    pub const ENUM: Self = Self(16384);
    pub const MANDATED: Self = Self(32768);
    pub const ACCESS_MODIFIERS: Self = Self(7);

    pub fn bits(&self) -> u32 {
        self.0
    }

    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub fn contains(&self, other: Modifier) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for Modifier {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Modifier {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::Not for Modifier {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::BitXor for Modifier {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Deref for Modifier {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Modifier> for u32 {
    fn from(modifier: Modifier) -> Self {
        modifier.0
    }
}

impl Debug for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut mods = vec![];
        if self.contains(Modifier::PUBLIC) {
            mods.push("public");
        }
        if self.contains(Modifier::PRIVATE) {
            mods.push("private");
        }
        if self.contains(Modifier::PROTECTED) {
            mods.push("protected");
        }
        if self.contains(Modifier::STATIC) {
            mods.push("static");
        }
        if self.contains(Modifier::FINAL) {
            mods.push("final");
        }
        if self.contains(Modifier::SYNCHRONIZED) {
            mods.push("synchronized");
        }
        if self.contains(Modifier::VOLATILE) {
            mods.push("volatile");
        }
        if self.contains(Modifier::TRANSIENT) {
            mods.push("transient");
        }
        if self.contains(Modifier::NATIVE) {
            mods.push("native");
        }
        if self.contains(Modifier::INTERFACE) {
            mods.push("interface");
        }
        if self.contains(Modifier::ABSTRACT) {
            mods.push("abstract");
        }
        if self.contains(Modifier::STRICT) {
            mods.push("strict");
        }
        if self.contains(Modifier::BRIDGE) {
            mods.push("bridge");
        }
        if self.contains(Modifier::VARARGS) {
            mods.push("varargs");
        }
        if self.contains(Modifier::SYNTHETIC) {
            mods.push("synthetic");
        }
        if self.contains(Modifier::ANNOTATION) {
            mods.push("annotation");
        }
        if self.contains(Modifier::ENUM) {
            mods.push("enum");
        }
        if self.contains(Modifier::MANDATED) {
            mods.push("mandated");
        }
        write!(f, "Modifier({})", mods.join(" "))
    }
}
