//! This module implements a type abstractly tracking in what kind of expression context
//! an item appears. This information is leverage to provide improved performance and
//! static caching of parts of the generated output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum AllowedUsage {
    // ```
    // let width = 500;
    // style! { width: ${width}; }
    // //               ^^^^^^ dynamic expression, can't wrap style in Lazy
    // ```
    Dynamic,
    // ```
    // style! { width: 500px; }
    // //       ------------- everything is static, do wrap style in Lazy
    // ```
    Static,
    // We can avoid a few allocations if we track which parts
    // of the ast can be constructed statically (with const methods), which is
    // even stronger than constructing it in the global context in a Lazy.
    Const,
}

#[derive(Debug, Clone)]
pub struct ContextRecorder {
    usage: AllowedUsage,
}

impl Default for ContextRecorder {
    fn default() -> Self {
        Self {
            usage: AllowedUsage::Const,
        }
    }
}

impl ContextRecorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn uses_nested(&mut self, other: &Self) {
        self.usage = self.usage.min(other.usage);
    }

    // Record the usage of a dynamic expression
    pub fn uses_dynamic_argument(&mut self) {
        self.usage = self.usage.min(AllowedUsage::Dynamic);
    }
    // Record the usage of an expression that is not allowed in const context
    pub fn uses_static(&mut self) {
        self.usage = self.usage.min(AllowedUsage::Static);
    }

    pub fn is_static(&self) -> bool {
        self.usage >= AllowedUsage::Static
    }

    pub fn is_const(&self) -> bool {
        self.usage >= AllowedUsage::Const
    }
}
