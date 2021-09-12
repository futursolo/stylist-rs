use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, Ordering};

/// A context to faciliate [`ToStyleStr`](super::ToStyleStr).
#[derive(Debug)]
pub struct StyleContext<'a> {
    pub class_name: Option<&'a str>,
    parent_ctx: Option<&'a StyleContext<'a>>,

    rules: Vec<Cow<'a, str>>,
    selectors: Vec<Cow<'a, str>>,

    is_open: AtomicBool,
}

impl<'a> StyleContext<'a> {
    pub fn new(class_name: Option<&'a str>) -> Self {
        Self {
            parent_ctx: None,
            class_name,
            rules: Vec::new(),
            selectors: Vec::new(),

            is_open: AtomicBool::new(false),
        }
    }

    fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Relaxed)
    }

    fn set_open(&self, value: bool) {
        self.is_open.store(value, Ordering::Relaxed);
    }

    // We close until we can find a parent that has nothing differs from current path.
    fn close_until_common_parent(&self, w: &mut String) {
        while let Some(m) = self.open_parent() {
            if self.differ_conditions().is_empty() {
                break;
            }
            m.finish(w);
        }
    }

    fn open_parent(&self) -> Option<&StyleContext<'a>> {
        self.parent_ctx.and_then(|m| {
            if m.is_open() {
                Some(m)
            } else {
                m.open_parent()
            }
        })
    }

    fn conditions(&self) -> Vec<&str> {
        self.rules
            .iter()
            .chain(self.selectors.iter())
            .map(|m| m.as_ref())
            .collect()
    }

    fn common_conditions(&self) -> Vec<&str> {
        match self.open_parent() {
            Some(m) => self
                .conditions()
                .into_iter()
                .zip(m.conditions())
                .filter_map(|(m1, m2)| if m1 == m2 { Some(m1) } else { None })
                .collect(),
            None => Vec::new(),
        }
    }

    /// Calculate the layers that current context differs from the parent context
    fn unique_conditions(&self) -> Vec<&str> {
        self.conditions()
            .into_iter()
            .skip(self.common_conditions().len())
            .collect()
    }

    /// Calculate the layers that parent context differs from current context
    fn differ_conditions(&self) -> Vec<&str> {
        match self.open_parent() {
            Some(m) => m
                .conditions()
                .into_iter()
                .skip(self.common_conditions().len())
                .collect(),
            None => Vec::new(),
        }
    }

    fn write_padding_impl(&self, w: &mut String, no: usize) {
        for _ in 0..no {
            w.push_str("    ");
        }
    }

    fn write_min_padding(&self, w: &mut String) {
        self.write_padding_impl(w, self.common_conditions().len())
    }

    pub fn finish(&self, w: &mut String) {
        if self.is_open() {
            for i in (0..self.unique_conditions().len()).rev() {
                self.write_min_padding(w);
                self.write_padding_impl(w, i);
                w.push_str("}\n");
            }
        }
        self.set_open(false);
    }

    pub fn start(&self, w: &mut String) {
        if !self.is_open() {
            self.close_until_common_parent(w);

            for (index, cond) in self.unique_conditions().iter().enumerate() {
                self.write_min_padding(w);
                self.write_padding_impl(w, index);
                w.push_str(cond);
                w.push_str(" {\n");
            }
        }
        self.set_open(true);
    }

    pub fn write_padding(&self, w: &mut String) {
        self.write_padding_impl(w, self.conditions().len());
    }

    pub fn with_block_condition<S>(&'a self, cond: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let mut selectors = self.selectors.clone();

        if let Some(m) = cond {
            selectors.push(m.into());
        } else if self.selectors.is_empty() {
            selectors.push(
                self.class_name
                    .map(|m| format!(".{}", m).into())
                    .unwrap_or_else(|| "html".into()),
            )
        }

        Self {
            parent_ctx: Some(self),
            class_name: self.class_name,
            rules: self.rules.clone(),
            selectors,

            is_open: AtomicBool::new(false),
        }
    }

    pub fn with_rule_condition<S: Into<Cow<'a, str>>>(&'a self, cond: S) -> Self {
        let mut rules = self.rules.clone();
        rules.push(cond.into());

        Self {
            parent_ctx: Some(self),
            class_name: self.class_name,
            rules,
            selectors: self.selectors.clone(),

            is_open: AtomicBool::new(false),
        }
    }
}
