// #[derive(Debug)]
// pub enum StyleKind {
//     Scoped,
//     Global,
//     Keyframes,
// }
//

#[derive(Debug, Clone)]
pub struct StyleContext<'a> {
    // pub kind: StyleKind,
    pub parent_conditions: Vec<&'a str>,
}

impl<'a> StyleContext<'a> {
    pub fn root_class_name(&self) -> Option<&'a str> {
        self.parent_conditions
            .first()
            .and_then(|m| if m.starts_with('@') { None } else { Some(*m) })
    }

    pub fn with_condition(mut self, condition: &'a str) -> Self {
        self.parent_conditions.push(condition);

        self
    }
}
