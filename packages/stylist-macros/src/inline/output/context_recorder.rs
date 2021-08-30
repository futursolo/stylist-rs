//! This module implements a type abstractly tracking in what kind of expression context
//! an item appears. This information is leverage to provide improved performance and
//! static caching of parts of the generated output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowedUsage {
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
    // TODO: we can probably avoid a few allocations if we track which parts
    // of the ast can be constructed statically (with const methods), which is
    // even stronger than constructing it in the global context in a Lazy.
    // Should you decide to implement this, keep in mind to change Self::MAX
    // and adjust the generation of cow-vec tokens. Also check the usages of
    // MaybeStatic::statick if they can be upgraded to Const.
    // Const,
}

pub struct ContextRecorder {
    usage: AllowedUsage,
}

impl Default for ContextRecorder {
    fn default() -> Self {
        Self {
            usage: AllowedUsage::Static,
        }
    }
}

impl ContextRecorder {
    // Record the usage of a dynamic expression
    pub fn uses_dynamic_argument(&mut self) {
        self.usage = self.usage.min(AllowedUsage::Dynamic)
    }
    pub fn merge_with(&mut self, other: &ContextRecorder) {
        self.usage = self.usage.min(other.usage)
    }
    pub fn usage(&self) -> AllowedUsage {
        self.usage
    }
}
