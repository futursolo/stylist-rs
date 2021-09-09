// #[derive(Debug)]
// pub enum StyleKind {
//     Scoped,
//     Global,
//     Keyframes,
// }

#[derive(Debug)]
pub struct StyleContext<'a> {
    // pub kind: StyleKind,
    pub class_name: Option<&'a str>,
}
