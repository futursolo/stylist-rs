//! This module implements a type abstractly tracking in what kind of expression context
//! an item appears. This information is leverage to provide improved performance and
//! static caching of parts of the generated output.

// The kind of an expression decribes in what context it can be used. It is harmless to
// underapproximate the usage and e.g. classify expressions as Dynamic even if they are
// actually Static.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExpressionKind {
    // A dynamic expression refers to local variables and thus can not be used in global
    // or compile-time `const` contexts.
    // ```
    // let width = 500;
    // style! { width: ${width}; }
    // //                ^^^^^ dynamic expression, can't wrap style in Lazy
    // ```
    Dynamic,
    // A static expression refers to global variables, which might still not be evaluatable
    // in compile-time `const` contexts.
    //
    // E.g. assuming a library method `rgb` producing a string.
    // ```
    // style! { color: ${rgb(100, 100, 100)}; }
    // //                ------------------ expression is static, can wrap style in Lazy
    // ```
    Static,
    // An expression that can be evaluated in `const` context.
    // We can avoid a few allocations if we track which parts of the ast can be constructed
    // statically (with const methods), which is even stronger than constructing it in
    // the global context in a Lazy.
    // ```
    // style! { color: black; }
    // //       ------------- everything can be constructed in a const context
    // ```
    Const,
}

impl ExpressionKind {
    // Compute the jointly allowed usage of two expression that are used together.
    pub fn join(&self, rhs: Self) -> Self {
        use ExpressionKind::*;
        match (self, rhs) {
            (Dynamic, _) | (_, Dynamic) => Dynamic,
            (Static, _) | (_, Static) => Static,
            _ => Const,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContextRecorder {
    usage: ExpressionKind,
}

impl Default for ContextRecorder {
    fn default() -> Self {
        Self {
            usage: ExpressionKind::Const,
        }
    }
}

impl ContextRecorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn uses_nested(&mut self, other: &Self) {
        self.usage = self.usage.join(other.usage);
    }

    // Record the usage of a dynamic expression
    pub fn uses_dynamic_argument(&mut self) {
        self.usage = self.usage.join(ExpressionKind::Dynamic);
    }
    // Record the usage of an expression that is not allowed in const context
    pub fn uses_static(&mut self) {
        self.usage = self.usage.join(ExpressionKind::Static);
    }

    pub fn is_static(&self) -> bool {
        matches!(self.usage, ExpressionKind::Static | ExpressionKind::Const)
    }

    pub fn is_const(&self) -> bool {
        matches!(self.usage, ExpressionKind::Const)
    }
}
