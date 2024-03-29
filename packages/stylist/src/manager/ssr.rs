use core::fmt;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use super::*;

/// The reader to read styles from a [`StyleManager`].
///
/// # Notes
///
/// You can only read styles from a style manager once. This is expected. You should never share
/// styles between renders.
#[derive(Debug)]
pub struct StaticReader {
    inner: Arc<Mutex<StyleData>>,
}

impl StaticReader {
    /// Reads [`StyleData`] from a [`StyleManager`].
    pub fn read_style_data(self) -> StyleData {
        self.inner
            .lock()
            .map(|m| m.clone())
            .unwrap_or_else(|_| StyleData::new())
    }
}

/// The writer to be passed to [`StyleManager`] to write styles.
#[derive(Debug)]
pub struct StaticWriter {
    inner: Arc<Mutex<StyleData>>,
}

/// Creates a [StaticWriter] - [StaticReader] pair.
pub fn render_static() -> (StaticWriter, StaticReader) {
    let inner = Arc::new(Mutex::new(StyleData::new()));

    (
        StaticWriter {
            inner: inner.clone(),
        },
        StaticReader { inner },
    )
}

impl StyleData {
    pub fn write_static_markup<W>(&self, w: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        for StyleDataContent { id, style_str, .. } in self.0.iter() {
            // We cannot guarantee a valid class name if the user choose to use a custom prefix.
            // If the default prefix is used, StyleId is guaranteed to be valid without
            // escaping.
            write!(w, r#"<style data-style="{}">"#, id)?;
            write!(w, "{}", html_escape::encode_style(&style_str))?;
            write!(w, "</style>")?;
        }

        Ok(())
    }
}

impl StyleManagerBuilder {
    /// Set the [StaticWriter] for current manager.
    ///
    /// # Note
    ///
    /// This also sets the StyleManager into the "static" mode. which it will stop rendering
    /// styles into any html element.
    pub fn writer(mut self, w: StaticWriter) -> Self {
        self.style_data = Some(w.inner);
        self
    }
}

impl StyleData {
    pub(crate) fn new() -> StyleData {
        StyleData(Arc::default())
    }
}

impl StyleManager {
    pub(crate) fn style_data(&self) -> Option<impl DerefMut<Target = StyleData> + '_> {
        self.inner
            .style_data
            .as_ref()
            .map(|m| m.lock().expect("failed to lock style data"))
    }
}

impl StyleData {
    pub(super) fn as_vec_mut(&mut self) -> &mut Vec<StyleDataContent> {
        Arc::make_mut(&mut self.0)
    }
}
