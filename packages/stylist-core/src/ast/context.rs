use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, Ordering};

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

    pub fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Relaxed)
    }

    // We close until we can find a parent that has nothing differs from current path.
    pub fn close_until_common_parent(&self, w: &mut String) {
        while let Some(m) = self.open_parent() {
            if self.differ_conditions().is_empty() {
                break;
            }
            m.finish(w);
        }
    }

    pub fn open_parent(&self) -> Option<&StyleContext<'a>> {
        match self.parent_ctx {
            Some(m) => {
                if m.is_open() {
                    Some(m)
                } else {
                    m.open_parent()
                }
            }
            None => None,
        }
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

    fn write_finish_impl(&self, w: &mut String, no: usize) {
        for i in (0..no).rev() {
            self.write_min_padding(w);
            self.write_padding_impl(w, i);
            w.push_str("}\n");
        }
    }

    fn write_start_impl(&self, w: &mut String, conds: Vec<&str>) {
        for (index, cond) in conds.iter().enumerate() {
            self.write_min_padding(w);
            self.write_padding_impl(w, index);
            w.push_str(cond);
            w.push_str(" {\n");
        }
    }

    pub fn finish(&self, w: &mut String) {
        if self.is_open() {
            self.write_finish_impl(w, self.unique_conditions().len());
        }
        self.is_open.store(false, Ordering::Relaxed);
    }

    pub fn start(&self, w: &mut String) {
        if !self.is_open() {
            self.close_until_common_parent(w);
            self.write_start_impl(w, self.unique_conditions());
        }
        self.is_open.store(true, Ordering::Relaxed);
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
